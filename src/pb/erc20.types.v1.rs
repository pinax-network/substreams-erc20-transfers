// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvents {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<TransferEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    /// contract address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// event payload
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub block_index: u64,
}
// @@protoc_insertion_point(module)
