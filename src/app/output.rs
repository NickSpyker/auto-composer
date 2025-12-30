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

use crate::{Player, Result, SoundFont};
use midly::Smf;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Output {
    pub smf: Smf<'static>,
    pub output_file: Option<PathBuf>,
    pub run_with_sound: Option<SoundFont>,
}

impl Output {
    pub fn process(self) -> Result<()> {
        if let Some(soundfont) = self.run_with_sound {
            let player = Player::new(self.smf, soundfont)?;
            player.run()?;
        }

        Ok(())
    }
}
