use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Vector2(pub f64, pub f64);
