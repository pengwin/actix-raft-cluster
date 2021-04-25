use std::fmt::Display;

pub trait ActorId: 'static
+ Display
+ Clone
+ Send
+ Sync
+ std::cmp::Eq
+ std::hash::Hash
+ serde::Serialize
+ serde::de::DeserializeOwned
+ Sized
{ }

impl ActorId for u64 {}
impl ActorId for u32 {}
impl ActorId for String{}