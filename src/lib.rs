//! Rust Macro which embeds files into your executable. A fork of `rust-embed`
//! with a focus on usage in web servers.
//!
//! Please check out the
//! [readme](https://github.com/SeriousBug/rust-embed-for-web/blob/master/readme.md)
//! in the repository to get started. There's an
//! [example](https://github.com/SeriousBug/rust-embed-for-web/blob/master/examples/actix.rs)
//! available too!
//!
//! If you are using this with Actix Web, there's an existing responder
//! [`actix-web-rust-embed-responder`](https://lib.rs/crates/actix-web-rust-embed-responder)
//! which will handle everything for you, from negotiating compressed responses
//! to cache revalidation.
#![forbid(unsafe_code)]

#[allow(unused_imports)]
#[macro_use]
extern crate rust_embed_for_web_impl;

pub use rust_embed_for_web_impl::*;

pub use rust_embed_for_web_utils::{DynamicFile, EmbedableFile, EmbeddedFile};

#[doc(hidden)]
pub extern crate rust_embed_for_web_utils as utils;

/// A folder of embedded files.
///
/// The type of the file `RustEmbed::File` depends on whether we're in debug
/// mode or release mode:
///
/// - In debug mode it will be a `DynamicFile`
/// - In release mode it will be a `EmbeddedFile`
///
/// The derivation will automatically generate the correct file type. You don't
/// need to directly interface with the different file types that might get
/// returned: you should instead use the `EmbedableFile`  trait which is
/// implemented for both.
pub trait RustEmbed {
    type File: EmbedableFile;

    /// Get a file out of the folder.
    fn get(file_path: &str) -> Option<Self::File>;
}
