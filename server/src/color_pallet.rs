pub struct ColorPallet {}

enum TeamColor {
    Mercedes,
    Ferrari,
    RedBull,
    McLaren,
    AlpineF1Team,
    AlphaTauri,
    AlfaRomeo,
    HaasF1Team,
    Williams,
    AstonMartin,
}

impl TeamColor {
    fn to_string(&self) -> String {
        match self {
            TeamColor::Mercedes => "#00D2BE".to_string(),
            TeamColor::Ferrari => "#DC0000".to_string(),
            TeamColor::RedBull => "#0600EF".to_string(),
            TeamColor::McLaren => "#FF8700".to_string(),
            TeamColor::AlpineF1Team => "#0090FF".to_string(),
            TeamColor::AlphaTauri => "#2B4562".to_string(),
            TeamColor::AlfaRomeo => "#900000".to_string(),
            TeamColor::HaasF1Team => "#252525".to_string(),
            TeamColor::Williams => "#005AFF".to_string(),
            TeamColor::AstonMartin => "#006F62".to_string(),
        }
    }
}

impl ColorPallet {
    // return a color for F1 team based off of string of team name or driver id
    // NOTE: need to delete driver id later on once lap chart starts to provide team and driver combo.
    pub fn get_color(key: &str) -> String {
        match key {
            "Mercedes" => TeamColor::Mercedes.to_string(),
            "Ferrari" => TeamColor::Ferrari.to_string(),
            "Red Bull" => TeamColor::RedBull.to_string(),
            "McLaren" => TeamColor::McLaren.to_string(),
            "Alpine F1 Team" => TeamColor::AlpineF1Team.to_string(),
            "AlphaTauri" => TeamColor::AlphaTauri.to_string(),
            "Alfa Romeo" => TeamColor::AlfaRomeo.to_string(),
            "Haas F1 Team" => TeamColor::HaasF1Team.to_string(),
            "Williams" => TeamColor::Williams.to_string(),
            "Aston Martin" => TeamColor::AstonMartin.to_string(),
            // to be deleted below
            "max_verstappen" => TeamColor::RedBull.to_string(),
            "leclerc" => TeamColor::Ferrari.to_string(),
            "perez" => TeamColor::RedBull.to_string(),
            "sainz" => TeamColor::Ferrari.to_string(),
            "hamilton" => TeamColor::Mercedes.to_string(),
            "russell" => TeamColor::Mercedes.to_string(),
            "alonso" => TeamColor::AstonMartin.to_string(),
            "bottas" => TeamColor::AlfaRomeo.to_string(),
            "stroll" => TeamColor::AstonMartin.to_string(),
            "norris" => TeamColor::McLaren.to_string(),
            "ocon" => TeamColor::AlpineF1Team.to_string(),
            "albon" => TeamColor::Williams.to_string(),
            "sargeant" => TeamColor::Williams.to_string(),
            "hulkenberg" => TeamColor::HaasF1Team.to_string(),
            "tsunoda" => TeamColor::AlphaTauri.to_string(),
            "piastri" => TeamColor::McLaren.to_string(),
            "zhou" => TeamColor::AlfaRomeo.to_string(),
            "kevin_magnussen" => TeamColor::HaasF1Team.to_string(),
            "gasly" => TeamColor::AlpineF1Team.to_string(),
            _ => "#252525".to_string(),
        }
    }
}
