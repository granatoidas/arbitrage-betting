#[derive(Debug)]
pub struct SportEvent {
    pub team1: String,
    pub team2: String,
    pub kof1: f64,
    pub kof2: f64,
    pub kof_draw: f64,
    pub provider: String,
}

impl SportEvent {
    pub fn switch_teams(&mut self) -> () {
        let team1 = self.team1.clone();
        let kof1 = self.kof1.clone();

        self.team1 = self.team2.clone();
        self.team2 = team1;
        self.kof1 = self.kof2.clone();
        self.kof2 = kof1;
    }
}
