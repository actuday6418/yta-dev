extern crate proc_macro2;

mod fns;
mod impls;
mod structs;
mod traits;

use proc_macro::TokenStream;
use quote::*;
use syn::parse::*;
use syn::punctuated::*;
use syn::*;

use traits::to_lowercase::*;
use traits::to_strings::*;

use fns::*;
use heck::*;
use structs::*;
use syn::__private::TokenStream2;

#[proc_macro_derive(IdentitySelf)]
pub fn identity_self(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let id_name = input.ident;
    let ts = match input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let idents = variants.iter().map(|x| x.ident.clone()).collect::<Vec<_>>();
            quote! {
                impl IdentitySelf for #id_name {
                    fn identity_self(&self) -> String {
                        match self {
                            #( #id_name::#idents => {
                                stringify!(#idents).to_string()
                            } )*
                        }
                    }
                }
            }
        }
        _ => panic!("this derive macro only works on enums"),
    };
    TokenStream::from(ts)
}

#[proc_macro]
pub fn combo(_input: TokenStream) -> TokenStream {
    let parser = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty;
    let exprs = parser.parse(_input).unwrap().to_strings();
    let string_refs = to_string_refs(&exprs);
    let q = quote! {
        vec![
            #(
                Combo::new(#string_refs)
            ),*
        ]
    };
    TokenStream::from(q)
}

#[proc_macro_derive(Identity)]
pub fn identity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let id_name = input.ident;
    let expanded = quote! {
        impl Identity for #id_name {
            fn identity() -> String {
                stringify!(#id_name).to_string()
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(InstrumentDerive)]
pub fn instrument_derive(input: TokenStream) -> TokenStream {
    let mut ts = quote!();
    let q1 = Instrument::_instrument_from(input.clone());
    let q2 = Instrument::_instrument_all(input.clone());
    let q3 = Instrument::_instrument_candles(input.clone());
    let q4 = Instrument::_instrument_highest_cip(input.clone());
    let q5 = Instrument::_instrument_ci(input.clone());
    let q6 = Instrument::_instruments_ci_table_points(input.clone());
    let q7 = Instrument::_instruments_combo_all_points(input);

    let ts1 = proc_macro2::TokenStream::from(q1);
    let ts2 = proc_macro2::TokenStream::from(q2);
    let ts3 = proc_macro2::TokenStream::from(q3);
    let ts4 = proc_macro2::TokenStream::from(q4);
    let ts5 = proc_macro2::TokenStream::from(q5);
    let ts6 = proc_macro2::TokenStream::from(q6);
    let ts7 = proc_macro2::TokenStream::from(q7);

    ts.extend(quote! {
        #ts1
        #ts2
        #ts3
        #ts4
        #ts5
        #ts6
        #ts7
    });

    let q = quote! {
        #ts
    };
    q.into()
}

#[proc_macro_derive(CandleDerive)]
pub fn candle_derive(input: TokenStream) -> TokenStream {
    let mut ts = quote!();
    let candle_field = proc_macro2::TokenStream::from(Candle::_candle_field(input.clone()));

    ts.extend(quote! {
        #candle_field
    });
    let q = quote! {
        #ts
    };
    q.into()
}
