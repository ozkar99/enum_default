extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;



/// EnumDefault provides a std::Default implementation for enums
/// by using the first item as the return of <enum>::default()
#[proc_macro_derive(EnumDefault)]
pub fn enum_default_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        syn::Data::Enum(data) => {
            if data.variants.is_empty() {
                return TokenStream::default();
            }

            let name = ast.ident;
            let first_variant = data.variants.first().unwrap();
            let first = &first_variant.ident;

            let result = quote! {
              impl Default for #name {
                fn default() -> #name {
                  #name::#first
                }
              }
            };
            result.into()
        }
        _ => TokenStream::default(),
    }
}
