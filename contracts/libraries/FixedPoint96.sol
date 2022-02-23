//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// A library for handling binary fixed point nubers,
library FixedPoint96 {
    uint8 internal constant RESOLUTION = 96;
    uint256 internal constant Q96 = 0x1000000000000000000000000;
}
