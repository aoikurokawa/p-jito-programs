// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// These parameters are fixed for a pool forever, i.e., the methods will always return the same values
interface IUniswapV3PoolImmutable {
    // The contract that deployed the pool, which must adhere to the IUniswapV3Factory interface
    function factory() external view returns (address);

    // The first of the tokens of the pool, sorted by address
    function token0() external view returns (address);

    // The second of the two tokens of the pool, sorted by address
    function token1() external view returns (address);

    // The pool's fee in hundredths of a bip
    function fee() external view returns (uint24);

    // The pool tick spacing
    function tickSpcing() external view returns (int24);

    // The maximum amount of postion liquidity that can use any tick in the range
    function maxLiquidityPerTick() external view returns (uint128);
}
