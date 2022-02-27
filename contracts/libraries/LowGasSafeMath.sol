//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Optimized overflow and underflow safe math operations
library LowGasSafeMath {
    // Returns x + y, reverts if sum overflows uint256
    function add(uint256 x, uint256 y) internal pure returns (uint256 z) {
        require((z = x + y) >= x);
    }

    // Returns x - y, reverts if underflows
    function sub(uint256 x, uint256 y) internal pure returns (uint256 z) {
        require((z = x - y) <= x);
    }

    // Returns x * y, reverts if overflows
    function mul(uint256 x, uint256 y) internal pure returns (uint256 z) {
        require(x == 0 || (z = x * y) / x == y);
    }

    // Returns x + y, reverts if overflows of underflows
    function add(int256 x, int256 y) internal pure returns (int256 z) {
        require((z = x + y) >= x == (y >= 0));
    }

    // Returns x - y, reverts if overflows or underflows
    function sub(int256 x, int256 y) internal pure returns (int256 z) {
        require((z = x - y) <= x == (y >= 0));
    }
}
