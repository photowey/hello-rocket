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

// Try visiting:
// http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// ----------------------------------------------------------------

/*

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
}

*/

fn rocket_setup() -> Rocket<Build> {
    rocket::build().mount("/hello", routes![world])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket_setup().launch().await?;

    Ok(())
}
