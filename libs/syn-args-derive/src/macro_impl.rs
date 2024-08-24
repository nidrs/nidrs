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
                let field_value = quote! { syn_args::utils::otr(v.get(#i))?.try_into()? };

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
                if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(#name::#variant_name(#(#match_arm),*))) {
                    return Ok(rt);
                }
            });
        }
        core_expand = quote! {
            if let syn_args::Value::Array(v) = v {
                #(#match_arms)*
            }
        };
    } else if let syn::Data::Struct(data) = &args.data {
        // if let syn_args::Value::Object(v) = value {
        //     return Ok(
        //         ModuleSubObj {
        //             imports: v.get("imports").ok_or(Error::new(proc_macro2::Span::call_site(), "Expected imports"))?.try_into()?,
        //         }
        //     );
        //  }

        let mut variant_fields_name = vec![];
        let mut variant_fields_type = vec![];
        let mut variant_fields_value = vec![];

        for field in &data.fields {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_type = &field.ty;
            let field_value = quote! { #field_ident: syn_args::utils::otr(v.get(#field_name))?.try_into()? };

            variant_fields_name.push(field_name);
            variant_fields_type.push(field_type);
            variant_fields_value.push(field_value);
        }

        core_expand = quote! {
            if let syn_args::Value::Object(v) = v {
                return Ok(
                    #name {
                        #(#variant_fields_value),*
                    }
                );
            }
        };
    }

    let expanded = quote! {
        impl #impl_generics TryFrom<&syn_args::Value> for #name #ty_generics #where_clause {
            type Error = Error;
            fn try_from(v: &syn_args::Value) -> Result<Self, Error> {
                #core_expand
                Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
            }
        }
        impl #impl_generics TryFrom<syn_args::Value> for #name #ty_generics #where_clause {
            type Error = Error;
            fn try_from(v: syn_args::Value) -> Result<Self, Error> {
                #name::try_from(&v)
            }
        }
        impl #impl_generics syn_args::ArgsParse for #name #ty_generics #where_clause {
            fn parse(input: &str) -> Result<Self, Error> {
                 syn_args::Formal::new().parse(input)?.try_into()
            }
        }
    };

    expanded
}

#[cfg(test)]
mod tests {
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

        let expected = quote! {
            impl TryFrom<&syn_args::Value> for ModuleArgs {
                type Error = Error;
                fn try_from(v: &syn_args::Value) -> Result<Self, Error> {
                    if let syn_args::Value::Array(v) = v {
                        if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F1(syn_args::utils::otr(v.get(0usize))?.try_into()?, syn_args::utils::otr(v.get(1usize))?.try_into()?))) {
                            return Ok(rt);
                        }
                        if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F2(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                            return Ok(rt);
                        }
                        if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F3(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                            return Ok(rt);
                        }
                        if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F4(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                            return Ok(rt);
                        }
                        if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F5(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                            return Ok(rt);
                        }
                        if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F6(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                            return Ok(rt);
                        }
                    }
                    Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
                }
            }
            impl TryFrom<syn_args::Value> for ModuleArgs {
                type Error = Error;
                fn try_from(v: syn_args::Value) -> Result<Self, Error> {
                    ModuleArgs::try_from(&v)
                }
            }
            impl syn_args::ArgsParse for ModuleArgs {
                fn parse(input: &str) -> Result<Self, Error> {
                    syn_args::Formal::new().parse(input)?.try_into()
                }
            }
        };

        let result = impl_args_parse(&input);
        assert_eq!(result.to_string(), expected.to_string());
    }
}
