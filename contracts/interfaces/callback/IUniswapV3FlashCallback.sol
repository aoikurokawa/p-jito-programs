// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Callback for IUniswapV3PoolActions#flash
interface IUniswapV3FlashCallback {
    // Called to msg.sender after transfering to the recipient from IUniswapV3Pool
    function uniswapV3FlashCallback(
        uint256 fee0,
        uint256 fee1,
        bytes calldata data
    ) external;
}
