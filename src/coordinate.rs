pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

impl Coordinates {
    pub fn new(latitude: String, longitude: String) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}
