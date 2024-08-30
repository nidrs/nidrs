use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_args_parse(args: &DeriveInput) -> TokenStream {
    let name = &args.ident;
    let (impl_generics, ty_generics, where_clause) = args.generics.split_for_impl();

    let mut fields_name = vec![];
    let mut fields_type = vec![];
    let mut fields_value = vec![];
    let mut core_expand = quote! {};
    let mut arguments_expand = quote! {};

    if let syn::Data::Enum(data) = &args.data {
        for variant in &data.variants {
            let variant_name = &variant.ident;
            let mut variant_fields_name = vec![];
            let mut variant_fields_type = vec![];
            let mut variant_fields_value = vec![];
            let mut i: usize = 0;
            for field in &variant.fields {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let i_str = i.to_string();
                let field_value = quote! { syn_args::Transform::new(v, #i_str).try_into()? };

                variant_fields_name.push(field_name);
                variant_fields_type.push(field_type);
                variant_fields_value.push(field_value);
                i += 1;
            }

            fields_name.push(variant_name);
            fields_type.push(variant_fields_type);
            fields_value.push(variant_fields_value);
        }
        let mut match_arms = vec![];
        for (i, variant_name) in fields_name.iter().enumerate() {
            let variant_fields_value = &fields_value[i];

            let mut match_arm = vec![];
            for (j, field_value) in variant_fields_value.iter().enumerate() {
                match_arm.push(quote! {
                    #field_value
                });
            }

            match_arms.push(quote! {
                if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(#name::#variant_name(#(#match_arm),*))) {
                    return Ok(rt);
                }
            });
        }
        core_expand = quote! {
            if let syn_args::Value::Array(_) = v {
                #(#match_arms)*
            }
        };
        arguments_expand = quote! {
            Self::try_from(&value.0)
        };
    } else if let syn::Data::Struct(data) = &args.data {
        let mut variant_fields_name = vec![];
        let mut variant_fields_type = vec![];
        let mut variant_fields_value = vec![];

        for field in &data.fields {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_type = &field.ty;
            let field_value = quote! { #field_ident:  syn_args::Transform::new(v, #field_name).try_into()? };

            variant_fields_name.push(field_name);
            variant_fields_type.push(field_type);
            variant_fields_value.push(field_value);
        }

        core_expand = quote! {
            if let syn_args::Value::Object(_) = v {
                return Ok(
                    #name {
                        #(#variant_fields_value),*
                    }
                );
            }
        };
        arguments_expand = quote! {
            if let syn_args::Value::Array(v) = value.0 {
                if let Some(value) = v.first() {
                    return Self::try_from(value);
                }
            }
            Err(Self::Error::new(proc_macro2::Span::call_site(), "Arguments Into T"))
        };
    }

    let expanded = quote! {
        impl #impl_generics TryFrom<&syn_args::Value> for #name #ty_generics #where_clause {
            type Error = syn::Error;
            fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
                #core_expand
                Err(Self::Error::new(proc_macro2::Span::call_site(), format!("Invalid args try_from {}", stringify!(#name))))
            }
        }
        impl #impl_generics TryFrom<syn_args::Value> for #name #ty_generics #where_clause {
            type Error = syn::Error;
            fn try_from(v: syn_args::Value) -> Result<Self, Self::Error> {
                #name::try_from(&v)
            }
        }
        impl #impl_generics syn_args::ArgsParse for #name #ty_generics #where_clause {
            fn parse(input: &str) -> Result<Self, syn::Error> {
                 syn_args::Formal::new().parse(input)?.try_into()
            }
        }
        impl #impl_generics TryFrom<syn_args::Transform<'_>> for #name #ty_generics #where_clause  {
            type Error = syn::Error;

            fn try_from(value: syn_args::Transform) -> Result<Self, Self::Error> {
                if let syn_args::Value::Object(obj) = value.value {
                    if let Some(v) = obj.get(value.key) {
                        return v.try_into();
                    }
                } else if let syn_args::Value::Array(v) = value.value {
                    let index = value.key.parse::<usize>().unwrap();
                    if let Some(value) = v.get(index) {
                        return Self::try_from(value);
                    }
                }

                Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected Transform value"))
            }
        }
        impl #impl_generics TryFrom<syn_args::Arguments> for #name #ty_generics #where_clause  {
            type Error = syn::Error;

            fn try_from(value: syn_args::Arguments) -> Result<Self, Self::Error> {
                #arguments_expand
            }
        }
    };

    expanded
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_impl_args_parse() {
        let input = syn::parse_quote! {
            enum ModuleArgs {
                F1(def::Int, def::Int),
                F2(def::Int),
                F3(def::Ident),
                F4(def::Array<def::Ident>),
                F5(ModuleSubObj),
                F6(def::Array<ModuleSubObj>),
            }
        };

        let result = impl_args_parse(&input);

        assert_snapshot!(result.to_string());
    }
}
