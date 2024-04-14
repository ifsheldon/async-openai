//! Rust library for OpenAI
//!
//! ## Creating client
//!
//! ```
//! use async_openai::{Client, config::OpenAIConfig};
//!
//! // Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
//! let client = Client::new();
//!
//! // Above is shortcut for
//! let config = OpenAIConfig::default();
//! let client = Client::with_config(config);
//!
//! // OR use API key from different source and a non default organization
//! let api_key = "sk-..."; // This secret could be from a file, or environment variable.
//! let config = OpenAIConfig::new()
//!     .with_api_key(api_key)
//!     .with_org_id("the-continental");
//!
//! let client = Client::with_config(config);
//!
//! // Use custom reqwest client
//! let http_client = reqwest::ClientBuilder::new().user_agent("async-openai-wasm").build().unwrap();
//! let client = Client::new().with_http_client(http_client);
//! ```
//!
//! ## Microsoft Azure Endpoints
//!
//! ```
//! use async_openai::{Client, config::AzureConfig};
//!
//! let config = AzureConfig::new()
//!     .with_api_base("https://my-resource-name.openai.azure.com")
//!     .with_api_version("2023-03-15-preview")
//!     .with_deployment_id("deployment-id")
//!     .with_api_key("...");
//!
//! let client = Client::with_config(config);
//!
//! // Note that Azure OpenAI service does not support all APIs and `async-openai-wasm`
//! // doesn't restrict and still allows calls to all of the APIs as OpenAI.
//!
//! ```
//!
//! ## Wasm Support
//!
//! Currently, `wasm32-unknown-unknown` target is support through feature flag `wasm`. To use this feature flag, you need to disable default features by add `default-features = false` under `async-openai-wasm` dependency to your `Cargo.toml` file.
//!
//! Enabling `wasm` will disable:
//! * retries
//! * file saving: You need to manage file operations yourself via web APIs.
//!
//! ## Making requests
//!
//!```
//!# tokio_test::block_on(async {
//!
//! use async_openai::{Client, types::{CreateCompletionRequestArgs}};
//!
//! // Create client
//! let client = Client::new();
//!
//! // Create request using builder pattern
//! // Every request struct has companion builder struct with same name + Args suffix
//! let request = CreateCompletionRequestArgs::default()
//!     .model("gpt-3.5-turbo-instruct")
//!     .prompt("Tell me the recipe of alfredo pasta")
//!     .max_tokens(40_u16)
//!     .build()
//!     .unwrap();
//!
//! // Call API
//! let response = client
//!     .completions()      // Get the API "group" (completions, images, etc.) from the client
//!     .create(request)    // Make the API call in that "group"
//!     .await
//!     .unwrap();
//!
//! println!("{}", response.choices.first().unwrap().text);
//! # });
//!```
//!
//! ## Examples
//! For full working examples for all supported features see [examples](https://github.com/64bit/async-openai/tree/main/examples) directory in the repository.
//!
//!
//! ## Feature Flags
//!
//! - `wasm`: Enables support for `wasm32-unknown-unknown` target
//!   - Disabling tokio support and backoff retries.
//!   - _Help wanted_ to re-enable backoff retries.
//! - `backoff`: Enables backoff retries for all requests.
//!   - Enabled by default.
//!   - Disabling this feature will disable all retries.
//! - `tokio`: Enables support for `tokio` runtime.
//!   - Enabled by default.
//!   - _Now_ disabling this feature will disable all media related functionalities.

mod assistant_files;
mod assistants;
mod audio;
mod chat;
mod client;
mod completion;
pub mod config;
mod embedding;
pub mod error;
mod file;
mod fine_tuning;
mod image;
mod message_files;
mod messages;
mod model;
mod moderation;
mod runs;
mod steps;
mod threads;
pub mod types;
mod util;

pub use assistant_files::AssistantFiles;
pub use assistants::Assistants;
pub use audio::Audio;
pub use chat::Chat;
pub use client::Client;
pub use completion::Completions;
pub use embedding::Embeddings;
pub use file::Files;
pub use fine_tuning::FineTuning;
pub use image::Images;
pub use message_files::MessageFiles;
pub use messages::Messages;
pub use model::Models;
pub use moderation::Moderations;
pub use runs::Runs;
pub use steps::Steps;
pub use threads::Threads;