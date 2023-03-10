pub mod configuration;
pub mod game;
pub mod utils;

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
#[derive(Debug, PartialEq)]
pub enum PlayerType {
    Human,
    Computer1,
    Computer2,
}

#[derive(Debug, PartialEq)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
