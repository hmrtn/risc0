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

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy
//! contract.

pub mod utils;

use std::{collections::HashMap, error::Error};

use ethers::prelude::*;
use merkle_contracts::Merkle;
use merkle_methods::{FIBONACCI_ELF, FIBONACCI_ID};
use risc0_zkvm::sha::Digest;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

use crate::utils::bonsai_test;

#[tokio::test]
pub async fn test_successful_contract_usage() -> Result<(), Box<dyn Error>> {
    let image_id = Digest::from(FIBONACCI_ID);
    let registry = HashMap::from([(image_id, FIBONACCI_ELF)]);

    fn test_merklize(input: &[H256]) -> H256 {
        let leaf_values = input
            .iter()
            .map(|x| Sha256::hash(x.as_bytes()))
            .collect::<Vec<[u8; 32]>>();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaf_values);
        let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root");
        H256::from(merkle_root.unwrap())
    }

    bonsai_test(registry, |client, bonsai_mock_address| async move {
        let merkle = Merkle::deploy(client.clone(), (bonsai_mock_address, H256(image_id.into())))?
            .send()
            .await?;

        let events = merkle.events();
        let mut subscription = events.subscribe().await?;

        let mut input = [[0u8; 32]; 32];
        println!("input: {:?}", input);

        let expected_input = vec![H256::zero(); 32];
        println!("expected input: {:?}", expected_input);
        let expected_merklized = test_merklize(&expected_input);
        println!("test_merklized: {:?}", expected_merklized);
        merkle.merkle_root(input).send().await?;

        let callback_log = subscription.next().await.unwrap()?;
        println!("callback_log: {:?}", callback_log.root);

        let mut root = Vec::new();
        for i in 0..callback_log.root.len() / 32 {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&callback_log.root[i * 32..(i + 1) * 32]);
            root.push(H256::from(hash));
        }
        println!("journal: {:?}", root);

        assert_eq!(expected_merklized, root[0]);
        assert_eq!(1, 2);
        Ok(())
    })
    .await
}
