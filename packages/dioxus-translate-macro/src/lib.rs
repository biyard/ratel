extern crate proc_macro;

use std::cell::RefCell;
use std::rc::Rc;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Fields, Ident, Lit, LitStr, Meta, Token, braced, parse_macro_input};

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

/// Implements a custom derive macro for `Translate`, which automatically generates
/// a `translate(&self, lang: &Language) -> &'static str` method for enums.
///
/// This macro extracts `#[translate(ko = "...")]` attributes from the enum variants
/// and maps them to Korean translations. If no translation is provided, the variant
/// name is used as the default English translation.
#[proc_macro_derive(Translate, attributes(translate))]
pub fn translate_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = ast.ident;

    // Ensure that the derive macro is applied to an enum
    let variants = match ast.data {
        syn::Data::Enum(ref data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(enum_name, "Translate can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let mut en_arms = Vec::new();
    #[cfg(feature = "ko")]
    let mut ko_arms = Vec::new();
    let mut display_arms = Vec::new();
    let mut from_str_arms = Vec::new();

    let mut idents: Vec<Ident> = Vec::new();
    let mut unit_variants = Vec::new();

    for variant in variants {
        let mut field_names = vec![];
        let mut tuple_len = 0;

        match variant.fields {
            Fields::Unit => {
                idents.push(variant.ident.clone());
                unit_variants.push(variant.ident.clone());
            }
            Fields::Named(ref f) => {
                idents.push(variant.ident.clone());
                for field in f.named.iter() {
                    field_names.push(field.ident.clone().unwrap());
                }
            }
            Fields::Unnamed(ref f) => {
                idents.push(variant.ident.clone());
                tuple_len = f.unnamed.len();
            }
        }
        let variant_ident = &variant.ident;
        let default_str = variant_ident.to_string();
        let en_translation = Rc::new(RefCell::new(default_str.clone()));
        #[cfg(feature = "ko")]
        let ko_translation = Rc::new(RefCell::new(default_str.clone()));

        // Process attributes to extract translations
        for attr in &variant.attrs {
            if let Meta::List(ref meta_list) = attr.meta {
                if meta_list.path.is_ident("translate") {
                    let en = Rc::clone(&en_translation);
                    #[cfg(feature = "ko")]
                    let ko = Rc::clone(&ko_translation);

                    let _ = meta_list.parse_nested_meta(move |nv| {
                        if nv.path.is_ident("en") {
                            let s: LitStr = nv.value()?.parse()?;
                            *en.borrow_mut() = s.value();
                        }

                        #[cfg(feature = "ko")]
                        if nv.path.is_ident("ko") {
                            let s: LitStr = nv.value()?.parse()?;
                            *ko.borrow_mut() = s.value();
                        }

                        Ok(())
                    });
                }
            }
        }

        let en_str = syn::LitStr::new(&en_translation.borrow(), proc_macro2::Span::call_site());
        #[cfg(feature = "ko")]
        let ko_str = syn::LitStr::new(&ko_translation.borrow(), proc_macro2::Span::call_site());
        let lower_name = syn::LitStr::new(
            &variant_ident.to_string().to_lowercase(),
            proc_macro2::Span::call_site(),
        );
        let arm_name = if field_names.len() > 0 {
            quote! {
                #enum_name::#variant_ident { .. }
            }
        } else if tuple_len > 0 {
            quote! {
                #enum_name::#variant_ident(..)
            }
        } else {
            quote! {
                #enum_name::#variant_ident
            }
        };

        let assigner = if field_names.len() > 0 {
            quote! {
                #enum_name::#variant_ident { #(#field_names: Default::default(),)* }
            }
        } else if tuple_len > 0 {
            let mut defaults = vec![];
            for _ in 0..tuple_len {
                defaults.push(quote! { Default::default() });
            }
            quote! {
                #enum_name::#variant_ident( #(#defaults,)* )
            }
        } else {
            quote! {
                #enum_name::#variant_ident
            }
        };

        en_arms.push(quote! {
            #arm_name => #en_str,
        });

        display_arms.push(quote! {
            #arm_name => write!(f, #lower_name),
        });

        if let Some((_, expr)) = &variant.discriminant {
            let value = LitStr::new(
                expr.to_token_stream().to_string().as_str(),
                proc_macro2::Span::call_site(),
            );

            from_str_arms.push(quote! {
                #value => Ok(#assigner),
            });
        }

        from_str_arms.push(quote! {
            #en_str | #lower_name => Ok(#assigner),
        });

        #[cfg(feature = "ko")]
        {
            ko_arms.push(quote! {
                #arm_name => #ko_str,
            });
            from_str_arms.push(quote! {
                #ko_str => Ok(#assigner),
            });
        }
    }

    #[cfg(feature = "ko")]
    let ko_arm = quote! {
        dioxus_translate::Language::Ko => match self {
            #(#ko_arms)*
        },
    };
    #[cfg(not(feature = "ko"))]
    let ko_arm = quote! {};

    // Generate the implementation block for `translate`
    let r#gen = quote! {
        impl #enum_name {
            pub fn translate(&self, lang: &dioxus_translate::Language) -> &'static str {
                match lang {
                    dioxus_translate::Language::En => match self {
                        #(#en_arms)*
                    },
                    #ko_arm
                }
            }

            pub const VARIANTS: &'static [Self] = &[ #(#enum_name::#unit_variants,)* ];
            pub fn variants(lang: &dioxus_translate::Language) -> Vec<String> {
                Self::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect::<Vec<_>>()
            }
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }


        impl std::str::FromStr for #enum_name {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    #(#from_str_arms)*
                    _ => Err(format!("invalid field")),
                }
            }
        }
    };

    r#gen.into()
}
