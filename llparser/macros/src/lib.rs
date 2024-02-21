use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, LitStr, Token};

#[derive(Debug)]
struct Line {
    left: LitStr,
    right: LitStr,
}

impl Parse for Line {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let left = input.parse()?;
        let _: Token![->] = input.parse()?;
        let right = input.parse()?;
        Ok(Line { left, right })
    }
}

#[derive(Debug)]
struct MacroInput {
    lines: Punctuated<Line, Token![;]>,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MacroInput {
            lines: input.parse_terminated(Line::parse, Token![;])?,
        })
    }
}

#[proc_macro]
pub fn grammar(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as MacroInput);
    let mut rules: HashMap<LitStr, Vec<LitStr>> = HashMap::new();
    for line in input.lines {
        rules.entry(line.left).or_default().push(line.right);
    }

    let rule_names: Vec<_> = rules.keys().map(LitStr::value).collect();
    let entries: Vec<_> = rules
        .iter()
        .map(|(name, rights)| {
            let rights: Vec<_> = rights
                .iter()
                .map(|right| {
                    let toks: Vec<_> = right
                        .value()
                        .split_whitespace()
                        .map(|v| {
                            if rule_names.contains(&v.into()) {
                                quote! { Symbol::NonTerminal(#v.into()) }
                            } else {
                                quote! { Symbol::Terminal(#v.into()) }
                            }
                        })
                        .collect();

                    quote! { vec![#(#toks),*] }
                })
                .collect();
            quote! { (#name.into(), vec![#(#rights),*]) }
        })
        .collect();

    quote! {
        Grammar {
            rules: ::std::collections::HashMap::from([
                #(#entries),*
            ])
        }
    }
    .into()
}
