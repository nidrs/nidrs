use nidrs_extern::{
    axum::{self, body::Bytes, http::request::Parts},
    meta::Meta,
    serde::{de::DeserializeOwned, Deserialize},
    serde_json::Value,
};

use crate::AppResult;

#[derive(Debug)]
pub struct Query<T>(T);

#[derive(Debug)]
pub struct Body<T>(T);

#[derive(Debug)]
pub struct Param<T>(T);

pub trait FromExtractor<'a> {
    type Output;

    fn extract(ext: &'a Extractor) -> AppResult<Self::Output>;
}

pub struct Extractor {
    pub meta: Meta,
    pub query: axum::extract::Query<Value>,
    pub parts: Parts,
    pub body: Bytes,
}

impl<'a, T> FromExtractor<'a> for Query<T>
where
    T: Deserialize<'a>,
{
    type Output = Self;

    fn extract(ext: &'a Extractor) -> AppResult<Self::Output> {
        let axum::extract::Query::<Value>(v) = &ext.query;
        Ok(Self(T::deserialize(v)?))
    }
}

impl<'a, T> FromExtractor<'a> for Body<T>
where
    T: DeserializeOwned,
{
    type Output = Self;

    fn extract(ext: &'a Extractor) -> AppResult<Self::Output> {
        let t = axum::extract::Json::<T>::from_bytes(&ext.body).unwrap(); //TODO:
        Ok(Self(t.0))
    }
}
