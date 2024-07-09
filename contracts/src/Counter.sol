// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    error InvalidNumber(uint256 number, string message);
    // This dummy error is only present to allow the errors from ethers-rs abigen to expand the errors properly. With 1 or less errors, it does not expand the errors per `contract::errors::Context`.
    error DummyError(uint256 dummy);

    function setNumber(uint256 newNumber) public {
        if (!isNumberValid(newNumber)) {
            revert InvalidNumber(newNumber, "Number must be even");
        }

        number = newNumber;
    }

    function setNumberV2(uint256 newNumber) public {
        require(isNumberValid(newNumber), "Number must be even");

        number = newNumber;
    }

    function isNumberValid(uint256 newNumber) public pure returns (bool) {
        return newNumber % 2 == 0;
    }

    function increment() public {
        number++;
    }
}
