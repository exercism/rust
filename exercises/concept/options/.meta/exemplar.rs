pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Player {
                health: 100,
                level: self.level,
                mana: if self.level >= 10 { Some(100) } else { None },
            })
        } else {
            // reviving a player who isn't dead has no effect
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(ref mut m) => {
                if *m >= mana_cost {
                    *m -= mana_cost;
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}
