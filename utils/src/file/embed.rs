use super::common::EmbedableFile;
use std::{borrow::Cow, fmt::Debug};

/// A file embedded into the binary.
///
/// `rust-embed-for-web` changes which type of file you get based on whether
/// it's a debug or release build. In release builds or with the `always-embed`
/// flag, you'll get `EmbeddedFile`s.
///
/// You should interface with this object using the `EmbedableFile` trait, which
/// is implemented for both the embedded and dynamic files.
#[derive(Clone, Copy)]
pub struct EmbeddedFile {
    name: &'static str,
    data: Option<&'static [u8]>,
    data_gzip: Option<&'static [u8]>,
    data_br: Option<&'static [u8]>,
    hash: &'static str,
    etag: &'static str,
    last_modified: Option<&'static str>,
    last_modified_timestamp: Option<i64>,
    mime_type: Option<&'static str>,
}

impl EmbedableFile for EmbeddedFile {
    fn name(&self) -> Cow<'static, str> {
        Cow::from(self.name)
    }

    fn data(&self) -> Option<Cow<'static, [u8]>> {
        self.data.map(Cow::from)
    }

    fn data_gzip(&self) -> Option<Cow<'static, [u8]>> {
        self.data_gzip.map(Cow::from)
    }

    fn data_br(&self) -> Option<Cow<'static, [u8]>> {
        self.data_br.map(Cow::from)
    }

    fn last_modified(&self) -> Option<Cow<'static, str>> {
        self.last_modified.map(Cow::from)
    }

    fn last_modified_timestamp(&self) -> Option<i64> {
        self.last_modified_timestamp
    }

    fn hash(&self) -> Cow<'static, str> {
        Cow::from(self.hash)
    }

    fn etag(&self) -> Cow<'static, str> {
        Cow::from(self.etag)
    }

    fn mime_type(&self) -> Option<Cow<'static, str>> {
        self.mime_type.map(Cow::from)
    }
}

impl EmbeddedFile {
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    /// This is used internally in derived code to create embedded file objects.
    /// You don't want to manually use this function!
    pub fn __internal_make(
        // Make sure that the order of these parameters is correct in respect to
        // the file contents! And if you are changing or reordering any of
        // these, make sure to update the corresponding call in `impl`
        name: &'static str,
        data: Option<&'static [u8]>,
        data_gzip: Option<&'static [u8]>,
        data_br: Option<&'static [u8]>,
        hash: &'static str,
        etag: &'static str,
        last_modified: Option<&'static str>,
        last_modified_timestamp: Option<i64>,
        mime_type: Option<&'static str>,
    ) -> EmbeddedFile {
        EmbeddedFile {
            name,
            data,
            data_gzip,
            data_br,
            hash,
            etag,
            last_modified,
            last_modified_timestamp,
            mime_type,
        }
    }
}

impl Debug for EmbeddedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddedFile")
            .field("name", &self.name)
            .field("hash", &self.hash)
            .field("last_modified", &self.last_modified())
            .field("mime_type", &self.mime_type)
            .finish()
    }
}

impl PartialEq for EmbeddedFile {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(other.hash)
    }
}
