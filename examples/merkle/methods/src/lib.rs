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
    use ethabi::{
        ethereum_types::{H256, U256},
        Token,
    };
    use risc0_zkvm::{Executor, ExecutorEnv};
    use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

    use super::MERKLE_ELF;

    #[test]
    fn fibonacci() {
        let test_input = vec![H256::from_low_u64_be(0xdeadbeefdeadbeef); 32];
        println!("test input: {:?}", test_input);
        let expected_root = test_merklize(&test_input);
        println!("expected merkle root: {:?}", expected_root);

        let encoded_test_input = ethabi::encode(&[Token::FixedBytes(
            test_input
                .into_iter()
                .map(|x| x.0)
                .flatten()
                .collect::<Vec<u8>>(),
        )]);
        println!("encoded test input: {:?}", encoded_test_input);

        let env = ExecutorEnv::builder()
            .add_input(&encoded_test_input)
            .build();
        let mut exec = Executor::from_elf(env.unwrap(), MERKLE_ELF).unwrap();
        let session = exec.run().unwrap();
        println!("session: {:?}", session.journal);

        let mut journal = Vec::new();
        for i in 0..session.journal.len() / 32 {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&session.journal[i * 32..(i + 1) * 32]);
            journal.push(H256::from(hash));
        }
        println!("journal: {:?}", journal);

        assert_eq!(expected_root, journal[0]);
    }
    fn test_merklize(input: &[H256]) -> H256 {
        let leaf_values = input
            .iter()
            .map(|x| Sha256::hash(x.as_bytes()))
            .collect::<Vec<[u8; 32]>>();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaf_values);
        let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root");
        H256::from(merkle_root.unwrap())
    }
}
