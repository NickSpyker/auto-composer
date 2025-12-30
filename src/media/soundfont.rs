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
use soundfont::SoundFont2;
use std::{fs, io::Cursor, path::PathBuf};

static SOUNDFONT_PIANO: &[u8] = include_bytes!("../../assets/soundfont/piano.sf2");

#[derive(Debug, Default)]
pub enum SoundFont {
    #[default]
    Piano,
    FromFile(Vec<u8>),
}

impl SoundFont {
    pub fn new_from_name(name: &str) -> Result<SoundFont> {
        match name {
            "default" | "piano" => Ok(SoundFont::Piano),
            invalid => Err(Error::BuiltInSound(invalid.to_string())),
        }
    }

    pub fn new_from_file(file: PathBuf) -> Result<Self> {
        let bytes = fs::read(file).map_err(Error::ReadSoundFontFile)?;

        let mut cursor = Cursor::new(&bytes);
        SoundFont2::load(&mut cursor).map_err(Error::ParseSoundFontFile)?;

        Ok(Self::FromFile(bytes))
    }

    pub fn get_bytes(&self) -> &[u8] {
        match &self {
            Self::Piano => SOUNDFONT_PIANO,
            Self::FromFile(bytes) => bytes,
        }
    }

    pub fn list() -> Vec<String> {
        vec![String::from("piano")]
    }
}
