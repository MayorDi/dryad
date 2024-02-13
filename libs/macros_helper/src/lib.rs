use proc_macro::TokenStream;

#[proc_macro_derive(GetPosition)]
pub fn get_position_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();

    impl_get_position(&ast)
}

fn impl_get_position(input: &syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;
    (quote::quote! {
        impl GetPosition for #ident {
            fn get_position(&self) -> Position {
                self.position
            }

            fn get_index(&self) -> usize {
                let (x, y) = (self.get_position().x, self.get_position().y);
                get_index(x, y, SIZE_WORLD[0])
            }
        }
    })
    .into()
}
