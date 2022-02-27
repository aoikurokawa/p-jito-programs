//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Provides price and liquidity data useful for a variety of system designs
library Oraclee {
    struct Observation {
        uint32 blockTimestamp;
        int56 tickCumulative;
        uint160 secondsPerLiquidityCumulativeX128;
        bool initialized;
    }

    // Transforms a previous observation into a new observation, given the passage of time and the current tick and liquidity values
    function transform(
        Observation memory last,
        uint32 blockTimestamp,
        int24 tick,
        uint128 liquidity
    ) private pure returns (Observation memory) {
        uint32 delta = blockTimestamp - last.blockTimestamp;
        return
            Observation({
                blockTimestamp: blockTimestamp,
                tickCumulative: last.tickCumulative +
                    int56(tick) *
                    int32(delta),
                secondsPerLiquidityCumulativeX128: last
                    .secondsPerLiquidityCumulativeX128 +
                    ((uint160(delta) << 128) / (liquidity > 0 ? liquidity : 1)),
                initialized: true
            });
    }

    // Initialize the oracle array by writing the first slot. Called once for the lifecycle of the observations array
    function initialize(Observation[65535] storage self, uint32 time)
        internal
        returns (uint16 cardinality, uint16 cardinalityNext)
    {
        self[0] = Observation({
            blockTimestamp: time,
            tickCumulative: 0,
            secondsPerLiquidityCumulativeX128: 0,
            initialized: true
        });
        return (1, 1);
    }

    // Writes an oracle observation to the array
    function write(
        Observation[65535] storage self,
        uint16 index,
        uint32 blockTimestamp,
        int24 tick,
        uint128 liquidity,
        uint16 cardinality,
        uint16 cardinalityNext
    ) internal returns (uint16 indexUpdated, uint16 cardinalityUpdated) {
        Observation memory last = self[index];

        if (last.blockTimestamp == blockTimestamp) return (index, cardinality);

        if (cardinalityNext > cardinality && index == (cardinality - 1)) {
            cardinalityUpdated = cardinalityNext;
        } else {
            cardinalityUpdated = cardinality;
        }

        indexUpdated = (index + 1) % cardinalityUpdated;
        self[indexUpdated] = transform(last, blockTimestamp, tick, liquidity);
    }

    // Prepares the oracle array to store up to 'next' observations
    function grow(
        Observation[65535] storage self,
        uint16 current,
        uint16 next
    ) internal returns (uint16) {
        require(current > 0, "I");

        if (next <= current) return current;

        for (uint16 i = current; i < next; i++) self[i].blockTimestamp = 1;
        return next;
    }

    // comparator for 32-bit timestamps
    function lte(
        uint32 time,
        uint32 a,
        uint32 b
    ) private pure returns (bool) {
        if (a <= time && b <= time) return a <= b;

        uint256 aAdjusted = a > time ? a : a + 2**32;
        uint256 bAdjusted = b > time ? b : b + 2**32;

        return aAdjusted <= bAdjusted;
    }

    // Fetches the observations beforeOrAt and atOrAfter a target, where [beforeOrAt, atOrAfter] is satisfied.
    // The result may be the same observation, or adjacent observations
    function binarySearch(
        Observation[65535] storage self,
        uint32 time,
        uint32 target,
        uint16 index,
        uint16 cardinality
    )
        private
        view
        returns (Observation memory beforeOrAt, Observation memory atOrAfter)
    {
        uint256 l = (index + 1) % cardinality; // oldest observation
        uint256 r = l + cardinality - 1; // newest observation
        uint256 i;

        while (true) {
            i = (l + r) / 2;

            beforeOrAt = self[i % cardinality];

            if (!beforeOrAt.initialized) {
                l = i + 1;
                continue;
            }

            atOrAfter = self[(i + 1) % cardinality];

            bool targetAtOrAfter = lte(time, beforeOrAt.blockTimestamp, target);

            if (targetAtOrAfter && lte(time, target, atOrAfter.blockTimestamp))
                break;

            if (!targetAtOrAfter) r = i - 1;
            else l = i + 1;
        }
    }

    // Fetches the observations beforeOrAt and atOrAfter a given target, i.e. where [beforeOrAt, atOrAfter] is satisfied
    function getSurroundingObservations(
        Observation[65535] storage self,
        uint32 time,
        uint32 target,
        int24 tick,
        uint16 index,
        uint128 liquidity,
        uint16 cardinality
    )
        private
        view
        returns (Observation memory beforeOrAt, Observation memory atOrAfter)
    {
        beforeOrAt = self[index];

        if (lte(time, beforeOrAt.blockTimestamp, target)) {
            if (beforeOrAt.blockTimestamp == target) {
                return (beforeOrAt, atOrAfter);
            } else {
                return (
                    beforeOrAt,
                    transform(beforeOrAt, target, tick, liquidity)
                );
            }
        }

        beforeOrAt = self[(index + 1) % cardinality];
        if (!beforeOrAt.initialized) beforeOrAt = self[0];

        require(lte(time, beforeOrAt.blockTimestamp, target), "OLD");

        return binarySearch(self, time, target, index, cardinality);
    }
}
