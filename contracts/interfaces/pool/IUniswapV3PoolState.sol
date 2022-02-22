// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// These methods compose the pool's state, and can change with any frequency including multiple times per transaction
interface IUniswapV3PoolState {
    // The 0th storage slot in the pool stores many values, and is exposed as single method to save gas
    // when accessed externally.
    function slot0()
        external
        view
        returns (
            uint160 sqrtPriceX86,
            int24 tick,
            uint16 observationIndex,
            uint16 observationCardinality,
            uint16 observationCardinalityNext,
            uint8 feeProtocol,
            bool unlocked
        );

    // The fee growth as a Q128.129 fees of token0 collected per unit of liquidity for the entire life of the pool
    function feeGrowthGlobal0X128() external view returns (uint256);

    // The fee growth as a Q128 fees of token1 collected per unit of liquidity for the entire life of the pool
    function feeGrowthGlobal1X128() external view returns (uint256);

    // The amounts of token0 and token1 that are owed to the protocol
    function protocolFees()
        external
        view
        returns (uint128 token0, uint128 token1);

    // The currently in range liquidity available to the pool.
    function liquidity() external view returns (uint128);

    // Look up information about a specific tick in the pool
    function ticks(int24 tick)
        external
        view
        returns (
            uint128 liquidityGross,
            int128 liquidityNet,
            uint256 feeGrowthOutside0X128,
            uint256 feeGrowthOutside1X128,
            int56 tickCumulativeOutside,
            uint160 secondsPerLiquidityOutsideX128,
            uint32 secondsOutside,
            bool initialized
        );

    // Returns 256 packed tick initialized boolean values. See TickBitmap for more information.
    function tickBitmap(int16 wordPosition) external view returns (uint256);

    // Returns the information about a postion by the position's key
    function positions(bytes32 key)
        external
        view
        returns (
            uint128 _liquidity,
            uint256 feeGrowthInside0LastX128,
            uint256 feeGrowthInside1LastX128,
            uint128 tokensOwed0,
            uint128 tokensOwed1
        );

    // Returns data about a specific observation index
    function observations(uint256 index)
        external
        view
        returns (
            uint32 blockTimestamp,
            int56 tickCumulative,
            uint160 secondsPerLiquidityCumulativeX128,
            bool initialized
        );
}
