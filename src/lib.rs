pub mod model;
pub mod parser;

pub use model::{JsonParseError, JsonValue};
pub use parser::parse_null;
