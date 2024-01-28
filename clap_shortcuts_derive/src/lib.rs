#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]
use core::panic;
use darling::{FromAttributes, FromMeta};
use heck::AsUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use quote_tool_params::{get_from_params, prepare_values_from_params};
use syn::DeriveInput;
#[derive(FromMeta)]
struct Shortcut {
    name: String,
    func: String,
}

#[derive(FromAttributes)]
#[darling(attributes(shortcut))]
struct ShortCutsOpts {
    params: Option<String>,
    #[darling(multiple)]
    values: Vec<Shortcut>,
}
#[doc = include_str!("../doc/attributs_derive.md")]
#[proc_macro_derive(ShortCuts, attributes(shortcut))]
pub fn promptable_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // get information on struct
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get attributs given
    let attrs_struct =
        ShortCutsOpts::from_attributes(&ast.attrs).expect("Wrong attributes on struct");
    if attrs_struct.values.is_empty() {
        panic!("at least one attribut #[shortcut(...)] must be present");
    }
    // get name of struct
    let name_struct = ast.ident;
    // params as type inside tuple
    let types_params: TokenStream = if let Some(p) = &attrs_struct.params {
        get_from_params(p, false)
    } else {
        String::new()
    }
    .parse()
    .unwrap();

    let prepare_values = if let Some(p) = &attrs_struct.params {
        prepare_values_from_params(p, "params")
    } else {
        vec![]
    };

    let shortcuts_mut = attrs_struct
        .values
        .iter()
        .filter(|s| s.func.contains("&mut self"))
        .collect::<Vec<&Shortcut>>();
    let variants_mut = get_variants(&shortcuts_mut);
    let lines_match_mut = get_match_lines(&shortcuts_mut, &variants_mut);

    let shortcuts_ref = attrs_struct
        .values
        .iter()
        .filter(|s| s.func.contains("&self"))
        .collect::<Vec<&Shortcut>>();
    let variants_ref = get_variants(&shortcuts_ref);
    let lines_match_ref = get_match_lines(&shortcuts_ref, &variants_ref);

    let shortcuts_once = attrs_struct
        .values
        .iter()
        .filter(|s| !(s.func.contains("&self") || s.func.contains("&mut self")))
        .collect::<Vec<&Shortcut>>();
    let variants_once = get_variants(&shortcuts_once);
    let lines_match_once = get_match_lines(&shortcuts_once, &variants_once);

    let content_mut = if !shortcuts_mut.is_empty() {
        quote! {
                #( #prepare_values)*
                match &shortcut {
                    #( #lines_match_mut,)*
                };
        }
    } else {
        quote! {}
    };
    let content_ref = if !shortcuts_ref.is_empty() {
        quote! {
                #( #prepare_values)*
                match &shortcut {
                    #( #lines_match_ref,)*
                };
        }
    } else {
        quote! {}
    };
    let content_once = if !shortcuts_once.is_empty() {
        quote! {
                #( #prepare_values)*
                match &shortcut {
                    #( #lines_match_once,)*
                };
        }
    } else {
        quote! {}
    };

    let trait_impl = quote! {
            impl clap_shortcuts::ShortCuts<(#types_params)> for #name_struct {
      fn shortcut_mut(&mut self, shortcut: &impl clap::ValueEnum, params: (#types_params)) -> anyhow::Result<()> {
                #content_mut
                Ok(())
            }
      fn shortcut_ref(&self, shortcut: &impl clap::ValueEnum, params: (#types_params)) -> anyhow::Result<()> {
                #content_ref
                Ok(())
            }
      fn shortcut_owned(self, shortcut: &impl clap::ValueEnum, params: (#types_params)) -> anyhow::Result<()> {
                #content_once
                Ok(())
            }
        }
    };
    let name_enum_ref: TokenStream = format!("ShortCuts{}Ref", name_struct).parse().unwrap();
    let name_enum_mut: TokenStream = format!("ShortCuts{}Mut", name_struct).parse().unwrap();
    let name_enum_once: TokenStream = format!("ShortCuts{}Once", name_struct).parse().unwrap();
    let enum_ref = if !shortcuts_ref.is_empty() {
        quote! {
        #[derive(clap::ValueEnum, Clone)]
        pub enum #name_enum_ref {
            #( #variants_ref),*
        }
            }
    } else {
        quote! {}
    };
    let enum_mut = if !shortcuts_mut.is_empty() {
        quote! {
        #[derive(clap::ValueEnum, Clone)]
        pub enum #name_enum_mut {
            #( #variants_mut),*
        }
            }
    } else {
        quote! {}
    };
    let enum_once = if !shortcuts_once.is_empty() {
        quote! {
        #[derive(clap::ValueEnum, Clone)]
        pub enum #name_enum_once {
            #( #variants_once),*
        }
            }
    } else {
        quote! {}
    };
    let enum_args = quote! {
        #enum_ref
        #enum_mut
        #enum_once
    };
    let name_arg: TokenStream = format!("ShortCutArg{}", name_struct).parse().unwrap();
    let arg_ref = if !shortcuts_ref.is_empty() {
        quote! {
        #[clap(long, value_enum)]
        pub shortcut_ref: Option<#name_enum_ref>,
        }
    } else {
        quote!()
    };
    let arg_mut = if !shortcuts_mut.is_empty() {
        quote! {
        #[clap(long, value_enum)]
        pub shortcut_mut: Option<#name_enum_mut>,
        }
    } else {
        quote!()
    };
    let arg_once = if !shortcuts_once.is_empty() {
        quote! {
        #[clap(long, value_enum)]
        pub shortcut_once: Option<#name_enum_once>,
        }
    } else {
        quote!()
    };
    let mut some = 0;
    if !shortcuts_ref.is_empty() {
        some += 1
    }
    if !shortcuts_mut.is_empty() {
        some += 1
    }
    if !shortcuts_once.is_empty() {
        some += 1
    }
    let multiple = if some > 1 {
        quote! { multiple = false}
    } else {
        quote! {}
    };
    let arg_group = quote! {
    #[derive(clap::Args)]
    #[group(required = true, #multiple)]
    pub struct #name_arg {
            #arg_ref
            #arg_mut
            #arg_once
    }
        };
    quote! {
        #trait_impl
        #enum_args
        #arg_group
    }
    .into()
}

fn get_variants(shortcuts: &[&Shortcut]) -> Vec<TokenStream> {
    shortcuts
        .iter()
        .map(|s| format!("{}", AsUpperCamelCase(&s.name)).parse().unwrap())
        .collect::<Vec<TokenStream>>()
}
fn get_match_lines(shortcuts: &[&Shortcut], variants: &[TokenStream]) -> Vec<TokenStream> {
    shortcuts
        .iter()
        .enumerate()
        .map(|(index, s)| {
            // let func = s.func.replace("&mut self", "self").replace("&self", "self");
            format!("&{} => {}", variants[index], s.func)
                .parse()
                .unwrap()
        })
        .collect::<Vec<TokenStream>>()
}
