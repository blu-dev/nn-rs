use syn::{parse_macro_input, token};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;

#[proc_macro]
pub fn static_c_str(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as syn::LitStr);
    syn::LitStr::new(format!("{}\u{0}", literal.value()).as_str(), Span::call_site()).to_token_stream().into()
}

#[proc_macro_attribute]
pub fn dev_inline(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut usr_fn = parse_macro_input!(input as syn::ItemFn);
    usr_fn.attrs.push(
        syn::Attribute {
            pound_token: token::Pound { spans: [Span::call_site()] },
            style: syn::AttrStyle::Outer,
            bracket_token: token::Bracket { span: Span::call_site() },
            path: syn::Ident::new("cfg_attr", Span::call_site()).into(),
            tokens: "(feature = \"dev_inline\", inline(never))".parse().unwrap()
        }
    );
    usr_fn.to_token_stream().into()
}