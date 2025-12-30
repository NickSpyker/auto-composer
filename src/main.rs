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
mod result;

use app::AutoComposer;
use args::Args;
use error::Error;
use input::Input;
use output::Output;
use result::Result;

fn main() -> Result<()> {
    let args = Args::parse();

    let input = Input::build(args)?;

    let output = AutoComposer::run(input)?;

    output.process()
}
