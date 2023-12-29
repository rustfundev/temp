use std::error::Error;

use mockall::{automock, mock, predicate::*};

use crate::coordinate::Coordinates;

#[cfg_attr(test, automock)]
pub trait Request {
    fn get(&self, coordinates: &Coordinates) -> String;
}

pub fn get_weather(
    req: &impl Request,
    coordinates: &Coordinates,
) -> Result<String, Box<dyn Error>> {
    let result = req.get(coordinates);
    Ok(result)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn test_get_weather() {
        let mut mock = MockRequest::new();
        let latitude = 35.0f64;
        let longitude = 34.0f64;
        let coordinates = Coordinates::new(latitude, longitude);
        mock.expect_get()
            .with(eq(coordinates))
            .times(1)
            .returning(|_| String::from("asda"));
        let result = get_weather(&mock, &coordinates).expect("Error");
        assert_eq!(String::from("asda"), result);
    }
}
