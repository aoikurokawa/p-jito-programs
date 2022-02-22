// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Contains pool methods that can be called by anyone
interface IUniswapV3PoolActions {
    // Sets the initial price for the pool
    function initialize(uint160 sqrtPriceX96) external;

    // Adds liquidity for the given recipient/tickLower/tickUpper position
    function mint(
        address recipient,
        int24 tickLower,
        int24 tockUpper,
        uint128 amount,
        bytes calldata data
    ) external returns (uint256 amount0, uint256 amount1);

    // Collects tokens owed to a position
    function collect(
        address recipient,
        int24 tickLower,
        int24 tickUpper,
        uint128 amount0Requested,
        uint128 amount1Requested
    ) external returns (uint128 amount0, uint128 amount1);

    // Burn liquidity from the sender and account tokens owed for the liquidity to the position
    function burn(
        int24 tickLower,
        int24 tickUpper,
        uint128 amount
    ) external returns (uint256 amount0, uint256 amount1);

    // Swap token0 for token1 or token1 for token0
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external returns (int256 amount0, int256 amount1);

    // Receive token0 and/or token1 and pay it back, plus a fee, in the callback
    function flash(
        address recipient,
        uint256 amount0,
        uint256 amount1,
        bytes calldata data
    ) external;

    // Increase the maximum number of price and liquidity observations that this pool will store
    function increaseObservationCardinalityNext(
        uint16 observationCardinalityNext
    ) external;
}
