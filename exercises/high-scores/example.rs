#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> u32 {
        *self.scores.last().unwrap()
    }

    pub fn personal_best(&self) -> u32 {
        *self.scores.iter().max().unwrap()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort();
        sorted.iter().rev().take(3).cloned().collect()
    }
}
