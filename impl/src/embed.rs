use std::borrow::Cow;

use proc_macro2::TokenStream as TokenStream2;
use rust_embed_for_web_utils::{get_files, Config, DynamicFile, EmbedableFile, FileEntry};

use crate::compress::{compress_br, compress_gzip};

/// Anything that can be embedded into the program.
///
/// We're using our own trait instead of the actual `ToTokens` trait because the
/// types we implement it for are not defined in this crate, so we'd have to
/// wrap all of them.
pub(crate) trait MakeEmbed {
    fn make_embed(&self) -> TokenStream2;
}

impl MakeEmbed for Vec<u8> {
    fn make_embed(&self) -> TokenStream2 {
        // Not sure why quote doesn't like it if I use #self here
        let v = self;
        quote! { &[#(#v),*] }
    }
}

impl MakeEmbed for Cow<'static, [u8]> {
    fn make_embed(&self) -> TokenStream2 {
        // We need to convert Cow to &[u8] to use it in the quote! macro
        let v: &[u8] = self.as_ref();
        quote! { &[#(#v),*] }
    }
}

impl MakeEmbed for Cow<'static, str> {
    fn make_embed(&self) -> TokenStream2 {
        // We need to convert Cow to String to use it in the quote! macro
        let s: &str = self.as_ref();
        quote! { #s }
    }
}

impl MakeEmbed for i64 {
    fn make_embed(&self) -> TokenStream2 {
        quote! { #self }
    }
}

impl<T: MakeEmbed> MakeEmbed for Option<T> {
    fn make_embed(&self) -> TokenStream2 {
        match self {
            Some(v) => {
                let embed = v.make_embed();
                quote! { Some(#embed) }
            }
            None => quote! { None },
        }
    }
}

struct EmbedDynamicFile<'t> {
    file: &'t DynamicFile,
    config: &'t Config,
    rel_path: &'t str,
}

impl<'t> EmbedDynamicFile<'t> {
    fn new(file: &'t DynamicFile, config: &'t Config, rel_path: &'t str) -> EmbedDynamicFile<'t> {
        EmbedDynamicFile {
            file,
            config,
            rel_path,
        }
    }
}

impl<'t> MakeEmbed for EmbedDynamicFile<'t> {
    fn make_embed(&self) -> TokenStream2 {
        let file = self.file;
        let name = file.name().make_embed();
        // safety: `data()` will always return `Some` for dynamic files
        let data = file.data().unwrap();
        let data_gzip = if self.config.should_gzip() {
            compress_gzip(data.as_ref()).make_embed()
        } else {
            None::<Vec<u8>>.make_embed()
        };
        let data_br = if self.config.should_br() {
            compress_br(data.as_ref()).make_embed()
        } else {
            None::<Vec<u8>>.make_embed()
        };
        // for example, preserve_source = false, preserve_source_except = "*.html"
        // will only preserve source for files that end with `.html`.
        let mut preserve_source = self.config.should_preserve_source();
        if self.config.is_preserve_source_except(self.rel_path) {
            preserve_source = !preserve_source;
        }
        let data = if preserve_source { Some(data) } else { None };
        let data = data.make_embed();
        let hash = file.hash().make_embed();
        let etag = file.etag().make_embed();
        let last_modified = file.last_modified().make_embed();
        let last_modified_timestamp = file.last_modified_timestamp().make_embed();
        let mime_type = file.mime_type().make_embed();
        // Make sure that the order of these parameters is correct!
        quote! {
            rust_embed_for_web::EmbeddedFile::__internal_make(
                #name,
                #data,
                #data_gzip,
                #data_br,
                #hash,
                #etag,
                #last_modified,
                #last_modified_timestamp,
                #mime_type,
            )
        }
    }
}

pub(crate) fn generate_embed_impl(
    ident: &syn::Ident,
    config: &Config,
    folder_path: &str,
    prefix: &str,
) -> TokenStream2 {
    let embeds: Vec<TokenStream2> = get_files(folder_path, config, prefix)
        .filter_map(
            |FileEntry {
                 rel_path,
                 full_canonical_path,
             }| {
                if let Ok(file) = DynamicFile::read_from_fs(full_canonical_path) {
                    let file_embed =
                        EmbedDynamicFile::new(&file, config, rel_path.as_str()).make_embed();
                    Some(quote! {
                        #rel_path => Some(#file_embed),
                    })
                } else {
                    None
                }
            },
        )
        .collect();

    quote! {
      impl #ident {
          fn get(path: &str) -> Option<rust_embed_for_web::EmbeddedFile> {
              match path {
                    #(#embeds)*
                    _ => None,
              }
          }
      }

      impl rust_embed_for_web::RustEmbed for #ident {
        type File = rust_embed_for_web::EmbeddedFile;

        fn get(file_path: &str) -> Option<Self::File> {
          #ident::get(file_path)
        }
      }
    }
}
