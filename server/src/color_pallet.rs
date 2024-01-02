pub struct ColorPallet {}

impl ColorPallet {
    // return a color for F1 team based off of string of team name
    pub fn get_color(team: &str) -> String {
        match team {
            "Mercedes" => "#00D2BE".to_string(),
            "Ferrari" => "#DC0000".to_string(),
            "Red Bull" => "#0600EF".to_string(),
            "McLaren" => "#FF8700".to_string(),
            "Alpine F1 Team" => "#0090FF".to_string(),
            "AlphaTauri" => "#2B4562".to_string(),
            "Alfa Romeo" => "#900000".to_string(),
            "Haas F1 Team" => "#252525".to_string(),
            "Williams" => "#005AFF".to_string(),
            "Aston Martin" => "#006F62".to_string(),
            _ => "#252525".to_string(),
        }
    }
}
