mod remote_message_derive;

use proc_macro::TokenStream;

use self::remote_message_derive::remote_message_derive_internal;

#[proc_macro_derive(RemoteMessage)]
pub fn remote_message_derive(input: TokenStream) -> TokenStream {
    remote_message_derive_internal(input)
}
