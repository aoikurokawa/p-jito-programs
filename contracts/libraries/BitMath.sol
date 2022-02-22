//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

library BitMath {
    // Returns the index of the most significant bit of the number
    // where the least significant bit is at index 0 and the most signifivcant bit is at index 255.
    function mostSignificantBit(uint256 x) internal pure returns (uint256 r) {
        require(x > 0);
    }
}
