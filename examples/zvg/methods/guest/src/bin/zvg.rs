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

use ethabi::ethereum_types::U256;
use ethabi::{ParamType, Token};
use risc0_zkvm::guest::env;

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

risc0_zkvm::guest::entry!(main);

struct ColorGenerator {
    rng: SmallRng,
}

impl ColorGenerator {
    fn new(seed: u64) -> Self {
        ColorGenerator {
            rng: SmallRng::seed_from_u64(seed),
        }
    }
    fn random_colors(&mut self, num_colors: usize) -> Vec<String> {
        (0..num_colors).map(|_| self.random_color()).collect()
    }
    fn random_color(&mut self) -> String {
        format!("#{:06x}", self.rng.gen_range(0..0x1000000))
    }
}

struct SvgGenerator {
    color_generator: ColorGenerator,
    num_colors: usize,
    id: u64,
}

impl SvgGenerator {
    fn new(id: u64, seed: u64, num_colors: usize) -> Self {
        SvgGenerator {
            color_generator: ColorGenerator::new(seed),
            num_colors,
            id,
        }
    }
    fn generate_svg(&mut self) -> String {
        let colors = self.color_generator.random_colors(self.num_colors);
        let stops = colors.iter().enumerate().map(|(i, color)| {
            let offset = (i as f32 / self.num_colors as f32) * 100.0;
            format!(r#"<stop offset="{}%" stop-color="{}"/>"#, offset, color)
        }).collect::<Vec<String>>().join("");
        format!(
            r##"<svg viewBox="0 0 148 148"><defs><radialGradient id="a" gradientTransform="matrix(2 0 0 2 -.9 0)"><stop offset="0%" stop-color="{}"/></radialGradient><linearGradient id="b" x1=".35" y1=".02" x2=".65" y2=".98">{}</linearGradient></defs><rect fill="url(#a)" height="100%" width="100%"/><rect fill="url(#b)" height="100%" width="100%"/><text x="74" y="24" font-family="-apple-system, system-ui, BlinkMacSystemFont, Roboto" dominant-baseline="middle" text-anchor="middle" font-size="18" fill="#74838f" font-weight="700">{}</text></svg>"##,
            colors[0],
            stops,
            self.id,
        ).to_string()
    }
}

const INPUT_LEN: usize = core::mem::size_of::<U256>();

pub fn main() {
    let mut input_bytes = [0u8; INPUT_LEN];
    env::read_slice(&mut input_bytes);
    let input = ethabi::decode_whole(&[ParamType::Uint(256)], &input_bytes).unwrap();
    let id: U256 = input[0].clone().into_uint().unwrap();
    // Custom amount of colors
    let num_colors = 11;
    // Use id as seed
    let mut svg_generator = SvgGenerator::new(id.as_u64(), id.as_u64(), num_colors);
    let result = svg_generator.generate_svg();

    env::commit_slice(&ethabi::encode(&[Token::Uint(id), Token::String(result)]));
}
