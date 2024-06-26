/// Core reservation object. contains all the information for a reservation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// Unique reservation id, if put into ReserveRequest, id should be empty.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// User id who made the reservation.
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// Reservation status, use for different purpose.
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    pub status: i32,
    /// Reserved resources.
    #[prost(string, tag = "4")]
    pub resource_id: ::prost::alloc::string::String,
    /// Start time for the reservation.
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    /// End time for the reservation.
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// Extra note.
    #[prost(string, tag = "7")]
    pub note: ::prost::alloc::string::String,
}
/// To make a reservation, send a ReserveRequest with Reservation object(id should be empty).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveRequest {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// After a reservation is made, the ReserveResponse will be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To change a reservation status from pending to confirmed, send a ConfirmRequest object with reservation id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// After a reservation is confirmed, the ConfirmResponse will be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To update a reservation note, send a UpdateRequest object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub note: ::prost::alloc::string::String,
}
/// After the reservation node is updated, the UpdateResponse will be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To cancel a reservation, send a CancelRequest object, the reservation will be deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// After the reservation is deleted, the CancelResponse will be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To get a reservation, send a GetRequest object with reservation id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// After the reservation is found, the GetResponse will be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To query reservations, send a QueryRequest object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// (option)filter by resource id.
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    /// (option)filter by user id.
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// (option)use status to filter reservations. if UNKNOWN, return all reservations
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    pub status: i32,
    /// (option)filter by start time, if 0, use Infinity for start time.
    #[prost(message, optional, tag = "4")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    /// (option)filter by end time, if 0, use Infinity for end time.
    #[prost(message, optional, tag = "5")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
/// Client can watch to reservation changes by sending a WatchRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchRequest {}
/// Server can send WatchResponse to client in stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchResponse {
    /// Change type for the reservation.
    #[prost(enumeration = "ReservationChangeType", tag = "1")]
    pub change_type: i32,
}
/// Reservation status for a given time period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReservationStatus {
    Unknown = 0,
    Pending = 1,
    Confirmed = 2,
    Blocked = 3,
}
impl ReservationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReservationStatus::Unknown => "RESERVATION_STATUS_UNKNOWN",
            ReservationStatus::Pending => "RESERVATION_STATUS_PENDING",
            ReservationStatus::Confirmed => "RESERVATION_STATUS_CONFIRMED",
            ReservationStatus::Blocked => "RESERVATION_STATUS_BLOCKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVATION_STATUS_UNKNOWN" => Some(Self::Unknown),
            "RESERVATION_STATUS_PENDING" => Some(Self::Pending),
            "RESERVATION_STATUS_CONFIRMED" => Some(Self::Confirmed),
            "RESERVATION_STATUS_BLOCKED" => Some(Self::Blocked),
            _ => None,
        }
    }
}
/// When a reservation is created/updated/deleted, it will be notified to the watch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReservationChangeType {
    Unknown = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
}
impl ReservationChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReservationChangeType::Unknown => "RESERVATION_CHANGE_TYPE_UNKNOWN",
            ReservationChangeType::Create => "RESERVATION_CHANGE_TYPE_CREATE",
            ReservationChangeType::Update => "RESERVATION_CHANGE_TYPE_UPDATE",
            ReservationChangeType::Delete => "RESERVATION_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVATION_CHANGE_TYPE_UNKNOWN" => Some(Self::Unknown),
            "RESERVATION_CHANGE_TYPE_CREATE" => Some(Self::Create),
            "RESERVATION_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "RESERVATION_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod reservation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// ReservationService provides reservation operations.
    #[derive(Debug, Clone)]
    pub struct ReservationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReservationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReservationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ReservationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ReservationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Make a reservation.
        pub async fn reserve(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveRequest>,
        ) -> std::result::Result<tonic::Response<super::ReserveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/reserve");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "reserve"));
            self.inner.unary(req, path, codec).await
        }
        /// Confirm a pending reservation, if reservation is not pending, do nothing.
        pub async fn confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmRequest>,
        ) -> std::result::Result<tonic::Response<super::ConfirmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/confirm");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "confirm"));
            self.inner.unary(req, path, codec).await
        }
        /// Update the reservation note.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/update");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "update"));
            self.inner.unary(req, path, codec).await
        }
        /// Cancel a reservation.
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRequest>,
        ) -> std::result::Result<tonic::Response<super::CancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/cancel");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "cancel"));
            self.inner.unary(req, path, codec).await
        }
        /// Get a reservation by id.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/reservation.ReservationService/get");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "get"));
            self.inner.unary(req, path, codec).await
        }
        /// Query reservations by resource id, user id, status, start time, end time.
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Reservation>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/query");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "query"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// another system could watch for reservation changes like: added/confirmed/canceled
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/watch");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("reservation.ReservationService", "watch"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod reservation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReservationServiceServer.
    #[async_trait]
    pub trait ReservationService: Send + Sync + 'static {
        /// Make a reservation.
        async fn reserve(
            &self,
            request: tonic::Request<super::ReserveRequest>,
        ) -> std::result::Result<tonic::Response<super::ReserveResponse>, tonic::Status>;
        /// Confirm a pending reservation, if reservation is not pending, do nothing.
        async fn confirm(
            &self,
            request: tonic::Request<super::ConfirmRequest>,
        ) -> std::result::Result<tonic::Response<super::ConfirmResponse>, tonic::Status>;
        /// Update the reservation note.
        async fn update(
            &self,
            request: tonic::Request<super::UpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
        /// Cancel a reservation.
        async fn cancel(
            &self,
            request: tonic::Request<super::CancelRequest>,
        ) -> std::result::Result<tonic::Response<super::CancelResponse>, tonic::Status>;
        /// Get a reservation by id.
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status>;
        /// Server streaming response type for the query method.
        type queryStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Reservation, tonic::Status>,
            > + Send
            + 'static;
        /// Query reservations by resource id, user id, status, start time, end time.
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<Self::queryStream>, tonic::Status>;
        /// Server streaming response type for the watch method.
        type watchStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WatchResponse, tonic::Status>,
            > + Send
            + 'static;
        /// another system could watch for reservation changes like: added/confirmed/canceled
        async fn watch(
            &self,
            request: tonic::Request<super::WatchRequest>,
        ) -> std::result::Result<tonic::Response<Self::watchStream>, tonic::Status>;
    }
    /// ReservationService provides reservation operations.
    #[derive(Debug)]
    pub struct ReservationServiceServer<T: ReservationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ReservationService> ReservationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReservationServiceServer<T>
    where
        T: ReservationService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/reservation.ReservationService/reserve" => {
                    #[allow(non_camel_case_types)]
                    struct reserveSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::ReserveRequest> for reserveSvc<T> {
                        type Response = super::ReserveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReserveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::reserve(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = reserveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/confirm" => {
                    #[allow(non_camel_case_types)]
                    struct confirmSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::ConfirmRequest> for confirmSvc<T> {
                        type Response = super::ConfirmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::confirm(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = confirmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::UpdateRequest> for updateSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/cancel" => {
                    #[allow(non_camel_case_types)]
                    struct cancelSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::CancelRequest> for cancelSvc<T> {
                        type Response = super::CancelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::cancel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = cancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::GetRequest> for getSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::get(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/query" => {
                    #[allow(non_camel_case_types)]
                    struct querySvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService>
                        tonic::server::ServerStreamingService<super::QueryRequest> for querySvc<T>
                    {
                        type Response = super::Reservation;
                        type ResponseStream = T::queryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::query(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = querySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/watch" => {
                    #[allow(non_camel_case_types)]
                    struct watchSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService>
                        tonic::server::ServerStreamingService<super::WatchRequest> for watchSvc<T>
                    {
                        type Response = super::WatchResponse;
                        type ResponseStream = T::watchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReservationService>::watch(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = watchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ReservationService> Clone for ReservationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ReservationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReservationService> tonic::server::NamedService for ReservationServiceServer<T> {
        const NAME: &'static str = "reservation.ReservationService";
    }
}
