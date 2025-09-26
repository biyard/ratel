use quote::quote;

pub fn build_order_by_functions(field_name: &str) -> proc_macro2::TokenStream {
    let field_id_str = syn::LitStr::new(field_name, proc_macro2::Span::call_site());
    let asc_fn = syn::Ident::new(
        &format!("order_by_{}_asc", field_name),
        proc_macro2::Span::call_site(),
    );
    let desc_fn = syn::Ident::new(
        &format!("order_by_{}_desc", field_name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        pub fn #asc_fn(mut self) -> Self {
            if let by_types::Order::Asc(ref mut field) = self.order {
                field.push(format!(",{}", #field_id_str));
            } else {
                self.order = by_types::Order::Asc(vec![#field_id_str.to_string()]);
            }
            self
        }

        pub fn #desc_fn(mut self) -> Self {
            if let by_types::Order::Desc(ref mut field) = self.order {
                field.push(format!(",{}", #field_id_str));
            } else {
                self.order = by_types::Order::Desc(vec![#field_id_str.to_string()]);
            }
            self
        }
    }
}

pub fn build_string_query_functions(field_name: &str, ty: &str) -> proc_macro2::TokenStream {
    let ty: proc_macro2::TokenStream = ty.parse().unwrap();
    let n: proc_macro2::TokenStream = field_name.parse().unwrap();
    let field_id_str = syn::LitStr::new(field_name, proc_macro2::Span::call_site());

    let eq_fn = syn::Ident::new(
        &format!("{}_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let neq_fn = syn::Ident::new(
        &format!("{}_not_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let contains_fn = syn::Ident::new(
        &format!("{}_contains", field_name),
        proc_macro2::Span::call_site(),
    );

    let not_contains_fn = syn::Ident::new(
        &format!("{}_not_contains", field_name),
        proc_macro2::Span::call_site(),
    );

    let starts_with_fn = syn::Ident::new(
        &format!("{}_starts_with", field_name),
        proc_macro2::Span::call_site(),
    );

    let not_starts_with_fn = syn::Ident::new(
        &format!("{}_not_starts_with", field_name),
        proc_macro2::Span::call_site(),
    );

    let ends_with_fn = syn::Ident::new(
        &format!("{}_ends_with", field_name),
        proc_macro2::Span::call_site(),
    );

    let not_ends_with_fn = syn::Ident::new(
        &format!("{}_not_ends_with", field_name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        pub fn #eq_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::EqualsText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #neq_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::NotEqualsText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #contains_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::ContainsText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #not_contains_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::NotContainsText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #starts_with_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::StartsWithText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #not_starts_with_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::NotStartsWithText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #ends_with_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::EndsWithText(#field_id_str.to_string(),#n));
            self
        }

        pub fn #not_ends_with_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::NotEndsWithText(#field_id_str.to_string(),#n));
            self
        }
    }
}

pub fn build_bigint_query_functions(field_name: &str, ty_str: &str) -> proc_macro2::TokenStream {
    let ty: proc_macro2::TokenStream = ty_str.parse().unwrap();
    let n: proc_macro2::TokenStream = field_name.parse().unwrap();
    let field_id_str = syn::LitStr::new(field_name, proc_macro2::Span::call_site());
    let bridge = if ty_str == "u64" {
        quote! { as i64 }
    } else {
        quote! {}
    };

    let eq_fn = syn::Ident::new(
        &format!("{}_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let neq_fn = syn::Ident::new(
        &format!("{}_not_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let gt_fn = syn::Ident::new(
        &format!("{}_greater_than", field_name),
        proc_macro2::Span::call_site(),
    );

    let lt_fn = syn::Ident::new(
        &format!("{}_less_than", field_name),
        proc_macro2::Span::call_site(),
    );

    let gte_fn = syn::Ident::new(
        &format!("{}_greater_than_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let lte_fn = syn::Ident::new(
        &format!("{}_less_than_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let between_fn = syn::Ident::new(
        &format!("{}_between", field_name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        pub fn #eq_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::EqualsBigint(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #neq_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::NotEqualsBigint(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #gt_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::GreaterThanBigint(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #lt_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::LessThanBigint(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #gte_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::GreaterThanEqualsBigint(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #lte_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::LessThanEqualsBigint(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #between_fn(mut self, from: #ty, to: #ty) -> Self {
            self.conditions.push(by_types::Conditions::BetweenBigint(#field_id_str.to_string(),from #bridge,to #bridge));
            self
        }
    }
}

pub fn build_integer_query_functions(field_name: &str, ty_str: &str) -> proc_macro2::TokenStream {
    let ty: proc_macro2::TokenStream = ty_str.parse().unwrap();
    let n: proc_macro2::TokenStream = field_name.parse().unwrap();
    let field_id_str = syn::LitStr::new(field_name, proc_macro2::Span::call_site());
    let bridge = if ty_str == "u32" {
        quote! { as i32 }
    } else if ty_str != "i32" {
        quote! {
           .into()
        }
    } else {
        quote! {}
    };

    let eq_fn = syn::Ident::new(
        &format!("{}_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let neq_fn = syn::Ident::new(
        &format!("{}_not_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let gt_fn = syn::Ident::new(
        &format!("{}_greater_than", field_name),
        proc_macro2::Span::call_site(),
    );

    let lt_fn = syn::Ident::new(
        &format!("{}_less_than", field_name),
        proc_macro2::Span::call_site(),
    );

    let gte_fn = syn::Ident::new(
        &format!("{}_greater_than_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let lte_fn = syn::Ident::new(
        &format!("{}_less_than_equals", field_name),
        proc_macro2::Span::call_site(),
    );

    let between_fn = syn::Ident::new(
        &format!("{}_between", field_name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        pub fn #eq_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::EqualsInteger(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #neq_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::NotEqualsInteger(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #gt_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::GreaterThanInteger(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #lt_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::LessThanInteger(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #gte_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::GreaterThanEqualsInteger(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #lte_fn(mut self, #n: #ty) -> Self {
            self.conditions.push(by_types::Conditions::LessThanEqualsInteger(#field_id_str.to_string(),#n #bridge));
            self
        }

        pub fn #between_fn(mut self, from: #ty, to: #ty) -> Self {
            self.conditions.push(by_types::Conditions::BetweenInteger(#field_id_str.to_string(),from #bridge,to #bridge));
            self
        }
    }
}

pub fn build_boolean_query_functions(field_name: &str) -> proc_macro2::TokenStream {
    let field_id_str = syn::LitStr::new(field_name, proc_macro2::Span::call_site());

    let eq_fn = syn::Ident::new(
        &format!("{}_is_true", field_name),
        proc_macro2::Span::call_site(),
    );

    let neq_fn = syn::Ident::new(
        &format!("{}_is_false", field_name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        pub fn #eq_fn(mut self) -> Self {
            self.conditions.push(by_types::Conditions::TrueBoolean(#field_id_str.to_string()));
            self
        }

        pub fn #neq_fn(mut self) -> Self {
            self.conditions.push(by_types::Conditions::FalseBoolean(#field_id_str.to_string()));
            self
        }
    }
}
