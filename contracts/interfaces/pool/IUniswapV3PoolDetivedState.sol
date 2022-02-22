// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Contains view functions to provide information about the pool that id computed rather than stored on the
// blochcain. The functions here may have variable gas costs.
interface IUniswapV3PoolDerivedState {
    // Returns the cumulative tick and liquidity as of each timestamp 'secondsAgo' from the current block timestamp
    function observe(uint32[] calldata secondsAgos)
        external
        view
        returns (
            int56[] memory tickCumulative,
            uint160[] memory secondsPerLiquidityCumulativeX128s
        );

    // Returns a snapshot of the tick cumulative. seconds per liquidity and seconds inside a tick range
    function snapshotCumulativeInside(int24 tickLower, int24 tickUpper)
        external
        view
        returns (
            int56 tickCumulativeInside,
            uint160 ssecondsPerLiquidtityInsideX128,
            uint32 secondsInside
        );
}
