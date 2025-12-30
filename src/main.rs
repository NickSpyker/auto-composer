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

mod app;
mod args;
mod core;
mod media;

use app::{AutoComposer, Input, Output};
use args::{Cli, Commands, Generate};
use core::{Error, Result};
use media::{Player, SoundFont};

fn main() -> Result<()> {
    match Cli::parse() {
        Commands::List => {
            println!("Available built-in soundfonts:");

            SoundFont::list()
                .into_iter()
                .for_each(|sound| println!("  - {sound}"));

            Ok(())
        }
        Commands::Generate(args) => {
            let input = Input::build(&args)?;

            let output = AutoComposer::run(&input)?;

            output.process()?;

            if args.run {
                let soundfont = if let Some(file) = args.custom_sound {
                    SoundFont::new_from_file(file)?
                } else {
                    SoundFont::new_from_name(&args.sound)?
                };

                let player = Player::new(input.smf, soundfont)?;

                player.run()?;
            }

            Ok(())
        }
    }
}
