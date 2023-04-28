use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{Data, Fields, Meta};

pub fn map_enum(args: TokenStream, body: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::Meta);
    let mut input = syn::parse_macro_input!(body as syn::DeriveInput);
    let Data::Enum(ref mut enum_) = input.data else {
        abort! {
            input, "Only enums are supported"
        }
    };

    let ident = input.ident.clone();

    let from_impl = match args {
        Meta::Path(path) => {
            let ts_ty = path.to_token_stream();
            let branches: Vec<Ts2> = enum_
                .variants
                .iter_mut()
                .map(|v| {
                    if !matches!(v.fields, Fields::Unit) {
                        abort! {
                            v.fields, "Only unit fields are supported"
                        }
                    }

                    let var_ident = &v.ident;
                    let discr = {
                        let Some((_, discr)) = &v.discriminant else {
                            abort! {
                                v, "No base enum variant specified"
                            }
                        };
                        discr.to_token_stream()
                    };
                    v.discriminant = None;

                    quote! {
                        #ident::#var_ident => Self::#discr
                    }
                })
                .collect();

            quote! {
                impl From<#ident> for #ts_ty {
                    fn from(value: #ident) -> #ts_ty {
                        match value {
                            #(#branches),*
                        }
                    }
                }
            }
        }

        _ => unreachable!(),
    };

    quote! {
        #input
        #from_impl
    }
    .into()
}
