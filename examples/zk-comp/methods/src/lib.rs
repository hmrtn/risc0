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

//! Generated create containing the image ID and ELF binary of the build guest.

include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use ethabi::Token;
    use risc0_zkvm::{Executor, ExecutorEnv};

    use super::COMPRESS_ELF;

    #[test]
    fn compress_test() {
        let input: Vec<u8> = b"AAAAAAAAAAAAAA!".to_vec();
        let env = ExecutorEnv::builder()
            .add_input(&input) //&ethabi::encode(&[Token::Bytes(input)]))
            .build();
        let mut exec = Executor::from_elf(env.unwrap(), COMPRESS_ELF).unwrap();
        let session = exec.run().unwrap();
        assert_eq!(
            &session.journal,
            &ethabi::encode(&[Token::Bytes(vec![b'A', 14])])
        );
    }
}