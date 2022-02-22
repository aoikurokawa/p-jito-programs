// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Contains all events emitted by the pool
interface IUniswapV3PoolEvents {
    // Emitted exactly once by a pool when #initialize is first called on the pool
    event Initialize(uint160 sqrtPriceX96, int24 tick);

    // Emitted when liquidity is minted for a given position
    event Mint(
        address sender,
        address indexed owner,
        int24 indexed tickLower,
        int24 indexed tickUpper,
        uint128 amount,
        uint256 amount0,
        uint256 amount1
    );

    // Emitted when fees are collected by the owner of a postion
    event Collect(
        address indexed owner,
        address recipient,
        int24 indexed tickLower,
        int24 indexed tickUpper,
        uint128 amount0,
        uint128 amount1
    );

    // Emitted when a postion's liquidity is removed
    event Burn(
        address indexed owner,
        int24 indexed tickLower,
        int24 indexed tickUpper,
        uint128 amount,
        uint256 amount0,
        uint256 amount1
    );

    // Emitted by the pool for any swaps between token and token1
    event Swap(
        address indexed sender,
        address indexed recipient,
        int256 amount0,
        int256 amount1,
        uint160 sqrtPriceX96,
        uint128 liquidity,
        int24 tick
    );

    // Emitted by the pool for any flashes of token0/token1
    event Flash(
        address indexed sender,
        address indexed recipient,
        uint256 amount0,
        uint256 amount1,
        uint256 paid0,
        uint256 paid1
    );

    // Emitted by the pool for increases to the number of obsservations that can be stored
    event IncreaseObservationCardinalityNext(
        uint16 observationCardinalityNextOld,
        uint16 observationCardinalityNextNew
    );

    // Emitted when the protocol fee is change by the pool
    event SetFeeProtocol(
        uint8 feeProtocol0Old,
        uint8 feeProtocol1Old,
        uint8 feeProtocol0New,
        uint8 feeProtocol1New
    );

    // Emitted when the collected protocol fees are withdrawn by the factory owner
    event CollectProtocol(
        address indexed sender,
        address indexed recipient,
        uint128 amount0,
        uint128 amount1
    );
}
