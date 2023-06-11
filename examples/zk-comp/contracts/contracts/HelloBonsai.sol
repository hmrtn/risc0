// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.16;

import {IBonsaiProxy} from "./IBonsaiProxy.sol";
import {BonsaiApp} from "./BonsaiApp.sol";

/// @title A simple example of a Bonsai application contract.
/// @dev This contract demonstrates basic RLE compression using Bonsai to offload the computation.
contract HelloBonsai is BonsaiApp {
  // Cache of the results calculated by our guest program in Bonsai.
  mapping(uint256 => uint256) public fibonnaci_cache;
  mapping(bytes32 => bytes) public compress_cache;

  // Initialize the contract, binding it to a specified Bonsai proxy and RISC Zero guest image.
  constructor(
    IBonsaiProxy _bonsai_proxy,
    bytes32 _image_id
  ) BonsaiApp(_bonsai_proxy, _image_id) {}

  event CompressCallback(bytes32 indexed hash, bytes result);

  /// @notice Sends a request to Bonsai to have have the bytes compressed using RLE.
  /// @dev This function sends the request to Bonsai through the on-chain proxy.
  function compress_bytes(bytes memory input) external {
    submit_bonsai_request(input);
  }

  /// @notice Callback function logic for processing verified journals from Bonsai.
  function bonsai_callback(bytes memory journal) internal override {
    emit CompressCallback(keccak256(journal), journal);
    compress_cache[keccak256(journal)] = journal;
  }
}
