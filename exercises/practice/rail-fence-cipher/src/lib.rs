pub struct RailFence;

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        unimplemented!("Construct a new fence with {} rails", rails)
    }

    pub fn encode(&self, text: &str) -> String {
        unimplemented!("Encode this text: {}", text)
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
    }
}
