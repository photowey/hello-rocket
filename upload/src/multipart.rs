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

use rocket::form::Form;
use rocket::fs::TempFile;

#[derive(FromForm)]
pub struct Upload<'r> {
    file: TempFile<'r>,
}

// Try visiting:
// POST http://127.0.0.1:8000/upload/multipart
// curl -X POST -F "file=@one_more_light.jpg" "http://127.0.0.1:8000/upload/multipart"
#[post("/multipart", data = "<data>")]
pub async fn upload(data: Form<Upload<'_>>) -> std::io::Result<String> {
    let upload = data.into_inner();
    let tm = upload.file;
    let filename = tm
        .raw_name()
        .unwrap()
        .dangerous_unsafe_unsanitized_raw()
        .to_owned();
    let content_type = tm.content_type().unwrap();

    Ok(format!(
        "the file name is: [{}],content-type is: [{}]",
        filename.into_string(),
        content_type.to_string()
    ))
}
