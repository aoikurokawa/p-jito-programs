//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

library BitMath {
    // Returns the index of the most significant bit of the number
    // where the least significant bit is at index 0 and the most signifivcant bit is at index 255.
    function mostSignificantBit(uint256 x) internal pure returns (uint256 r) {
        require(x > 0);

        // 32
        if (x >= 0x100000000000000000000000000000000) {
            x >>= 128;
            r += 128;
        }

        // 16
        if (x >= 0x10000000000000000) {
            x >>= 64;
            r += 64;
        }

        // 8
        if (x >= 0x100000000) {
            x >>= 32;
            r += 32;
        }

        if (x >= 0x10000) {
            x >>= 16;
            r += 16;
        }

        if (x >= 0x100) {
            x >>= 8;
            r += 8;
        }

        if (x >= 0x10) {
            x >>= 4;
            x += 4;
        }

        if (x >= 0x4) {
            x >>= 2;
            r += 2;
        }

        if (x >= 0x2) r += 1;
    }
}
