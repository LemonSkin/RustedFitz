use crate::FitzError;

#[derive(Debug, PartialEq)]
pub struct ConfigPlayers {
    pub p1type: char,
    pub p2type: char,
}

impl ConfigPlayers {
    pub fn build(p1: &String, p2: &String) -> Result<ConfigPlayers, FitzError> {
        let config_players: ConfigPlayers = ConfigPlayers {
            p1type: p1.chars().next().unwrap(),
            p2type: p2.chars().next().unwrap(),
        };

        // Ensure p1type and p2type are valid characters
        let valid_pxtypes = ['1', '2', 'h'];
        if !valid_pxtypes.contains(&config_players.p1type)
            || !valid_pxtypes.contains(&config_players.p2type)
        {
            return Err(FitzError {
                code: 4,
                message: "Invalid player type".to_string(),
            });
        }

        return Ok(config_players);
    }
}
