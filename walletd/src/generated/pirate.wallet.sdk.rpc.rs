/// CompactBlock is a packaging of ONLY the data from a block that's needed to:
///   1. Detect a payment to your shielded Sapling address
///   2. Detect a spend of your shielded Sapling notes
///   3. Update your witnesses to generate new Sapling spend proofs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlock {
    /// the version of this wire format, for storage
    #[prost(uint32, tag = "1")]
    pub proto_version: u32,
    /// the height of this block
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// the ID (hash) of this block, same as in block explorers
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the ID (hash) of this block's predecessor
    #[prost(bytes = "vec", tag = "4")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    /// Unix epoch time when the block was mined
    #[prost(uint32, tag = "5")]
    pub time: u32,
    /// (hash, prevHash, and time) OR (full header)
    #[prost(bytes = "vec", tag = "6")]
    pub header: ::prost::alloc::vec::Vec<u8>,
    /// zero or more compact transactions from this block
    #[prost(message, repeated, tag = "7")]
    pub vtx: ::prost::alloc::vec::Vec<CompactTx>,
}
/// CompactTx contains the minimum information for a wallet to know if this transaction
/// is relevant to it (either pays to it or spends from it) via shielded elements
/// only. This message will not encode a transparent-to-transparent transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactTx {
    /// the index within the full block
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// the ID (hash) of this transaction, same as in block explorers
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// The transaction fee: present if server can provide. In the case of a
    /// stateless server and a transaction with transparent inputs, this will be
    /// unset because the calculation requires reference to prior transactions.
    /// in a pure-Sapling context, the fee will be calculable as:
    ///    valueBalance + (sum(vPubNew) - sum(vPubOld) - sum(tOut))
    #[prost(uint32, tag = "3")]
    pub fee: u32,
    /// inputs
    #[prost(message, repeated, tag = "4")]
    pub spends: ::prost::alloc::vec::Vec<CompactSpend>,
    /// outputs
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<CompactOutput>,
}
/// CompactSpend is a Sapling Spend Description as described in 7.3 of the Zcash
/// protocol specification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactSpend {
    /// nullifier (see the Zcash protocol specification)
    #[prost(bytes = "vec", tag = "1")]
    pub nf: ::prost::alloc::vec::Vec<u8>,
}
/// output is a Sapling Output Description as described in section 7.4 of the
/// Zcash protocol spec. Total size is 948.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactOutput {
    /// note commitment u-coordinate
    #[prost(bytes = "vec", tag = "1")]
    pub cmu: ::prost::alloc::vec::Vec<u8>,
    /// ephemeral public key
    #[prost(bytes = "vec", tag = "2")]
    pub epk: ::prost::alloc::vec::Vec<u8>,
    /// ciphertext and zkproof
    #[prost(bytes = "vec", tag = "3")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
/// A BlockID message contains identifiers to select a block: a height or a
/// hash. Specification by hash is not implemented, but may be in the future.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockId {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// BlockRange specifies a series of blocks from start to end inclusive.
/// Both BlockIDs must be heights; specification by hash is not yet supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<BlockId>,
}
/// A TxFilter contains the information needed to identify a particular
/// transaction: either a block and an index, or a direct transaction hash.
/// Currently, only specification by hash is supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxFilter {
    /// block identifier, height or hash
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockId>,
    /// index within the block
    #[prost(uint64, tag = "2")]
    pub index: u64,
    /// transaction ID (hash, txid)
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// RawTransaction contains the complete transaction data. It also optionally includes
/// the block height in which the transaction was included.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawTransaction {
    /// exact data returned by Zcash 'getrawtransaction'
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// height that the transaction was mined (or -1)
    #[prost(uint64, tag = "2")]
    pub height: u64,
}
/// A SendResponse encodes an error code and a string. It is currently used
/// only by SendTransaction(). If error code is zero, the operation was
/// successful; if non-zero, it and the message specify the failure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(int32, tag = "1")]
    pub error_code: i32,
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}
/// Chainspec is a placeholder to allow specification of a particular chain fork.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainSpec {}
/// Empty is for gRPCs that take no arguments, currently only GetLightdInfo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// LightdInfo returns various information about this lightwalletd instance
/// and the state of the blockchain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightdInfo {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vendor: ::prost::alloc::string::String,
    /// true
    #[prost(bool, tag = "3")]
    pub taddr_support: bool,
    /// either "main" or "test"
    #[prost(string, tag = "4")]
    pub chain_name: ::prost::alloc::string::String,
    /// depends on mainnet or testnet
    #[prost(uint64, tag = "5")]
    pub sapling_activation_height: u64,
    /// protocol identifier, see consensus/upgrades.cpp
    #[prost(string, tag = "6")]
    pub consensus_branch_id: ::prost::alloc::string::String,
    /// latest block on the best chain
    #[prost(uint64, tag = "7")]
    pub block_height: u64,
    #[prost(string, tag = "8")]
    pub git_commit: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub branch: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub build_date: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub build_user: ::prost::alloc::string::String,
    /// less than tip height if zcashd is syncing
    #[prost(uint64, tag = "12")]
    pub estimated_height: u64,
    /// example: "v4.1.1-877212414"
    #[prost(string, tag = "13")]
    pub zcashd_build: ::prost::alloc::string::String,
    /// example: "/MagicBean:4.1.1/"
    #[prost(string, tag = "14")]
    pub zcashd_subversion: ::prost::alloc::string::String,
}
/// TransparentAddressBlockFilter restricts the results to the given address
/// or block range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransparentAddressBlockFilter {
    /// t-address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// start, end heights
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<BlockRange>,
}
/// Duration is currently used only for testing, so that the Ping rpc
/// can simulate a delay, to create many simultaneous connections. Units
/// are microseconds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    #[prost(int64, tag = "1")]
    pub interval_us: i64,
}
/// PingResponse is used to indicate concurrency, how many Ping rpcs
/// are executing upon entry and upon exit (after the delay).
/// This rpc is used for testing only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(int64, tag = "1")]
    pub entry: i64,
    #[prost(int64, tag = "2")]
    pub exit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressList {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(int64, tag = "1")]
    pub value_zat: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exclude {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// The TreeState is derived from the Zcash z_gettreestate rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeState {
    /// "main" or "test"
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// block id
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    /// Unix epoch time when the block was mined
    #[prost(uint32, tag = "4")]
    pub time: u32,
    /// sapling commitment tree state
    #[prost(string, tag = "5")]
    pub tree: ::prost::alloc::string::String,
}
/// Results are sorted by height, which makes it easy to issue another
/// request that picks up from where the previous left off.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressUtxosArg {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "2")]
    pub start_height: u64,
    /// zero means unlimited
    #[prost(uint32, tag = "3")]
    pub max_entries: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressUtxosReply {
    #[prost(string, tag = "6")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag = "2")]
    pub index: i32,
    #[prost(bytes = "vec", tag = "3")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "4")]
    pub value_zat: i64,
    #[prost(uint64, tag = "5")]
    pub height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressUtxosReplyList {
    #[prost(message, repeated, tag = "1")]
    pub address_utxos: ::prost::alloc::vec::Vec<GetAddressUtxosReply>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRequest {
    /// List of timestamps(in sec) at which the price is being requested
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// 3 letter currency-code
    #[prost(string, tag = "2")]
    pub currency: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceResponse {
    /// Timestamp at which this price quote was fetched. Note, this may not be the same
    /// as the request timestamp, but the server will respond with the closest timestamp that it has/can fetch    
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    /// 3-letter currency code, matching the request
    #[prost(string, tag = "2")]
    pub currency: ::prost::alloc::string::String,
    /// price of ZEC
    #[prost(double, tag = "3")]
    pub price: f64,
}
#[doc = r" Generated client implementations."]
pub mod compact_tx_streamer_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct CompactTxStreamerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CompactTxStreamerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CompactTxStreamerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Return the height of the tip of the best chain"]
        pub async fn get_latest_block(
            &mut self,
            request: impl tonic::IntoRequest<super::ChainSpec>,
        ) -> Result<tonic::Response<super::BlockId>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetLatestBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Return the compact block corresponding to the given block identifier"]
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockId>,
        ) -> Result<tonic::Response<super::CompactBlock>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Return a list of consecutive compact blocks"]
        pub async fn get_block_range(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockRange>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CompactBlock>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetBlockRange",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Get the historical and current prices "]
        pub async fn get_zec_price(
            &mut self,
            request: impl tonic::IntoRequest<super::PriceRequest>,
        ) -> Result<tonic::Response<super::PriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetZECPrice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_current_zec_price(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::PriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetCurrentZECPrice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Return the requested full (not compact) transaction (as from zcashd)"]
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::TxFilter>,
        ) -> Result<tonic::Response<super::RawTransaction>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submit the given transaction to the Zcash network"]
        pub async fn send_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::RawTransaction>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/SendTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Return the txids corresponding to the given t-address within the given block range"]
        pub async fn get_taddress_txids(
            &mut self,
            request: impl tonic::IntoRequest<super::TransparentAddressBlockFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RawTransaction>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTaddressTxids",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Legacy API that is used as a fallback for t-Address support, if the server is running the old version (lwdv2)"]
        pub async fn get_address_txids(
            &mut self,
            request: impl tonic::IntoRequest<super::TransparentAddressBlockFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RawTransaction>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetAddressTxids",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_taddress_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::AddressList>,
        ) -> Result<tonic::Response<super::Balance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_taddress_balance_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Address>,
        ) -> Result<tonic::Response<super::Balance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalanceStream",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Return the compact transactions currently in the mempool; the results"]
        #[doc = " can be a few seconds out of date. If the Exclude list is empty, return"]
        #[doc = " all transactions; otherwise return all *except* those in the Exclude list"]
        #[doc = " (if any); this allows the client to avoid receiving transactions that it"]
        #[doc = " already has (from an earlier call to this rpc). The transaction IDs in the"]
        #[doc = " Exclude list can be shortened to any number of bytes to make the request"]
        #[doc = " more bandwidth-efficient; if two or more transactions in the mempool"]
        #[doc = " match a shortened txid, they are all sent (none is excluded). Transactions"]
        #[doc = " in the exclude list that don't exist in the mempool are ignored."]
        pub async fn get_mempool_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::Exclude>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CompactTx>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetMempoolTx",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " GetTreeState returns the note commitment tree state corresponding to the given block."]
        #[doc = " See section 3.7 of the Zcash protocol specification. It returns several other useful"]
        #[doc = " values also (even though they can be obtained using GetBlock)."]
        #[doc = " The block can be specified by either height or hash."]
        pub async fn get_tree_state(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockId>,
        ) -> Result<tonic::Response<super::TreeState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTreeState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_address_utxos(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAddressUtxosArg>,
        ) -> Result<tonic::Response<super::GetAddressUtxosReplyList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_address_utxos_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAddressUtxosArg>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetAddressUtxosReply>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxosStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Return information about this lightwalletd instance and the blockchain"]
        pub async fn get_lightd_info(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::LightdInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetLightdInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Testing-only, requires lightwalletd --ping-very-insecure (do not enable in production)"]
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::Duration>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/Ping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CompactTxStreamerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CompactTxStreamerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CompactTxStreamerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod compact_tx_streamer_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CompactTxStreamerServer."]
    #[async_trait]
    pub trait CompactTxStreamer: Send + Sync + 'static {
        #[doc = " Return the height of the tip of the best chain"]
        async fn get_latest_block(
            &self,
            request: tonic::Request<super::ChainSpec>,
        ) -> Result<tonic::Response<super::BlockId>, tonic::Status>;
        #[doc = " Return the compact block corresponding to the given block identifier"]
        async fn get_block(
            &self,
            request: tonic::Request<super::BlockId>,
        ) -> Result<tonic::Response<super::CompactBlock>, tonic::Status>;
        #[doc = "Server streaming response type for the GetBlockRange method."]
        type GetBlockRangeStream: futures_core::Stream<Item = Result<super::CompactBlock, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Return a list of consecutive compact blocks"]
        async fn get_block_range(
            &self,
            request: tonic::Request<super::BlockRange>,
        ) -> Result<tonic::Response<Self::GetBlockRangeStream>, tonic::Status>;
        #[doc = " Get the historical and current prices "]
        async fn get_zec_price(
            &self,
            request: tonic::Request<super::PriceRequest>,
        ) -> Result<tonic::Response<super::PriceResponse>, tonic::Status>;
        async fn get_current_zec_price(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::PriceResponse>, tonic::Status>;
        #[doc = " Return the requested full (not compact) transaction (as from zcashd)"]
        async fn get_transaction(
            &self,
            request: tonic::Request<super::TxFilter>,
        ) -> Result<tonic::Response<super::RawTransaction>, tonic::Status>;
        #[doc = " Submit the given transaction to the Zcash network"]
        async fn send_transaction(
            &self,
            request: tonic::Request<super::RawTransaction>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the GetTaddressTxids method."]
        type GetTaddressTxidsStream: futures_core::Stream<Item = Result<super::RawTransaction, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Return the txids corresponding to the given t-address within the given block range"]
        async fn get_taddress_txids(
            &self,
            request: tonic::Request<super::TransparentAddressBlockFilter>,
        ) -> Result<tonic::Response<Self::GetTaddressTxidsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetAddressTxids method."]
        type GetAddressTxidsStream: futures_core::Stream<Item = Result<super::RawTransaction, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Legacy API that is used as a fallback for t-Address support, if the server is running the old version (lwdv2)"]
        async fn get_address_txids(
            &self,
            request: tonic::Request<super::TransparentAddressBlockFilter>,
        ) -> Result<tonic::Response<Self::GetAddressTxidsStream>, tonic::Status>;
        async fn get_taddress_balance(
            &self,
            request: tonic::Request<super::AddressList>,
        ) -> Result<tonic::Response<super::Balance>, tonic::Status>;
        async fn get_taddress_balance_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::Address>>,
        ) -> Result<tonic::Response<super::Balance>, tonic::Status>;
        #[doc = "Server streaming response type for the GetMempoolTx method."]
        type GetMempoolTxStream: futures_core::Stream<Item = Result<super::CompactTx, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Return the compact transactions currently in the mempool; the results"]
        #[doc = " can be a few seconds out of date. If the Exclude list is empty, return"]
        #[doc = " all transactions; otherwise return all *except* those in the Exclude list"]
        #[doc = " (if any); this allows the client to avoid receiving transactions that it"]
        #[doc = " already has (from an earlier call to this rpc). The transaction IDs in the"]
        #[doc = " Exclude list can be shortened to any number of bytes to make the request"]
        #[doc = " more bandwidth-efficient; if two or more transactions in the mempool"]
        #[doc = " match a shortened txid, they are all sent (none is excluded). Transactions"]
        #[doc = " in the exclude list that don't exist in the mempool are ignored."]
        async fn get_mempool_tx(
            &self,
            request: tonic::Request<super::Exclude>,
        ) -> Result<tonic::Response<Self::GetMempoolTxStream>, tonic::Status>;
        #[doc = " GetTreeState returns the note commitment tree state corresponding to the given block."]
        #[doc = " See section 3.7 of the Zcash protocol specification. It returns several other useful"]
        #[doc = " values also (even though they can be obtained using GetBlock)."]
        #[doc = " The block can be specified by either height or hash."]
        async fn get_tree_state(
            &self,
            request: tonic::Request<super::BlockId>,
        ) -> Result<tonic::Response<super::TreeState>, tonic::Status>;
        async fn get_address_utxos(
            &self,
            request: tonic::Request<super::GetAddressUtxosArg>,
        ) -> Result<tonic::Response<super::GetAddressUtxosReplyList>, tonic::Status>;
        #[doc = "Server streaming response type for the GetAddressUtxosStream method."]
        type GetAddressUtxosStreamStream: futures_core::Stream<Item = Result<super::GetAddressUtxosReply, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_address_utxos_stream(
            &self,
            request: tonic::Request<super::GetAddressUtxosArg>,
        ) -> Result<tonic::Response<Self::GetAddressUtxosStreamStream>, tonic::Status>;
        #[doc = " Return information about this lightwalletd instance and the blockchain"]
        async fn get_lightd_info(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::LightdInfo>, tonic::Status>;
        #[doc = " Testing-only, requires lightwalletd --ping-very-insecure (do not enable in production)"]
        async fn ping(
            &self,
            request: tonic::Request<super::Duration>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CompactTxStreamerServer<T: CompactTxStreamer> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CompactTxStreamer> CompactTxStreamerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CompactTxStreamerServer<T>
    where
        T: CompactTxStreamer,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetLatestBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestBlockSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::ChainSpec> for GetLatestBlockSvc<T> {
                        type Response = super::BlockId;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChainSpec>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_latest_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetLatestBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::BlockId> for GetBlockSvc<T> {
                        type Response = super::CompactBlock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetBlockRange" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockRangeSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer>
                        tonic::server::ServerStreamingService<super::BlockRange>
                        for GetBlockRangeSvc<T>
                    {
                        type Response = super::CompactBlock;
                        type ResponseStream = T::GetBlockRangeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockRange>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_range(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetBlockRangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetZECPrice" => {
                    #[allow(non_camel_case_types)]
                    struct GetZECPriceSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::PriceRequest> for GetZECPriceSvc<T> {
                        type Response = super::PriceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PriceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_zec_price(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetZECPriceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetCurrentZECPrice" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentZECPriceSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::Empty> for GetCurrentZECPriceSvc<T> {
                        type Response = super::PriceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Empty>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_current_zec_price(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCurrentZECPriceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::TxFilter> for GetTransactionSvc<T> {
                        type Response = super::RawTransaction;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/SendTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendTransactionSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::RawTransaction>
                        for SendTransactionSvc<T>
                    {
                        type Response = super::SendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RawTransaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTaddressTxids" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaddressTxidsSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer>
                        tonic::server::ServerStreamingService<super::TransparentAddressBlockFilter>
                        for GetTaddressTxidsSvc<T>
                    {
                        type Response = super::RawTransaction;
                        type ResponseStream = T::GetTaddressTxidsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransparentAddressBlockFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_taddress_txids(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetTaddressTxidsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetAddressTxids" => {
                    #[allow(non_camel_case_types)]
                    struct GetAddressTxidsSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer>
                        tonic::server::ServerStreamingService<super::TransparentAddressBlockFilter>
                        for GetAddressTxidsSvc<T>
                    {
                        type Response = super::RawTransaction;
                        type ResponseStream = T::GetAddressTxidsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransparentAddressBlockFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_address_txids(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetAddressTxidsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaddressBalanceSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::AddressList>
                        for GetTaddressBalanceSvc<T>
                    {
                        type Response = super::Balance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddressList>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_taddress_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTaddressBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalanceStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaddressBalanceStreamSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::ClientStreamingService<super::Address>
                        for GetTaddressBalanceStreamSvc<T>
                    {
                        type Response = super::Balance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::Address>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_taddress_balance_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetTaddressBalanceStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetMempoolTx" => {
                    #[allow(non_camel_case_types)]
                    struct GetMempoolTxSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::ServerStreamingService<super::Exclude>
                        for GetMempoolTxSvc<T>
                    {
                        type Response = super::CompactTx;
                        type ResponseStream = T::GetMempoolTxStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Exclude>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_mempool_tx(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetMempoolTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetTreeState" => {
                    #[allow(non_camel_case_types)]
                    struct GetTreeStateSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::BlockId> for GetTreeStateSvc<T> {
                        type Response = super::TreeState;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tree_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTreeStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxos" => {
                    #[allow(non_camel_case_types)]
                    struct GetAddressUtxosSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer>
                        tonic::server::UnaryService<super::GetAddressUtxosArg>
                        for GetAddressUtxosSvc<T>
                    {
                        type Response = super::GetAddressUtxosReplyList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAddressUtxosArg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_address_utxos(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAddressUtxosSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxosStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetAddressUtxosStreamSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer>
                        tonic::server::ServerStreamingService<super::GetAddressUtxosArg>
                        for GetAddressUtxosStreamSvc<T>
                    {
                        type Response = super::GetAddressUtxosReply;
                        type ResponseStream = T::GetAddressUtxosStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAddressUtxosArg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_address_utxos_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetAddressUtxosStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/GetLightdInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetLightdInfoSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::Empty> for GetLightdInfoSvc<T> {
                        type Response = super::LightdInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Empty>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_lightd_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetLightdInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pirate.wallet.sdk.rpc.CompactTxStreamer/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: CompactTxStreamer>(pub Arc<T>);
                    impl<T: CompactTxStreamer> tonic::server::UnaryService<super::Duration> for PingSvc<T> {
                        type Response = super::PingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Duration>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CompactTxStreamer> Clone for CompactTxStreamerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CompactTxStreamer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CompactTxStreamer> tonic::transport::NamedService for CompactTxStreamerServer<T> {
        const NAME: &'static str = "pirate.wallet.sdk.rpc.CompactTxStreamer";
    }
}
