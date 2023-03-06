pub mod configuration;

#[derive(Debug, PartialEq)]

pub struct FitzError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum GameOptions<ConfigNew, ConfigLoad> {
    View,
    New(ConfigNew),
    Load(ConfigLoad),
}

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
