use std::error::Error;

use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Request {
    fn get(&self, url: &str) -> String;
}

pub fn get_weather(req: &impl Request, url: &str) -> Result<String, Box<dyn Error>> {
    let result = req.get(url);
    Ok(result)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn test_get_weather() {
        let mut mock = MockRequest::new();
        mock.expect_get()
            .with(eq(String::from("hey")))
            .times(1)
            .returning(|_| String::from("asda"));
        let result = get_weather(&mock, "hey").expect("Error");
        assert_eq!(String::from("asda"), result);
    }
}
