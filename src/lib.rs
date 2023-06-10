//! ```rust(ignore)
//! let (_, _) = (APP_ARGS.deref(), APP_CONF.deref());
//!
//! let worker = thread::Builder::new()
//!   .name(THREAD_NAME_TASKER.to_string())
//!   .spawn(|| TaskerBuilder::build().run());
//! ```
#![deny(unstable_features, unused_imports, unused_qualifications, clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod channel;
mod config;
mod executor;
mod lexer;
mod query;
mod stopwords;
mod store;
mod tasker;

use config::options::Config;
use config::reader::ConfigReader;

pub static THREAD_NAME_TASKER: &str = "sonic-tasker";
pub use channel::message::ChannelMessage;
pub use std::ops::Deref;
pub use tasker::runtime::TaskerBuilder;

pub use store::fst::StoreFSTPool;
pub use store::kv::StoreKVPool;
// pub use std::str::FromStr;
pub use std::thread;
pub use std::time::Duration;
pub struct AppArgs {
    config: String,
}

lazy_static! {
    pub static ref APP_ARGS: AppArgs = AppArgs {
        config: "./config.cfg".into()
    };
    pub static ref APP_CONF: Config = ConfigReader::make();
}
