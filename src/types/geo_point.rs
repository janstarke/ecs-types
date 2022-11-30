use serde::Serialize;

#[derive(Serialize, Default, Clone)]
pub struct GeoPoint {
    lat: f32,
    lon: f32
}

impl GeoPoint {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self {
            lat,
            lon
        }
    }
}