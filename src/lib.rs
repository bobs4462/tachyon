#[macro_use]
extern crate serde_json;
pub mod request;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::request::Request;
}
