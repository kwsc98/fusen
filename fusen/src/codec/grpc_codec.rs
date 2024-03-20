use crate::StreamBody;
use fusen_common::error::FusenError;
use http_body::Frame;
use std::{error::Error, fmt::Debug, marker::PhantomData};

use super::BodyCodec;

pub struct GrpcBodyCodec<D, E> {
    _d: PhantomData<D>,
    _e: PhantomData<E>,
}

impl<D, E> GrpcBodyCodec<D, E>
where
D: bytes::Buf + Debug,
E: std::error::Error,
{
    pub fn new() -> Self {
        GrpcBodyCodec {
            _d: PhantomData,
            _e: PhantomData,
        }
    }
}

impl<D, E> BodyCodec<D, E> for GrpcBodyCodec<D, E>
where
    D: bytes::Buf + Debug ,
    E: std::error::Error ,
{
    fn decode(&self, mut body: Frame<D>) -> Result<Vec<String>, FusenError> {
        let data = body.data_ref().unwrap().chunk();
        Ok(if data.starts_with(b"[") {
            match serde_json::from_slice(&data) {
                Ok(req) => req,
                Err(err) => return Err(FusenError::Client(err.to_string())),
            }
        } else {
            vec![String::from_utf8(data.to_vec()).unwrap()]
        })
    }

    fn encode(
        &self,
        res: Result<String, FusenError>,
    ) -> Result<StreamBody<bytes::Bytes, E>, FusenError> {
        let res = res?;
        let chunks = vec![Ok(Frame::data(bytes::Bytes::from(res)))];
        let stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(stream);
        Ok(stream_body)
    }
}