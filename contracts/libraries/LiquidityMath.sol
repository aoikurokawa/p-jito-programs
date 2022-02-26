//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

library LiquidityMath {
    // Add a signed liquidity delta to liquidity and revert if overflows or underflows
    function addDelta(uint128 x, uint128 y) internal pure returns (uint128 z) {
        if (y < 0) {
            require((z = x - uint128(-y)) < x, "LS");
        } else {
            require((z = x + uint128(y)) >= x, "LA");
        }
    }
}