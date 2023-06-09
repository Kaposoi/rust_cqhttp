use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use crate::macros::structs::Context;

mod macros;

/// 派生宏， 用于要实现[`crate::message::cq_code::CQCode`]的结构体上， 可以自动让结构体实现该`trait`
#[proc_macro_derive(CQCode)]
pub fn cq_code(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Context::from(input).render_cq_code().into()
}
