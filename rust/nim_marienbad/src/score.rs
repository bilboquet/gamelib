pub struct Scorer {
    wins: Vec<u32>,
}

impl Scorer {
    pub fn new(players_count: usize) -> Self {
        Self {
            wins: vec![0u32; players_count],
        }
    }

    pub fn winner(&mut self, player: usize) {
        self.wins[player - 1] += 1;
        self.display();
    }

    pub fn display(&self) {
        let sum: u32 = self.wins.iter().sum();

        for (p, &s) in self.wins.iter().enumerate() {
            let s = s * 100 / sum;
            println!("player{} won {}%", p + 1, s)
        }
    }
}
