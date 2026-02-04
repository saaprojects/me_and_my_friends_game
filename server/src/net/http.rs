use crate::prelude::*;
use crate::services;

pub async fn health() -> Json<Health> {
    Json(services::health::health())
}
