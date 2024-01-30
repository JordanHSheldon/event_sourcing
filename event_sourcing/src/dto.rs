use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Event {
    pub event_id: String,
    pub event_type: String,
    pub event_time: String,
}