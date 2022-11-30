
//!
//! # Usage example
//! ```rust
//! use ecs_types::types::Timestamp;
//! use ecs_types::*;
//! use serde_json::json;
//! 
//! let now: Timestamp = chrono::offset::Local::now().into();
//! let mut base = Base::new(now.clone());
//! let mut file = File::default();
//! file.set_name("readme.txt".into());
//! file.set_mtime(now);
//! 
//! base.with_file(file);
//! println!("{}", serde_json::to_string_pretty(&json!(base)).unwrap() );
//! ```
//! 
//! creates the following result:
//! ```json
//! {
//!   "@timestamp": 1669822098181,
//!   "file": {
//!     "attributes": [],
//!     "mtime": 1669822098181,
//!     "name": "readme.txt"
//!   },
//!   "tags": []
//! }

//! ```
//! 
//! 
pub mod types;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
