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

use alloc::vec;
use alloc::vec::Vec;
use ethabi::{Token};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn compress(input: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let count = input[i..]
            .iter()
            .take_while(|&&x| x == input[i])
            .take(u8::MAX as usize)  // Restrict to 255 counts.
            .count();

        // Check for more than 255 repeats.
        // If there are, break up the repeats into multiple runs.
        if count == u8::MAX as usize && i + count != input.len() && input[i] == input[i + count] {
            let mut j = 0;
            while j < count {
                let count2 = input[i + j..]
                    .iter()
                    .take_while(|&&x| x == input[i + j])
                    .take(u8::MAX as usize)  // Restrict to 255 counts.
                    .count();
                result.push(input[i + j]);
                result.push(count2 as u8);  // Push count as u8.
                j += count2;
            }
            i += count;
            continue;
        }
        // if count == u8::MAX as usize && i + count != input.len() && input[i] == input[i + count] {
        //     panic!("Too many repeated bytes in input.");
        // }

        result.push(input[i]);
        result.push(count as u8);  // Push count as u8.
        i += count;
    }

    result
}

const INPUT_LEN: usize = 14;//core::mem::size_of::<U256>();

pub fn main() {
    let mut input_bytes = vec![0u8; INPUT_LEN];
    env::read_slice(&mut input_bytes);

    let result = compress(input_bytes);
    env::commit_slice(&ethabi::encode(&[Token::Bytes(result)]));
}
