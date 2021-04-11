extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parser, parse_macro_input, Attribute, DeriveInput, Field, Type};

#[proc_macro_derive(FromCStruct, attributes(hstrace))]
pub fn from_c_struct(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;

    let fields: Vec<&Field> = match &derive_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named.iter().collect(),
            _ => panic!("FromCStruct fields must be named"),
        },
        _ => panic!("Must use FromCStruct for a struct"),
    };

    let attr: &Attribute = match derive_input
        .attrs
        .iter()
        .find(|x| x.path.is_ident("hstrace"))
    {
        Some(path) => path,
        None => panic!("hstrace macro requires hstrace attribute"),
    };

    let c_struct = attr.parse_args_with(KeyValParser::new("c_struct")).unwrap();

    let fields = Fields {
        fields,
        c_struct: c_struct.clone(),
    };

    let tok: TokenStream = (quote! {
        impl #ident {
            fn from_c(c: #c_struct) -> #ident {
                #ident {
                    #fields
                }
            }
        }

        impl CToCall for #ident {
            fn from_src<'a, T>(src: &mut ValueTransformer<'a, T>) -> Result<Value, crate::TraceError>
            where
                T: crate::Tracer,
            {
                Ok(Value::CStruct(CStruct::#c_struct(#ident::from_c(
                    src.to_type()?,
                ))))
            }
        }
    })
    .into();

    tok
}

struct KeyValParser<'a> {
    ident: &'a str,
}

impl<'a> KeyValParser<'a> {
    pub fn new(ident: &'a str) -> Self {
        KeyValParser { ident }
    }
}

impl<'a> Parser for KeyValParser<'a> {
    type Output = Ident;
    fn parse2(self, tokens: proc_macro2::TokenStream) -> Result<Self::Output, syn::Error> {
        let mut tokens = tokens.into_iter();
        let ne = tokens.next().unwrap();

        match ne {
            TokenTree::Ident(i) => {
                if i == self.ident {
                } else {
                    return Err(syn::Error::new(i.span(), "expected c_struct"));
                }
            }
            _ => return Err(syn::Error::new(ne.span(), "expected c_struct")),
        };

        let ne = tokens.next().unwrap();
        match ne {
            TokenTree::Punct(i) => {
                if i.as_char() != '=' {
                    return Err(syn::Error::new(i.span(), "expected punct ="));
                }
            }
            _ => return Err(syn::Error::new(ne.span(), "expected punct")),
        };

        let ne = tokens.next().unwrap();
        let ret = match ne {
            TokenTree::Ident(i) => i,
            _ => return Err(syn::Error::new(ne.span(), "expected ident")),
        };

        Ok(ret)
    }
}

struct Fields<'a> {
    pub fields: Vec<&'a Field>,
    pub c_struct: Ident,
}

impl<'a> ToTokens for Fields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let name = field.ident.as_ref().unwrap();

            let attr: &Attribute = match field.attrs.iter().find(|x| x.path.is_ident("hstrace")) {
                Some(path) => path,
                None => continue,
            };

            let x = match attr.parse_args::<Ident>() {
                Ok(tok) => match tok.to_string().as_str() {
                    "c_char" => quote! {
                        c_char_to_string(&c.#name as *const c_char)
                    },
                    _ => panic!("field {} has unknown ident {}", name, tok),
                },
                Err(_) => {
                    quote! {
                        c.#name as usize
                    }
                }
            };

            for token in quote! {
                #name: #x,
            } {
                tokens.append(token);
            }
        }
    }
}

#[proc_macro_derive(FromPtrace, attributes(hstrace))]
pub fn from_ptrace(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;
    let ident_lowcase_string = ident.to_string().to_lowercase();

    let fields: Vec<&Field> = match &derive_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named.iter().collect(),
            _ => panic!("FromCStruct fields must be named"),
        },
        _ => panic!("Must use FromCStruct for a struct"),
    };

    let fields = ConvertFields { fields };

    let mut humanize = quote! {};

    match derive_input
        .attrs
        .iter()
        .find(|x| x.path.is_ident("hstrace"))
    {
        Some(attribute) => {
            let attribute: &Attribute = attribute;
            let mut iter = attribute.tokens.clone().into_iter();
            let next: TokenTree = iter.next().unwrap();
            if let TokenTree::Group(grp) = &next {
                let stream = grp.stream();
                let mut iter = stream.into_iter();
                let first = iter.next().unwrap();
                if let TokenTree::Ident(i) = first {
                    if i.to_string() == "hmz" {
                        let second: TokenTree = iter.next().unwrap();
                        if let TokenTree::Group(grp) = second {
                            let format_parameters = grp.stream().to_token_stream();

                            // TODO: colorize parameters with cyan
                            humanize = quote! {
                                impl Humanize for #ident {
                                    fn hmz(&self) -> String {
                                        hmz_format(#ident_lowcase_string, &format!(#format_parameters))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => (),
    };

    //let c_struct = attr.parse_args_with(KeyValParser::new("c_struct")).unwrap();

    let tok: TokenStream = (quote! {
        #humanize
        impl From<TraceOutput> for Option<#ident> {
            fn from(s: TraceOutput) -> Option<#ident> {
                Some(#ident {
                    #fields
                })
            }
        }
    })
    .into();

    tok
}

struct ConvertFields<'a> {
    pub fields: Vec<&'a Field>,
}

impl<'a> ToTokens for ConvertFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for (i, field) in self.fields.iter().enumerate() {
            let name = field.ident.as_ref().unwrap();

            let c_struct = match field.attrs.iter().find(|x| x.path.is_ident("hstrace")) {
                Some(attribute) => match attribute.parse_args_with(KeyValParser::new("c_struct")) {
                    Ok(i) => Some(i),
                    Err(_) => None, // can be caused by erroneous c_struct attribute
                },
                None => None,
            };

            let mut is_option = false;

            let ident: &Ident = match &field.ty {
                Type::Path(p) => {
                    let mut iter = p.path.segments.iter();
                    let next = iter.next().unwrap();
                    if next.ident.to_string() == "Option" {
                        is_option = true;
                        match &next.arguments {
                            syn::PathArguments::AngleBracketed(a) => {
                                match &a.args.first().unwrap() {
                                    syn::GenericArgument::Type(t) => match t {
                                        Type::Path(p) => p.path.get_ident().unwrap(),
                                        _ => {
                                            panic!("Option arg must have Type::Path");
                                        }
                                    },
                                    _ => {
                                        panic!("WOT");
                                    }
                                }
                            }
                            _ => panic!("Option arg must have type Option<T>"),
                        }
                    } else {
                        match p.path.get_ident() {
                                    Some(i) => i,
                                    None => panic!("struct field types must be Idents (use String or SomeStruct instead of some::path::SomeStruct")
                                }
                    }
                }
                _ => panic!("struct has unknown -type field. Please use syn::Type::Path"),
            };

            let option_some: proc_macro2::TokenStream = if is_option {
                quote! { Some(s.clone()) }
            } else {
                quote! { s.clone() }
            };

            let option_none: proc_macro2::TokenStream = if is_option {
                quote! { None }
            } else {
                quote! { return None } // FIXME throw error?
            };

            let tt: proc_macro2::TokenStream = match c_struct {
                Some(struct_type) => {
                    // have CStruct
                    quote! {
                        #name: match &s.variables[#i] {
                            Value::CStruct(s) => match s {
                                CStruct::#struct_type (s) => #option_some,
                                _ => #option_none
                            }
                            _ => #option_none
                        },
                    }
                }

                None => {
                    // have one of standard types
                    let tt = match ident.to_string().as_str() {
                        "isize" => quote! { Int },
                        "usize" => quote! { SizeT },
                        "String" => quote! { CString },
                        _ => ident.to_token_stream(),
                    };

                    quote! {
                        #name: match &s.variables[#i] {
                            Value::#tt(s) => #option_some,
                            _ => #option_none,
                        },
                    }
                }
            };

            for token in tt {
                tokens.append(token);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_c_struct_2() {
        /*
        assert_eq!(
            from_c_struct(TokenStream::from(quote! {tok})).to_string(),
            TokenStream::from(quote! {tok}).to_string()
        );
        */
    }
}
