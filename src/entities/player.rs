use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub cards: Vec<u8>,
    pub score: u8,
}

impl Player {
    pub fn remove_cards(&mut self, card: u8) -> Vec<u8> {
        let removed: Vec<u8> = self
            .cards
            .iter()
            .filter(|&&c| c == card)
            .map(|&e| e)
            .collect();
        self.cards = self
            .cards
            .iter()
            .filter(|&&c| c != card)
            .map(|&e| e)
            .collect();
        removed
    }

    pub fn has_cards(&self) -> bool {
        self.cards.len() > 0
    }

    pub fn add_cards(&mut self, cards: &[u8]) {
        self.cards.extend(cards);
    }

    pub fn reduce_groups(&mut self) -> Vec<u8> {
        let mut counter: [u8; 12] = [0; 12];
        for &card in &self.cards {
            counter[card as usize - 1] += 1;
        }
        self.cards = self
            .cards
            .clone()
            .into_iter()
            .filter(|&c| counter[c as usize - 1] < 4)
            .collect();
        let groups: Vec<u8> = counter
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (index, curr)| {
                if *curr >= 4 {
                    acc.push((index as u8) + 1);
                }
                acc
            });
        self.score += groups.len() as u8;
        groups
    }
}
