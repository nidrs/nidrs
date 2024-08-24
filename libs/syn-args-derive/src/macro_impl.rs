use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_args_parse(args: &DeriveInput) -> TokenStream {
    let name = &args.ident;
    let (impl_generics, ty_generics, where_clause) = args.generics.split_for_impl();

    let mut fields = vec![];
    let mut fields_name = vec![];
    let mut fields_type = vec![];
    let mut fields_value = vec![];

    if let syn::Data::Enum(data) = &args.data {
        for variant in &data.variants {
            let variant_name = &variant.ident;
            let mut variant_fields = vec![];
            let mut variant_fields_name = vec![];
            let mut variant_fields_type = vec![];
            let mut variant_fields_value = vec![];
            let mut i: usize = 0;
            for field in &variant.fields {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_value = quote! { otr(args.get(#i))?.try_into()? };

                variant_fields.push(quote! { #field_name: #field_type });
                variant_fields_name.push(field_name);
                variant_fields_type.push(field_type);
                variant_fields_value.push(field_value);
                i += 1;
            }

            fields.push(quote! { #variant_name(#(#variant_fields),*) });
            fields_name.push(variant_name);
            fields_type.push(variant_fields_type);
            fields_value.push(variant_fields_value);
        }
    }

    let mut match_arms = vec![];
    for (i, field) in fields.iter().enumerate() {
        let variant_name = &fields_name[i];
        let variant_fields = &fields_type[i];
        let variant_fields_value = &fields_value[i];

        let mut match_arm = vec![];
        for (j, field) in variant_fields.iter().enumerate() {
            let field_name = &variant_fields[j];
            let field_value = &variant_fields_value[j];

            match_arm.push(quote! {
                #field_value
            });
        }

        match_arms.push(quote! {
            if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(#name::#variant_name(#(#match_arm),*))) {
                return Ok(rt);
            }
        });
    }

    let expanded = quote! {
        impl #impl_generics TryFrom<&syn_args::macro_args::Value> for #name #ty_generics #where_clause {
            type Error = Error;
            fn try_from(v: &Value) -> Result<Self, Error> {
                if let Value::Array(args) = v {
                    #(#match_arms)*
                }
                Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
            }
        }
        impl #impl_generics TryFrom<syn_args::macro_args::Value> for #name #ty_generics #where_clause {
            type Error = Error;
            fn try_from(v: Value) -> Result<Self, Error> {
                #name::try_from(&v)
            }
        }
        impl #impl_generics syn_args::traits::ArgsParse for #name #ty_generics #where_clause {
            fn parse(input: &str) -> Result<Self, Error> {
                 syn_args::macro_args::Formal::new().parse(input)?.try_into()
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
        impl syn_args::traits::ArgsParse for ModuleArgs {
            fn parse(args: Vec<Value>) -> Result<Self, Error> {
                if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F1(otr(args.get(0usize))?.try_into()?, otr(args.get(1usize))?.try_into()?))) {
                    return Ok(rt);
                }
                if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F2(otr(args.get(0usize))?.try_into()?))) {
                    return Ok(rt);
                }
                if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F3(otr(args.get(0usize))?.try_into()?))) {
                    return Ok(rt);
                }
                if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F4(otr(args.get(0usize))?.try_into()?))) {
                    return Ok(rt);
                }
                if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F5(otr(args.get(0usize))?.try_into()?))) {
                    return Ok(rt);
                }
                if let Ok(rt) = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F6(otr(args.get(0usize))?.try_into()?))) {
                    return Ok(rt);
                }
                Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
            }
        }
                };

        let result = impl_args_parse(&input);
        assert_eq!(result.to_string(), expected.to_string());
    }
}
