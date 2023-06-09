use proc_macro2::{Ident, TokenStream};
use quote::quote;
use crate::macros::structs::{Context, Fd};

impl Context {
    pub fn render_cq_code(&self) -> TokenStream {
        let name = &self.name;

        let cq_code_name = Ident::new(&format!("{}CQ", name), name.span());

        let optionized_fields = self.gen_optionized_fields();

        quote! {
            impl CQCode for #name{
                fn cq_code(&self) -> CQCodeType {
                    self.typ.clone()
                }
            }
        }
    }

    fn gen_optionized_fields(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| quote! {
                #name: ty,
            })
            .collect()
    }
}