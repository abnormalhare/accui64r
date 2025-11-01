use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{self, Token};
use syn::parse::Parse;

struct Input {
    vmatch: syn::Ident,
    _discard1: Token![,],
    vcpu:   syn::Ident,
    _discard2: Token![,],
    vram:   syn::Ident,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            vmatch: input.parse()?,
            _discard1: input.parse()?,
            vcpu: input.parse()?,
            _discard2: input.parse()?,
            vram: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn find_opcode(input: TokenStream) -> TokenStream {
    let Input {
        vmatch,
        vcpu,
        vram,
        ..
    } = syn::parse(input).unwrap();

    
    let mut out_stream = TokenStream2::new();
    for i in 0u8..=255 {
        let ident = format_ident!("op_{:0>2x}", i);
        out_stream.extend(quote! { #i => #ident(#vcpu, #vram), });
    }

    let pound = quote! {#};

    quote! {
        match #vmatch {
            #out_stream
            #pound[allow(unreachable_code)]
            e => todo!("opcode {e} is not implemented")
        }
    }.into()
}

#[proc_macro]
pub fn find_opcode_0f(input: TokenStream) -> TokenStream {
    let Input {
        vmatch,
        vcpu,
        vram,
        ..
    } = syn::parse(input).unwrap();

    
    let mut out_stream = TokenStream2::new();
    for i in 0u8..=255 {
        let ident = format_ident!("op_0f_{:0>2x}", i);
        out_stream.extend(quote! { #i => #ident(#vcpu, #vram), });
    }

    let pound = quote! {#};

    quote! {
        match #vmatch {
            #out_stream
            #pound[allow(unreachable_code)]
            e => todo!("opcode {e} is not implemented")
        }
    }.into()
}