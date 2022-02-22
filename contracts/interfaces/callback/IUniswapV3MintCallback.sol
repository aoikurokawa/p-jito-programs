// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Callback for IUniswapV3PoolAction#mint
interface IUniswapV3MintCallback {
    // Called to 'msg.sender' after minting liquidity to a postion from IUniswapV3Pool
    function uniswapV3MintCallback(
        uint256 amount0Owed,
        uint256 amount1Owed,
        bytes calldata data
    ) external;
}
