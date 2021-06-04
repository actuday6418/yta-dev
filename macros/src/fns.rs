use syn::*;

pub fn ident_enum_variants(input: &DeriveInput) -> Vec<Ident> {
    match input.data {
        Data::Enum(DataEnum { ref variants, .. }) => {
            variants.iter().map(|x| x.ident.clone()).collect::<Vec<_>>()
        }
        _ => panic!("this is not an instrument struct"),
    }
}

pub fn ident_struct_named_fields(input: &DeriveInput) -> Vec<Ident> {
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|x| x.ident.clone().unwrap())
            .collect::<Vec<_>>(),
        _ => panic!("this derive macro only works on structs with named fields"),
    }
}

pub fn struct_types(input: &DeriveInput) -> Vec<Type> {
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|x| x.ty.clone())
            .collect::<Vec<_>>(),
        _ => panic!("this derive macro only works on structs with named fields"),
    }
}

pub fn to_string_refs(strings: &Vec<String>) -> Vec<&str> {
    strings.iter().map(|x| x.as_ref()).collect::<Vec<_>>()
}
