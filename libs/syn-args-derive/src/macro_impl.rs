use proc_macro::TokenStream;
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

            for field in &variant.fields {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_value = quote! { otr(args.first())?.try_into()? };

                variant_fields.push(quote! { #field_name: #field_type });
                variant_fields_name.push(field_name);
                variant_fields_type.push(field_type);
                variant_fields_value.push(field_value);
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

            match_arm.push(quote! { #field_name: #field_value });
        }

        match_arms.push(quote! {
            #name::#variant_name(#(#match_arm),*) => {
                let r: Result<#name, anyhow::Error> = ewc(|| Ok(#name::#variant_name(#(#match_arm),*)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }
            }
        });
    }

    let expanded = quote! {
        impl #impl_generics ArgsParseImpl for #name #ty_generics #where_clause {
            fn parse(args: Vec<Value>) -> Result<Self, Error> {
                #(#match_arms)*

                Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
            }
          }
    };

    expanded.into()
}
