// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// An interface for a contract that is capable of deploying Uniswap V3 Pools
// A contract that constructs a pool must impolement this to pass arguments to the pool
interface IUniswapV3PoolDeployer {
    // Get the parameters to be used in constructing the pool, set trasiently duting pool creation.
    function parameters()
        external
        view
        returns (
            address factory,
            address token0,
            address token1,
            uint24 fee,
            int24 tickSpacing
        );
}
