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

use crate::{Args, Error, Result};
use midly::Smf;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Input {
    pub smf: Smf<'static>,
    pub output_file: Option<PathBuf>,
    pub run: bool,
}

impl Input {
    pub fn build(args: Args) -> Result<Self> {
        let bytes = fs::read(&args.file).map_err(Error::ReadInputFile)?;
        let smf = Smf::parse(&bytes).map_err(Error::ParseInputFile)?;

        Ok(Self {
            smf: smf.make_static(),
            output_file: args.output,
            run: args.run,
        })
    }
}
