use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, parse_quote};

/// Implements `redis::ToRedisArgs` by serializing the value as JSON.
#[proc_macro_derive(RedisArgs)]
pub fn derive_redis_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut generics = input.generics;
    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: ::serde::Serialize));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::redis::ToRedisArgs for #name #ty_generics #where_clause {
            fn write_redis_args<W: ?Sized + ::redis::RedisWrite>(&self, out: &mut W) {
                let json = ::serde_json::to_string(self)
                    .expect("serialize value for Redis");
                <::std::string::String as ::redis::ToRedisArgs>::write_redis_args(&json, out);
            }
        }
    }
    .into()
}

/// Implements `redis::FromRedisValue` by deserializing JSON.
#[proc_macro_derive(RedisValue)]
pub fn derive_redis_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut generics = input.generics;
    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: ::serde::de::DeserializeOwned));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::redis::FromRedisValue for #name #ty_generics #where_clause {
            fn from_redis_value(value: &::redis::Value) -> ::redis::RedisResult<Self> {
                let json = <::std::string::String as ::redis::FromRedisValue>::from_redis_value(value)?;
                ::serde_json::from_str(&json).map_err(|error| {
                    (
                        ::redis::ErrorKind::TypeError,
                        "JSON parse error",
                        error.to_string(),
                    ).into()
                })
            }
        }
    }
    .into()
}
