//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Contains methods for safely casting between casting between types
library SafeCast {
    // Cast a uint256 to a uint160, revert on overflow
    function toUint160(uint256 y) internal pure returns (uint160 z) {
        require((z = uint160(y)) == y);
    }

    // Cast a int256 to a int128, revert on overflow or underflow
    function toUint128(int256 y) internal pure returns (int128 z) {
        require((z = int128(y)) == y);
    }

    // Cast a uint256 to int256, revert on overflow
    function toInt256(uint256 y) internal pure returns (int256 z) {
        require(y < 2**255);
        z = int256(y);
    }
}
