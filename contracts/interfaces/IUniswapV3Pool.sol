// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

import "./pool/IUniswapV3PoolImmutable.sol";
import "./pool/IUniswapV3PoolState.sol";
import "./pool/IUniswapV3PoolDetivedState.sol";
import "./pool/IUniswapV3PoolActions.sol";
import "./pool/IUniswapV3PoolOwnerActions.sol";
import "./pool/IUniswapV3PoolEvents.sol";

// The interface for a Uniswap V3 Pool
// A Uniswap pool facilitates swapping and automated market making between any two assets that strictly conform
interface IUniswapV3Pool is
    IUniswapV3PoolImmutable,
    IUniswapV3PoolState,
    IUniswapV3PoolDerivedState,
    IUniswapV3PoolActions,
    IUniswapV3PoolOwnerActions,
    IUniswapV3PoolEvents
{

}
