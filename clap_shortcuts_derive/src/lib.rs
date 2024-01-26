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
    let lines_match_mut = get_match_lines(&shortcuts_mut);

    let shortcuts_ref = attrs_struct
        .values
        .iter()
        .filter(|s| s.func.contains("&self"))
        .collect::<Vec<&Shortcut>>();
    let lines_match_ref = get_match_lines(&shortcuts_ref);

    let shortcuts_once = attrs_struct
        .values
        .iter()
        .filter(|s| !(s.func.contains("&self") || s.func.contains("&mut self")))
        .collect::<Vec<&Shortcut>>();

    let lines_match_once = get_match_lines(&shortcuts_once);

    let ok: TokenStream = "Ok(())".parse().unwrap();
    let ok_ref = if !lines_match_ref.is_empty() {
        quote! {#ok}
    } else {
        quote! {}
    };
    let ok_mut = if !lines_match_mut.is_empty() {
        quote! {#ok}
    } else {
        quote! {}
    };
    let ok_once = if !lines_match_once.is_empty() {
        quote! {#ok}
    } else {
        quote! {}
    };
    let trait_impl = quote! {
            impl clap_shortcuts::ShortCuts<(#types_params)> for #name_struct {
      fn shortcut_mut(&mut self, shortcut: &impl clap::ValueEnum, params: (#types_params)) -> anyhow::Result<()> {
                #( #prepare_values)*
                match &shortcut {
                    #( #lines_match_mut,)*
                    _ => anyhow::bail!("This shortcut variant is not mutable, use another method of the trait Shortcut")
                };
                #ok_mut
            }
      fn shortcut_ref(&self, shortcut: &impl clap::ValueEnum, params: (#types_params)) -> anyhow::Result<()> {
                #( #prepare_values)*
                match &shortcut {
                    #( #lines_match_ref,)*
                    _ => anyhow::bail!("This shortcut variant is not mutable, use another method of the trait Shortcut")
                }
                #ok_ref
            }
      fn shortcut_owned(self, shortcut: &impl clap::ValueEnum, params: (#types_params)) -> anyhow::Result<()> {
                #( #prepare_values)*
                match &shortcut {
                    #( #lines_match_once,)*
                    _ => anyhow::bail!("This shortcut variant is not mutable, use another method of the trait Shortcut")
                }
                #ok_once
            }
        }
    };
    let name_enum: TokenStream = format!("ShortCuts{}", name_struct).parse().unwrap();
    let variants = get_variants(&attrs_struct.values.iter().collect::<Vec<&Shortcut>>());
    let enum_args = quote! {
        #[derive(clap::ValueEnum, Clone)]
        enum #name_enum {
            #( #variants),*
        }
    };
    quote! {
        #trait_impl
        #enum_args
    }
    .into()
}

fn get_variants(shortcuts: &[&Shortcut]) -> Vec<TokenStream> {
    shortcuts
        .iter()
        .map(|s| format!("{}", AsUpperCamelCase(&s.name)).parse().unwrap())
        .collect::<Vec<TokenStream>>()
}
fn get_match_lines(shortcuts: &[&Shortcut]) -> Vec<TokenStream> {
    let variants = get_variants(shortcuts);
    shortcuts
        .iter()
        .enumerate()
        .map(|(index, s)| {
            let func = s.func.replace("&mut self", "self").replace("&self", "self");
            format!("&{} => {}", variants[index], func).parse().unwrap()
        })
        .collect::<Vec<TokenStream>>()
}
