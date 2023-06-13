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
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

risc0_zkvm::guest::entry!(main);

struct Stop {
    offset: &'static str,
    color: String,
}

struct Gradient {
    id: u8,
    is_radial: bool,
    transform: Option<&'static str>,
    stops: Vec<Stop>,
}
struct ColorGenerator {
    rng: SmallRng,
}

impl ColorGenerator {
    fn new(seed: [u8; 16]) -> Self { // NOTE: 16 bytes is the size of a seed for SmallRng.
        ColorGenerator {
            rng: SmallRng::from_seed(seed),
        }
    }

    fn random_color(&mut self) -> String {
        format!("#{:06x}", self.rng.gen_range(0..0x1000000))
    }

}

impl Gradient {
    fn render(&self) -> String {
        let gradient_type = if self.is_radial { "radialGradient" } else { "linearGradient" };
        let transform = if let Some(t) = self.transform {
            format!(" gradientTransform=\"{}\"", t)
        } else {
            String::new()
        };

        let mut stops = String::new();
        for stop in &self.stops {
            stops.push_str(&format!(
                "<stop offset=\"{}\" stop-color=\"{}\"/>\n",
                stop.offset, stop.color
            ));
        }

        format!(
            "<{} id=\"{}\"{} x1=\"0.5\" y1=\"0\" x2=\"0.5\" y2=\"1\">\n{}\n</{}>\n",
            gradient_type, self.id, transform, stops, gradient_type
        )
    }
}


fn fibonacci(n: U256) -> U256 {
    let mut color_generator = ColorGenerator::new([0; 16]); // NOTE: 16 bytes is the size of a seed for SmallRng.
    let gradients = vec![
        Gradient {
            id: 0,
            is_radial: false,
            transform: None,
            stops: vec![
                Stop {
                    offset: "0%",
                    color: color_generator.random_color(),
                },
            ],
        },
        Gradient {
            id: 1,
            is_radial: true,
            transform: Some("translate(-1 -0.5) scale(2, 2)"),
            stops: vec![
                Stop {
                    offset: "0%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "7.33%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "14.67%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "22%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "29.33%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "36.67%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "44%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "51.33%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "58.67%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "66%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "73.33%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "80.67%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "88%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "95.33%",
                    color: color_generator.random_color(),
                },
                Stop {
                    offset: "100%",
                    color: color_generator.random_color(),
                },
            ],
        },
    ];

    let defs: String = gradients.iter().map(|g| g.render()).collect();

    let svg = format!(
        "<?xml version=\"1.0\" standalone=\"no\"?>
        <svg width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 148 148\">
            <rect fill=\"#cfde5a\" height=\"100%\" width=\"100%\"/>
            <defs>
                {}
            </defs>
            <rect fill=\"url(#0)\" height=\"100%\" width=\"100%\"/>
            <rect fill=\"url(#1)\" height=\"100%\" width=\"100%\"/>
            <text x=\"74\" y=\"24\" font-family=\"-apple-system, system-ui, BlinkMacSystemFont, Roboto\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"18\" fill=\"#74838f\" font-weight=\"700\">
                Test
            </text>
        </svg>",
        defs
    );



    ///
    let (mut prev, mut curr) = (U256::one(), U256::one());
    for _ in 2..=n.as_u32() {
        (prev, curr) = (curr, prev + curr);
    }
    return curr;
}

const INPUT_LEN: usize = core::mem::size_of::<U256>();

pub fn main() {
    // NOTE: Reads must be of known length. https://github.com/risc0/risc0/issues/402
    let mut input_bytes = [0u8; INPUT_LEN];
    env::read_slice(&mut input_bytes);
    let input = ethabi::decode_whole(&[ParamType::Uint(256)], &input_bytes).unwrap();
    let n: U256 = input[0].clone().into_uint().unwrap();

    // Run the computation.
    let result = fibonacci(n);

    // Commit the journal that will be decoded in the application contract.
    env::commit_slice(&ethabi::encode(&[Token::Uint(n), Token::Uint(result)]));
}
