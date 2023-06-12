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

#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use ethabi::ethereum_types::{U256, H256};
use ethabi::{ParamType, Token};
use risc0_zkvm::guest::env;
use rs_merkle::{
    algorithms::Sha256,
    Hasher, MerkleTree,
};

risc0_zkvm::guest::entry!(main);

fn merkle_root(input: &[H256]) -> H256 {
    let leaf_values = input.iter().map(|x| Sha256::hash(x.as_bytes())).collect::<Vec<[u8; 32]>>();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaf_values);
    let merkle_root = merkle_tree.root().ok_or("could not obtain merkle root");
    H256::from(merkle_root.unwrap())
}

const H256_LEN: usize = core::mem::size_of::<H256>();
const ARRAY_LEN: usize = 32;
const INPUT_LEN: usize = H256_LEN * ARRAY_LEN;

pub fn main() {
    // NOTE: Reads must be of known length. https://github.com/risc0/risc0/issues/402
    let mut input_bytes = [0u8; INPUT_LEN];
    env::read_slice(&mut input_bytes);
    // decode the H256 array
    let mut input = [H256::zero(); ARRAY_LEN];
    for i in 0..ARRAY_LEN {
        let mut bytes = [0u8; H256_LEN];
        bytes.copy_from_slice(&input_bytes[i * H256_LEN..(i + 1) * H256_LEN]);
        input[i] = H256::from_slice(&bytes);
    }

    // Run the computation.
    let result = merkle_root(&input);

    env::commit_slice(&ethabi::encode(&[
        Token::FixedBytes(result.as_bytes().to_vec()),
    ]));
}
