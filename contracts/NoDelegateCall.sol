//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Prevents delegatecall to a contract
// Base contract that provides a modifier for preventing delegatecall to metbods in a child contract
abstract contract NoDelegateCall {
    // The origianl address of this contract
    address private immutable original;

    constructor() {
        // Immutable are computed in the init code of the contract, and then inlined into the deployed bytecode.
        // In other words, this variable won't change when it's checked at runtime.
        original = address(this);
    }

    // Private method is used instead of inlining into modifier because modifiers are copied into each method,
    // and the use of immutable means the address bytes are copied in every place the modifier is used.
    function checkNotDelegateCall() private view {
        require(address(this) == original);
    }

    // Prevents delegatecall into the modified method
    modifier noDelegateCall() {
        checkNotDelegateCall();
        _;
    }
}
