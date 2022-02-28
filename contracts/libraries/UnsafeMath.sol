//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Math functions that do not check inputs or outputs
library UnsafeMath {
    // Returns ceil(x / y)
    function divRoundingUp(uint256 x, uint256 y)
        internal
        pure
        returns (uint256 z)
    {
        assembly {
            z := add(div(x, y), gt(mod(x, y), 0))
        }
    }
}
