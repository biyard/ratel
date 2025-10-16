use proc_macro2::Span;
use quote::*;
use syn::*;

#[derive(Default, Clone, Debug)]
pub struct DynamoIndexKey {
    pub prefix: Option<String>,
    pub fields: Vec<(Ident, Type, i32)>, // (field_name, field_type, order)
}

#[derive(Default, Clone, Debug)]
pub struct DynamoIndex {
    pub name: String,
    #[allow(dead_code)]
    pub base_index_name: String,
    pub pk: DynamoIndexKey,
    pub sk: Option<DynamoIndexKey>,
}

impl DynamoIndex {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let mut out = vec![];
        // let real_index_name = format!("{}-index", self.base_index_name);
        // let ident_pk_field =
        //     syn::Ident::new(&format!("{}_pk", self.base_index_name), Span::call_site());
        // let ident_sk_field =
        // syn::Ident::new(&format!("{}_sk", self.base_index_name), Span::call_site());

        out.push(self.generate_key_generation(&self.name, "pk", &self.pk));

        if let Some(sk) = &self.sk {
            out.push(self.generate_key_generation(&self.name, "sk", sk));
        }

        quote! {
            #(#out)*
        }
    }

    pub fn get_additional_fields(&self) -> proc_macro2::TokenStream {
        let pk_key_name = format!("{}_pk", self.base_index_name);
        let sk_key_name = format!("{}_sk", self.base_index_name);
        let get_pk_fn_name =
            syn::Ident::new(&format!("get_pk_for_{}", self.name), Span::call_site());
        let get_sk_fn_name =
            syn::Ident::new(&format!("get_sk_for_{}", self.name), Span::call_site());

        let mut out = quote! {
            let value = self.#get_pk_fn_name();
            if !value.is_empty() {
                item.insert(
                    #pk_key_name.to_string(),
                    aws_sdk_dynamodb::types::AttributeValue::S(value),
                );
            }
        };

        if self.sk.is_some() {
            out = quote! {
                #out
                let value = self.#get_sk_fn_name();

                if !value.is_empty() {
                    item.insert(
                        #sk_key_name.to_string(),
                        aws_sdk_dynamodb::types::AttributeValue::S(value),
                    );
                }
            };
        }

        out
    }

    pub fn generate_key_generation(
        &self,
        name: &str,
        scheme: &str,
        key: &DynamoIndexKey,
    ) -> proc_macro2::TokenStream {
        let fn_name = syn::Ident::new(
            &format!("generate_{}_for_{}", scheme, name),
            Span::call_site(),
        );

        let get_fn_name =
            syn::Ident::new(&format!("get_{}_for_{}", scheme, name), Span::call_site());

        let get_idx_fn_name = syn::Ident::new(
            &format!("get_{}_for_{}", scheme, self.base_index_name),
            Span::call_site(),
        );

        let mut args = vec![];
        let mut arg_names = vec![];
        let mut get_arg_formatter = vec![];

        for (field_name, field_type, _) in key.fields.iter() {
            let ft = self.inner_type(field_type);
            let error = format!(
                "Field {} is associated by {}-index",
                field_name, self.base_index_name
            );

            args.push(quote! { #field_name: #ft });
            arg_names.push(field_name);
            get_arg_formatter.push(if self.is_option(field_type) {
                quote! {
                    if let Some(v) = &self.#field_name {
                        v.to_string()
                    } else {
                        tracing::debug!(#error);
                        "".to_string()
                    }
                }
            } else {
                quote! { self.#field_name.to_string() }
            });
        }

        let out = if let Some(prefix) = &key.prefix {
            quote! {
                pub fn #fn_name(#(#args),*) -> String {
                    vec![#prefix.to_string(), #(#arg_names.to_string()),*].join("#")
                }

                pub fn #get_fn_name(&self) -> String {
                    vec![#prefix.to_string(), #(#get_arg_formatter),*].join("#")
                }

                pub fn #get_idx_fn_name(&self) -> String {
                    vec![#prefix.to_string(), #(#get_arg_formatter),*].join("#")
                }
            }
        } else {
            quote! {
                pub fn #fn_name(#(#args),*) -> String {
                    vec![#(#arg_names.to_string()),*].join("#")
                }

                pub fn #get_fn_name(&self) -> String {
                    vec![#(#get_arg_formatter),*].join("#")
                }

                pub fn #get_idx_fn_name(&self) -> String {
                    vec![#(#get_arg_formatter),*].join("#")
                }
            }
        };

        out
    }

    pub fn is_option(&self, ty: &Type) -> bool {
        use syn::{Type, TypePath};
        match ty {
            Type::Path(TypePath { path, .. }) => path
                .segments
                .last()
                .map(|seg| seg.ident == "Option")
                .unwrap_or(false),
            _ => false,
        }
    }

    pub fn inner_type(&self, ty: &Type) -> Type {
        if let Type::Path(TypePath { path, .. }) = ty {
            if let Some(seg) = path.segments.last() {
                if seg.ident == "Option" {
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                return inner_ty.clone();
                            }
                        }
                    }
                }
            }
        }
        ty.clone()
    }
}
