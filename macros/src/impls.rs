use crate::*;
use proc_macro::TokenStream;
use quote::*;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::*;

impl Candle {
    pub fn _candle_field(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let idents = ident_struct_named_fields(&input);
        let ident_strings = idents.to_strings();
        let string_refs = to_string_refs(&ident_strings);
        let taf_output_names = string_refs
            .iter()
            .filter(|x| x.contains("_out_") || x.contains("num"))
            .collect::<Vec<_>>();
        let taf_output_name_idents = taf_output_names
            .iter()
            .map(|x| format_ident!("{}", x))
            .collect::<Vec<_>>();
        let type_strings = struct_types(&input).to_strings();
        let _type_string_refs = to_string_refs(&type_strings);

        let q = quote! {
            impl Candle {
                pub fn candle_field_f64(&self,field:&str) -> f64 {
                    match field {
                        #(#taf_output_names => {
                            self.#taf_output_name_idents
                        }),*
                        x => panic!("unknown candle field {:?}",x),
                    }
                }
                pub fn candle_field_f64_assign(&mut self,field:&str,value:f64){
                    match field {
                        #(#taf_output_names => {
                            self.#taf_output_name_idents = value;
                        }),*
                        x => panic!("unknown candle field {:?}",x),
                    }
                }
            }
        };
        q.into()
    }
}

impl Instrument {
    pub fn _instrument_from(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let idents = ident_enum_variants(&input);
        let strings = idents
            .to_strings()
            .iter()
            // This should be pascal
            // TODO - remove heck lib
            .map(|x| x.to_snake_case().to_uppercase())
            .collect::<Vec<_>>();

        let string_refs = strings.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let q = quote! {
            impl Instrument {
                pub fn new(value:&str) -> Self {
                    match value.to_uppercase().as_str() {
                        #(#string_refs => {
                            Instrument::#idents
                        }),*
                        x => panic!("unknown instrument {:?}",x),
                    }
                }
            }
        };
        q.into()
    }
    pub fn _instrument_all(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let exclude = env!("EXCLUDE").split(",").collect::<Vec<_>>();
        let mut idents = ident_enum_variants(&input);
        idents = idents
            .into_iter()
            .filter(|x| !exclude.contains(&x.to_string().as_str()))
            .collect::<Vec<_>>();
        let strings = idents.to_strings();
        let _string_refs = to_string_refs(&strings);
        let q = quote! {
            impl Instrument {
                pub fn instrument_all() -> Vec<Instrument> {
                    vec![
                        #(Instrument::#idents),*
                    ]
                }
            }
        };
        q.into()
    }
    pub fn _instrument_candles(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let idents = ident_enum_variants(&input)
            .iter()
            .map(|x| format_ident!("{}", x.to_string().to_snake_case()))
            .collect::<Vec<_>>();
        let strings = idents.to_strings();
        let string_refs = strings.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let q = quote! {
            impl Instrument {
                pub fn instrument_candles<'a>(self,y2:&'a YorexInstrument2) -> &'a Vec<Candle> {
                    match format!("{:?}",self).to_snake_case().as_str() {
                        #(#string_refs => {
                            &y2.candles.#idents
                        }),*
                        x => panic!("unknown instrument {:?}",x),
                    }
                }
            }
        };
        q.into()
    }
    pub fn _instrument_highest_cip(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let idents = ident_enum_variants(&input)
            .iter()
            .map(|x| format_ident!("{}", x.to_string().to_snake_case()))
            .collect::<Vec<_>>();
        let strings = idents.to_strings();
        let string_refs = strings.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let q = quote! {
            impl Instrument {
                pub fn highest_cip<'a>(ins:&Vec<Instrument>,co:&'a Combo) -> &'a ComboInstrument {
                    let mut ci = &co.aud_cad;
                    ins.iter().for_each(|x|{
                        match format!("{:?}",x).to_snake_case().as_str() {
                            #(#string_refs => {
                                if co.#idents.points > ci.points {
                                    ci = &co.#idents;
                                } else {
                                    println!("#0 {} {:?}",co.#idents.points,x);

                                }
                            }),*
                            x => panic!("unknown instrument {:?}",x),
                        };
                    });

                    if ci.points == 0.0 {

                        unimplemented!();
                    }
                    ci
                }
            }
        };
        q.into()
    }
    pub fn _instrument_ci(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let idents = ident_enum_variants(&input);
        let idents_snake = idents
            .iter()
            .map(|x| format_ident!("{}", x.to_string().to_snake_case()))
            .collect::<Vec<_>>();
        let strings = idents_snake.to_strings();
        let string_refs = strings.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let q = quote! {
            impl Instrument {
                pub fn ci(ins:Instrument,co:&Combo) -> &ComboInstrument {
                    let mut ci = ComboInstrument::default();
                    match format!("{:?}",ins).to_snake_case().as_str() {
                        #(#string_refs => {
                            &co.#idents_snake
                        }),*
                        x => panic!("unknown instrument {:?}",x),
                    }
                }
            }
        };
        q.into()
    }
    pub fn _instruments_ci_table_points(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let exclude = env!("EXCLUDE").split(",").collect::<Vec<_>>();
        let mut idents = ident_enum_variants(&input);
        idents = idents
            .into_iter()
            .filter(|x| !exclude.contains(&x.to_string().as_str()))
            .collect::<Vec<_>>();
        idents = idents
            .iter()
            .map(|x| format_ident!("{}", x.to_string().to_snake_case()))
            .collect::<Vec<_>>();
        let strings = idents.to_strings();
        let string_refs = strings.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let q = quote! {
            impl Instrument {
                pub fn ci_table_points(ci:&Combo) -> String {
                    let mut ci_tp:Vec<String> = vec![];
                    #(
                        ci_tp.push(format!("{} {}",#strings,ci.#idents.points));
                    )*
                    format!("{:?}",ci_tp)
                }
            }
        };
        q.into()
    }
    pub fn _instruments_combo_all_points(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let exclude = env!("EXCLUDE").split(",").collect::<Vec<_>>();
        let mut idents = ident_enum_variants(&input);
        idents = idents
            .into_iter()
            .filter(|x| !exclude.contains(&x.to_string().as_str()))
            .collect::<Vec<_>>();
        idents = idents
            .iter()
            .map(|x| format_ident!("{}", x.to_string().to_snake_case()))
            .collect::<Vec<_>>();
        let q = quote! {
            impl Instrument {
                pub fn combo_all_points(ci:&Combo) -> f64 {
                    let mut points:f64 = 0.0;
                    #(
                        points = points+ci.#idents.points;
                    )*
                    points
                }
            }
        };
        q.into()
    }
}
