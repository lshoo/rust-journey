use core::panic;
use std::iter::Map;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    punctuated::{Iter, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path, Type,
    TypePath,
};

pub struct BuilderContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}

type TokenStreamIter<'a> = Map<Iter<'a, Field>, fn(&'a Field) -> TokenStream>;

impl BuilderContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        Self { name, fields }
    }

    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        // builder name: {}Builder, e.g. CommandBuilder
        let builder_name = Ident::new(&format!("{}Builder", name), name.span());

        // optional fields, e.g. executable: String -> executable: Option<String>
        let optionized_fields = self.gen_optionized_fields();

        // methods: fn executable(mut self, v: String) -> Self { self.executable(Some(v); self) }
        // Command.builer().executable("hello").args(vec![]).envs(vec![]).finish()
        let methods = self.gen_methods();

        // assign Builder fields back to orignal struct fields
        // #field_name: self.field_name.take().ok_or("xxx need to be set!")
        let assigns = self.gen_assigns();

        quote! {
            /// Builder sructure
            #[derive(Debug, Default)]
            struct #builder_name {
                #(#optionized_fields,)*
            }

            impl #builder_name {
                #(#methods)*

                pub fn finish(mut self) -> Result<#name, &'static str> {
                    Ok(#name {
                        #(#assigns,)*
                    })
                }
            }

            impl #name {
                fn builder() -> #builder_name {
                    Default::default()
                }
            }
        }
    }

    fn gen_optionized_fields(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (_, ty) = get_option_inner(&f.ty);
            let name = &f.ident;
            quote! { #name: std::option::Option<#ty> }
        })
    }

    fn gen_methods(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (_, ty) = get_option_inner(&f.ty);
            let name = &f.ident;
            quote! {
                pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                    self.#name = Some(v.into());
                    self
                }
            }
        })
    }

    fn gen_assigns(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (optional, _) = get_option_inner(&f.ty);
            let name = &f.ident;
            // #field_name: self.field_name.take().ok_or("xxx need to be set!")
            if optional {
                quote! {
                    #name: self.#name.take()
                }
            } else {
                quote! {
                    #name: self.#name.take().ok_or(concat!(stringify!(#name), " need to be set"))?
                }
            }
        })
    }
}

fn get_option_inner(ty: &Type) -> (bool, &Type) {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArgument"),
                };

                return (true, t);
            }
        }
    }

    (false, ty)
}

// fn is_optional(ty: &Type) -> bool {
//     if let Type::Path(TypePath { path: Path { segments, ..}, .. }) = ty {
//         if let Some(v) = segments.iter().next() {
//             return v.ident == "Option";
//         }
//     }

//     false
// }
