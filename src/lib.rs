extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

/// EnumDefault provides a std::Default implementation for enums
/// by using the first item as the return of <enum>::default()
#[deprecated(note="Superseded by https://rust-lang.github.io/rfcs/3107-derive-default-enum.html")]
#[proc_macro_derive(EnumDefault, attributes(default))]
pub fn enum_default_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        syn::Data::Enum(data) => {
            if data.variants.is_empty() {
                return TokenStream::default();
            }
            let name = ast.ident;
            let generics = ast.generics;

            // check if they have the "#[default]" attribute
            let iter = data.variants.iter();
            for variant in iter {
                for attr in &variant.attrs {
                    if attr.path.is_ident("default") {
                        return impl_enum_default(&name, &generics, &variant.ident);
                    }
                }
            }

            // fallback to the first item
            let first_variant = data.variants.first().unwrap();
            let variant = &first_variant.ident;
            impl_enum_default(&name, &generics, variant)
        }
        _ => TokenStream::default(),
    }
}

fn impl_enum_default(
    name: &syn::Ident,
    generics: &syn::Generics,
    variant: &syn::Ident,
) -> TokenStream {
    let result = quote! {
      impl#generics Default for #name#generics {
        fn default() -> Self {
          Self::#variant
        }
      }
    };
    result.into()
}
