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
mod error;
mod input;
mod output;
mod player;
mod result;
mod soundfont;

use app::AutoComposer;
use args::{Cli, Commands, Generate};
use error::Error;
use input::Input;
use output::Output;
use player::Player;
use result::Result;
use soundfont::SoundFont;

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
