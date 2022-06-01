use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InterpolationPoint {
    #[serde(rename = "value")]
    pub value: i32,

    #[serde(rename = "weight")]
    pub weight: i32,
}

#[derive(Deserialize, Serialize)]
pub struct InterpolationPointFloat {
    #[serde(rename = "value")]
    pub value: f32,

    #[serde(rename = "weight")]
    pub weight: f32,
}
