/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// MIDI file to analyze
    #[arg(short, long)]
    pub file: PathBuf,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Run the generated file
    #[arg(short, long)]
    pub run: bool,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
