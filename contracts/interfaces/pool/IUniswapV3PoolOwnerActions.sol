// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.8.0;

// Permissioned pool actions
interface IUniswapV3PoolOwnerActions {
    // Set the denominator of the protocol's % share of the fees
    function setFeeProtocol(uint8 feeProtocol0, uint8 feeProtocol1) external;

    // Collect the protocol fee accured ti the pool
    function collectProtocol(
        address recipient,
        uint128 amount0Requested,
        uint128 amount1Requested
    ) external returns (uint128 amount0, uint128 amount1);
}
