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

use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    ReadInputFile(io::Error),
    ParseInputFile(midly::Error),
    ReadSoundFontFile(io::Error),
    ParseSoundFontFile(/* TODO */),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadInputFile(err) => write!(f, "Failed to read input file: {err}"),
            Self::ParseInputFile(err) => write!(f, "Failed to parse MIDI file: {err}"),
            Self::ReadSoundFontFile(err) => write!(f, "Failed to read sound font file: {err}"),
            Self::ParseSoundFontFile(/* TODO */) => write!(f, "Failed to parse sound font file: {}", "todo"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ReadInputFile(err) => Some(err),
            Self::ParseInputFile(err) => Some(err),
            Self::ReadSoundFontFile(err) => Some(err),
            Self::ParseSoundFontFile(/* TODO */) => None,
        }
    }
}
