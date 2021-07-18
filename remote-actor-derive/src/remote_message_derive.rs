use proc_macro::TokenStream;
use quote::quote;

pub fn remote_message_derive_internal(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_remote_message(&ast)
}

fn impl_remote_message(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl RemoteMessage for #name {
            fn name() -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}
