/*
 * Copyright © 2024 the original author or authors.
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

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

// ----------------------------------------------------------------

#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

mod multipart;

// ----------------------------------------------------------------

// Try visiting:
// GET http://127.0.0.1:8000/upload/greeting
#[get("/greeting")]
fn world() -> &'static str {
    "Hello, upload!"
}

// ----------------------------------------------------------------
// ----------------------------------------------------------------

fn rocket_setup() -> Rocket<Build> {
    rocket::build().mount("/upload", routes![world, multipart::upload])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket_setup().launch().await?;

    Ok(())
}
