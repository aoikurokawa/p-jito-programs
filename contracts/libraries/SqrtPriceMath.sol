//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "./LowGasSafeMath.sol";
import "./SafeCast.sol";

import "./FullMath.sol";
import "./UnsafeMath.sol";
import "./FixedPoint96.sol";

library SqrtPriceMath {
    using LowGasSafeMath for uint256;
    using SafeCast for uint256;

    // Gets the next sqrt price given a delta of token0
    function getNextSqrtPriceFromAmount0RoundingUp(
        uint160 sqrtPX96,
        uint128 liquidity,
        uint256 amount,
        bool add
    ) internal pure returns (uint160) {
        // we short circuit amount == 0 because the result is therwise not guaranteed to equal the input price
        if (amount == 0) return sqrtPX96;
        uint256 numerator1 = uint256(liquidity) << FixedPoint96.RESOLUTION;

        if (add) {
            uint256 product;
            if ((product = amount * sqrtPX96) / amount == sqrtPX96) {
                uint256 denominator = numerator1 + product;
                if (denominator >= numerator1) {
                    return
                    // always fits in 160 bits
                        uint160(
                            FullMath.mulDivRoundingUp(
                                numerator1,
                                sqrtPX96,
                                denominator
                            )
                        );
                }
            }

            return
                uint160(
                    UnsafeMath.divRoundingUp(
                        numerator1,
                        (numerator1 / sqrtPX96).add(amount)
                    )
                );
        } else {
            uint256 product;
            // if the product overflows, we know the denominator underflows
            // in addition, we must check that the denominator does not underflow
            require(
                (product = amount * sqrtPX96) / amount == sqrtPX96 &&
                    numerator1 > product
            );
            uint256 denominator = numerator1 - product;
            return
                FullMath
                    .mulDivRoundingUp(numerator1, sqrtPX96, denominator)
                    .toUint160();
        }
    }
}
