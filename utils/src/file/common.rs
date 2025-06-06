use std::borrow::Cow;

/// An embedable file.
///
/// The file is embedded into the program for release builds, and dynamically
/// read for debug builds. This trait allows common access for both types.
///
/// There are associated types for each file which you can get with
/// `EmbeddedFile::Data`/`EmbeddedFile::Meta` or
/// `DynamicFile::Data`/`DynamicFile::Meta`. The data of the file depends on
/// this associated types because embedded files use static types while dynamic
/// files have to use owned types.
///
/// You can access the data by calling the `as_ref` function through the `AsRef`
/// trait. For example:
/// ```
/// # use rust_embed_for_web_utils::EmbedableFile;
/// fn print_hash<T: EmbedableFile>(file: T) {
///     println!("The file hash is {}", file.hash().as_ref());
/// }
/// ```
use enum_dispatch::enum_dispatch;

use crate::{DynamicFile, EmbeddedFile};

#[enum_dispatch(EmbedableFile)]
pub enum EmbeddedFileImpl {
    EmbeddedFile,
    DynamicFile,
}

#[enum_dispatch]
pub trait EmbedableFile {
    /// The name of the embedded file.
    fn name(&self) -> Cow<'static, str>;
    /// The contents of the embedded file.
    fn data(&self) -> Option<Cow<'static, [u8]>>;
    /// The contents of the file, compressed with gzip.
    ///
    /// This is `Some` if precompression has been done. `None` if the file was
    /// not precompressed, either because the file doesn't benefit from
    /// compression or because gzip was disabled with `#[gzip = false]`.
    fn data_gzip(&self) -> Option<Cow<'static, [u8]>>;
    /// The contents of the file, compressed with brotli.
    ///
    /// This is `Some` if precompression has been done. `None` if the file was
    /// not precompressed, either because the file doesn't benefit from
    /// compression or because gzip was disabled with `#[br = false]`.
    fn data_br(&self) -> Option<Cow<'static, [u8]>>;
    /// The UNIX timestamp of when the file was last modified.
    fn last_modified_timestamp(&self) -> Option<i64>;
    /// The rfc2822 encoded last modified date. This is the format you use for
    /// `Last-Modified` headers.
    fn last_modified(&self) -> Option<Cow<'static, str>>;
    /// The hash value for the file. This is a base85 encoded sha256 hash.
    fn hash(&self) -> Cow<'static, str>;
    /// The ETag value for the file. This is just the file hash, wrapped with
    /// quote symbols.
    fn etag(&self) -> Cow<'static, str>;
    /// The mime type for the file, if one can be guessed from the file
    /// extension.
    fn mime_type(&self) -> Option<Cow<'static, str>>;
}
