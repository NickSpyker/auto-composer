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
    /// Path to the input MIDI file to analyze and compose from
    #[arg(short, long, value_name = "FILE")]
    pub file: PathBuf,

    /// Path where the generated MIDI file will be saved
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Play the generated composition immediately after creation
    #[arg(short, long)]
    pub run: bool,

    /// Display all available built-in soundfonts
    #[arg(short, long)]
    pub list: bool,

    /// Built-in soundfont to use for playback
    #[arg(short, long, value_name = "NAME", default_value = "piano")]
    pub sound: String,

    /// Path to a custom SF2 soundfont file for playback
    #[arg(short, long, value_name = "FILE")]
    pub custom_sound: Option<PathBuf>,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = <Self as Parser>::parse();

        if args.output.is_none() {
            args.run = true;
        }

        args
    }
}
