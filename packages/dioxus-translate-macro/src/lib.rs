extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input, Ident, Lit, Token};

#[proc_macro]
pub fn translate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TranslateInput);

    let struct_name = input.struct_name;
    let mut fields = Vec::new();
    let mut ko_impl = Vec::new();
    let mut en_impl = Vec::new();

    for field in input.fields {
        let field_name = field.field_name;

        fields.push(quote! {
            pub #field_name: &'static str,
        });

        let mut ko_value = None;
        let mut en_value = None;

        for translation in field.translations {
            if translation.lang == "ko" {
                ko_value = Some(translation.value);
            } else if translation.lang == "en" {
                en_value = Some(translation.value);
            }
        }

        ko_impl.push(quote! {
            #field_name: #ko_value,
        });

        en_impl.push(quote! {
            #field_name: #en_value,
        });
    }

    let en = quote! {
            fn en() -> Self {
                Self {
                    #(#en_impl)*
                }
            }
    };

    #[allow(unused_variables)]
    let ko = quote! {};

    #[cfg(feature = "ko")]
    let ko = quote! {
        fn ko() -> Self {
                Self {
                    #(#ko_impl)*
                }
            }

    };

    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #struct_name {
            #(#fields)*
        }

        impl dioxus_translate::Translator for #struct_name {
            #en

            #ko
        }
    };

    TokenStream::from(expanded)
}

/// Macro input structure
struct TranslateInput {
    struct_name: Ident,
    fields: Vec<FieldTranslations>,
}

struct FieldTranslations {
    field_name: Ident,
    translations: Vec<LanguageTranslation>,
}

struct LanguageTranslation {
    lang: Ident,
    value: String,
}

impl Parse for TranslateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the struct name
        let struct_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        let mut fields = Vec::new();

        // Parse fields
        while !input.is_empty() {
            let field_name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let content;
            braced!(content in input);

            let mut translations = Vec::new();

            while !content.is_empty() {
                let lang: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let value: Lit = content.parse()?;
                content.parse::<Token![,]>().ok(); // Allow trailing commas
                if let Lit::Str(lit_str) = value {
                    translations.push(LanguageTranslation {
                        lang,
                        value: lit_str.value(),
                    });
                }
            }

            fields.push(FieldTranslations {
                field_name,
                translations,
            });

            input.parse::<Token![,]>().ok(); // Allow trailing commas
        }

        Ok(TranslateInput {
            struct_name,
            fields,
        })
    }
}
