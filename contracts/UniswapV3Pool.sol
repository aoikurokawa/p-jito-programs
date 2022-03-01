//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "./interfaces/IUniswapV3Pool.sol";

import "./NoDelegateCall.sol";

import "./libraries/LowGasSafeMath.sol";
import "./libraries/SafeCast.sol";
import "./libraries/Tick.sol";
import "./libraries/TickBitmap.sol";
import "./libraries/Position.sol";
import "./libraries/Oracle.sol";

import "./libraries/FullMath.sol";
import "./libraries/FixedPoint128.sol";
import "./libraries/TransferHelper.sol";
import "./libraries/TickMath.sol";
import "./libraries/LiquidityMath.sol";
import "./libraries/SqrtPriceMath.sol";
import "./libraries/SwapMath.sol";

import "./interfaces/IUniswapV3PoolDeployer.sol";
import "./interfaces/IUniswapV3Factory.sol";
import "./interfaces/IERC20Minimal.sol";

import "./interfaces/IUniswapV3PoolDeployer.sol";
import "./interfaces/IUniswapV3Factory.sol";
import "./interfaces/callback/IUniswapV3MintCallback.sol";
import "./interfaces/callback/IUniswapV3SwapCallback.sol";
import "./interfaces/callback/IUniswapV3FlashCallback.sol";

contract UniswapV3Pool is IUniswapV3Pool, NoDelegateCall {
    using LowGasSafeMath for uint256;
    using LowGasSafeMath for int256;
    using SafeCast for uint256;
    using SafeCast for int256;
    using Tick for mapping(int24 => Tick.Info);
    using TickBitmap for mapping(int16 => uint256);
    using Position for mapping(bytes32 => Position.Info);
    using Position for Position.Info;
    using Oracle for Oracle.Observation[65535];

    // IUniswapV3PoolImmutable
    address public immutable override factory;

    // IUniswapV3PoolImmutable
    address public immutable override token0;

    // IUniswapV3PoolImmutables
    address public immutable override token1;

    // IUniswapV3PoolImmutables
    uint24 public immutable override fee;

    // IUniswapV3PoolImmutable
    int24 public immutable override tickSpacing;

    // IUniswapV3PoolImmutables
    uint128 public immutable override maxLiquidityPerTick;

    struct Slot0 {
        // the current price
        uint160 sqrtPriceX96;
        // the current tick
        int24 tick;
        // the most-recently updated index of the observations array
        uint16 observationIndex;
        // the current maximum number of observatios that are being stored
        uint16 observationCardinalityNext;
        // the current protocol fee as percentage of the swap fee token on withdrawal
        // represented as an integer denominator
        uint8 feeProtocol;
        // whether the pool is locked
        bool unlocked;
    }

    Slot0 public override slot0;
}
