use serde::{Deserialize, Serialize};
use std::fmt::Display;
use actix_web::body::MessageBody;
use std::convert::Infallible;
use actix_web::body::BodySize;
use std::task::{Context, Poll};
use std::pin::Pin;
use actix_web::web::Bytes;

#[derive(Serialize,Deserialize)]
pub struct Event {
    pub event_id: String,
    pub event_type: String,
    pub event_time: String,
}

