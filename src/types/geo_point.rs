use serde::Serialize;

#[derive(Serialize)]
pub struct GeoPoint {
    lat: f32,
    long: f32
}