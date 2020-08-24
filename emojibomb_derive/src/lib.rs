extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn;


#[proc_macro_derive(WriteTo)]
pub fn write_to_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_write_to(&ast)
}

fn impl_write_to(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
                let buf: &[u8] = unsafe {
                    std::slice::from_raw_parts((self as *const Self) as *const u8, std::mem::size_of::<Self>())
                };
                writer.write_all(buf)
            }            
        }
        impl #name {
            pub async fn async_write_to(&self, writer: &mut (dyn tokio::io::AsyncWrite + std::marker::Unpin)) -> std::io::Result<()> {
                use tokio::io::AsyncWriteExt;
                let buf: &[u8] = unsafe {
                    std::slice::from_raw_parts((self as *const Self) as *const u8, std::mem::size_of::<Self>())
                };
                writer.write_all(buf).await
            }            
        }
    };
    gen.into()
}


#[proc_macro_derive(ReadFrom)]
pub fn read_from_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_read_from(&ast)
}

fn impl_read_from(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
                let mut buf = vec![0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                Ok(unsafe { std::ptr::read(buf.as_ptr() as *const _) })
            }
        }
        impl #name {
            pub async fn async_read_from(reader: &mut (dyn tokio::io::AsyncRead + std::marker::Unpin)) -> std::io::Result<Self> {
                use tokio::io::AsyncReadExt;
                let mut buf = vec![0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf).await?;
                Ok(unsafe { std::ptr::read(buf.as_ptr() as *const _) })
            }
        }
    };
    gen.into()
}
