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
import {ERC721} from "./ERC721.sol";

contract HelloBonsai is BonsaiApp, ERC721 {

  mapping(uint256 => string) public token_svg_cache;

  event MintCallback(uint256 indexed id, string result);

  constructor(
    IBonsaiProxy _bonsai_proxy,
    bytes32 _image_id
  ) BonsaiApp(_bonsai_proxy, _image_id) {}

  function name() public view virtual override returns (string memory) {
    return "BONSAI-NFT";
  }

  function symbol() public view virtual override returns (string memory) {
    return "BNFT";
  }

  function bonsai_callback(bytes memory journal) internal override {
    (uint256 id, string memory result) = abi.decode(journal, (uint256, string));
    emit MintCallback(id, result);
    token_svg_cache[id] = result;
  }

  function tokenURI(uint256 id) public view virtual override returns (string memory) {
    if (!_exists(id)) revert TokenDoesNotExist();
    return token_svg_cache[id];
  }

  function mint(uint256 id) public virtual {
    _mint(msg.sender, id);
    submit_bonsai_request(abi.encode(id));
  }

  function burn(uint256 id) public virtual {
    _burn(msg.sender, id);
  }

}
