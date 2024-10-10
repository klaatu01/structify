use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::{parse::Parser, parse_macro_input, ItemFn};

fn is_dependency_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Dep";
        }
    }
    false
}

fn get_struct_name_ident(attr: &TokenStream, input_fn: &syn::ItemFn) -> syn::Ident {
    let fn_name_str = input_fn.sig.ident.to_string();

    let default_struct_name = fn_name_str.to_case(Case::Pascal).to_string();

    if let Ok(args) = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
        .parse(attr.clone())
    {
        if args.is_empty() {
            return syn::Ident::new(&default_struct_name, proc_macro2::Span::call_site());
        }
        if args.len() > 1 {
            panic!("Expected exactly one for struct name #[structify(StructName)]");
        }
        syn::Ident::new(
            &args[0].get_ident().unwrap().to_string(),
            proc_macro2::Span::call_site(),
        )
    } else {
        panic!("expected only a punctuated path");
    }
}

#[proc_macro_attribute]
pub fn structify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let struct_name_ident = get_struct_name_ident(&_attr, &input_fn);

    let mut fields = Vec::new();
    let mut execute_bindings = Vec::new();
    let mut new_struct_fields = Vec::new();
    let mut execute_args = vec![quote::quote! {&self}];

    let asyncness = match input_fn.sig.asyncness {
        Some(_) => quote::quote! {async},
        None => quote::quote! {},
    };

    for arg in input_fn.sig.inputs.iter() {
        match arg {
            syn::FnArg::Typed(pat_type) => {
                let pat = pat_type.pat.clone();
                let ty = pat_type.ty.clone();
                if is_dependency_type(&pat_type.ty) {
                    execute_args.push(quote::quote! { #pat: impl Into<#ty> });
                    execute_bindings.push(quote::quote! {
                        let #pat = #pat.into();
                    });
                } else {
                    fields.push(quote::quote! {
                        #pat: #ty
                    });
                    execute_bindings.push(quote::quote! {
                        let #pat = &self.#pat;
                    });
                    new_struct_fields.push(quote::quote! {
                        #pat: #pat
                    });
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("Receiver arguments are not supported");
            }
        }
    }

    let block = input_fn.block.clone();

    let ret_ty = match &input_fn.sig.output {
        syn::ReturnType::Default => quote::quote! { () },
        syn::ReturnType::Type(_, ty) => quote::quote! { #ty },
    };

    quote::quote! {
        #[allow(unused)]
        #input_fn

        pub struct #struct_name_ident {
            #(#fields),*
        }

        impl #struct_name_ident {
            pub fn new(#(#fields),*) -> Self {
                Self {
                    #(#new_struct_fields),*
                }
            }
            pub #asyncness fn execute(#(#execute_args),*) -> #ret_ty {
                #(#execute_bindings)*
                #block
            }
        }
    }
    .into()
}
