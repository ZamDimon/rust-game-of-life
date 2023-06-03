use crate::settings::parser;

lazy_static! {
    pub static ref CONFIG: parser::Settings =
        parser::Settings::new().expect("config cannot be loaded!");
}