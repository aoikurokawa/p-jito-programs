// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Callback for IUniswapV3PoolAction#swap
interface IUniswapV3SwapCallback {
    // Called to 'msg.sender' after executing a swap via IUniswapV3Pool#swap.
    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external;
}
