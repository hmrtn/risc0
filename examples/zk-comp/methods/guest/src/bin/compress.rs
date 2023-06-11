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
#![feature(alloc_error_handler)]
extern crate alloc;
use alloc::vec::Vec;
use ethabi::{ethereum_types::U256, ParamType, Token};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn compress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let count = input[i..].iter().take_while(|&&x| x == input[i]).count();
        result.push(input[i]);
        result.extend(&count.to_ne_bytes());  // Convert count to bytes.
        i += count;
    }

    result
}

// const INPUT_LEN: usize = 256; // Fixed size input to 256 bytes.

const INPUT_LEN: usize = core::mem::size_of::<U256>();
pub fn main() {
    let mut input_bytes = [0u8; INPUT_LEN];
    env::read_slice(&mut input_bytes);
    let input = ethabi::decode_whole(&[ParamType::Uint(256)], &input_bytes).unwrap();
    // let mut input_bytes = [0u8; INPUT_LEN];
    // env::read_slice(&mut input_bytes);
    //
    // let input = ethabi::decode_whole(&[ParamType::Bytes], &input_bytes).unwrap();
    // let to_compress: Vec<u8> = input[0].clone().into_bytes().unwrap();
    //
    // let result = compress(&to_compress);

    // env::commit_slice(&ethabi::encode(&[Token::Bytes(to_compress), Token::Bytes(result)]));
}
