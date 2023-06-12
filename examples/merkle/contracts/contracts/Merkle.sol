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

/// @title A starter application using Bonsai through the on-chain proxy.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
//       or difficult to implement function to a RISC Zero guest running on Bonsai.
contract Merkle is BonsaiApp {
  // Cache of the results calculated by our guest program in Bonsai.
  mapping(uint256 => bytes32) public merkle_cache;
  uint256 public merkle_cache_size;

  // Initialize the contract, binding it to a specified Bonsai proxy and RISC Zero guest image.
  constructor(
    IBonsaiProxy _bonsai_proxy,
    bytes32 _image_id
  ) BonsaiApp(_bonsai_proxy, _image_id) {}

  event MerkleCallback(bytes32 root);

  function merkle_root(
    bytes32[32] memory n
  ) external {
    submit_bonsai_request(abi.encode(n));
  }

  /// @notice Callback function logic for processing verified journals from Bonsai.
  function bonsai_callback(bytes memory journal) internal override {

    // Decode the journal into the result.
    (bytes32 root) = abi.decode(journal, (bytes32));
    emit MerkleCallback(root);
    merkle_cache[merkle_cache_size] = root;
    merkle_cache_size += 1;
  }
}
