// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug)]
pub(crate) struct WebhookError(pub(crate) anyhow::Error);

impl<E> From<E> for WebhookError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for WebhookError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {}", self.0.to_string()),
        )
            .into_response()
    }
}

pub(crate) type Result<T, E = WebhookError> = std::result::Result<T, E>;
