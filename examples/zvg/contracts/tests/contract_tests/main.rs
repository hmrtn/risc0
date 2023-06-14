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

//! Tests for the ZVG contract using a mock for the Bonsai proxy
//! contract.

pub mod utils;

use std::{collections::HashMap, error::Error};

use ethers::prelude::*;
use risc0_zkvm::sha::Digest;
use zvg_contracts::ZVG;
use zvg_methods::{ZVG_ELF, ZVG_ID};

use crate::utils::bonsai_test;

#[tokio::test]
pub async fn test_successful_contract_usage() -> Result<(), Box<dyn Error>> {
    let image_id = Digest::from(ZVG_ID);
    let registry = HashMap::from([(image_id, ZVG_ELF)]);

    bonsai_test(registry, |client, bonsai_mock_address| async move {
        // Deploy the ZVG contract.
        let hello_bonsai =
            ZVG::deploy(client.clone(), (bonsai_mock_address, H256(image_id.into())))?
                .send()
                .await?;

        // Subscribe to events on ZVG.
        let events = hello_bonsai.events();
        let mut subscription = events.subscribe().await?;

        // Call a function which offloads work to Bonsai.
        hello_bonsai.mint(U256::from(10)).send().await?;

        // Wait for the callback to come from Bonsai.
        let mut callback_log = subscription.next().await.unwrap()?;
        println!("callback_log: {}", callback_log);
        callback_log = subscription.next().await.unwrap()?;
        println!("callback_log: {:?}", callback_log);
        // Fail test for now.
        assert_eq!(1, 2);
        Ok(())
    })
    .await
}
