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

use crate::{Error, Result};
use std::{fs, path::PathBuf};

static SOUNDFONT_PIANO: &[u8] = include_bytes!("../assets/sound_font/piano.sf2");

#[derive(Debug, Default)]
pub enum SoundFont {
    #[default]
    Piano,
    FromFile(/* TODO */),
}

impl SoundFont {
    pub fn new_from_file(file: PathBuf) -> Result<Self> {
        let bytes = fs::read(file).map_err(Error::ReadSoundFontFile)?;

        // TODO: parse bytes into soundfont format

        Ok(Self::FromFile(/* TODO */))
    }

    pub fn get_bytes(&self) -> &[u8] {
        match &self {
            Self::Piano => SOUNDFONT_PIANO,
            Self::FromFile(/* TODO */) => &[0],
        }
    }
}
