use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use crate::token::Token;

struct Bandit {
    arms: Vec<BigDecimal>,
    rewards: HashMap<BigDecimal,(BigDecimal, BigDecimal)>,
    mean_gain: HashMap<BigDecimal, BigDecimal>,
    inventory: HashMap<Token, BigDecimal>,
}

impl Bandit {
    pub fn new() -> Self {

        let mut rewards = HashMap::new();
        let mut mean_gain = HashMap::new();
        let mut inventory = HashMap::new();
        // Create initial state
        inventory.insert(Token::USDC::new(), BigDecimal::from(1000));
        let arms = create_bandit_arms();
        for arm in arms.iter() {
            mean_gain.insert(arm.clone(), BigDecimal::zero());
            rewards.insert(arm.clone(), (BigDecimal::zero(), BigDecimal::zero()));
        }
        Bandit {
            arms: arms.clone(),
            rewards: rewards.clone(),
            mean_gain: mean_gain.clone(),
            inventory: inventory.clone(),
        }
    }
    // Takes as input a gain obtained by a 3-hop swap starting and ending in USDC, denominated in USDC
    pub fn tick(&mut self, gain: BigDecimal) {
        let arm = self.get_arm(gain);
        let expected_gain = self.mean_gain.get(&arm).unwrap();
        if self.rewards.len() >= 111 {
            if expected_gain.to_f64().unwrap() > 0.0 {
                self.trade()
            }
        }
        else {
            self.trade()
        }

    }
    fn get_arm(&mut self, gain: BigDecimal) -> BigDecimal {
        for arm in self.arms.iter() {
            if gain <= *arm {
                let out = arm.clone();
                return out
            }
        }
        let arm = self.arms.last().unwrap();
        arm.clone()
    }
    fn trade(&mut self) {
        // Placeholder for trading logic
    }

}

// Get evenly spaced vector mapping discrete bins of percentage gains of an arb opportunity
pub fn create_bandit_arms() -> Vec<BigDecimal> {
    let start = BigDecimal::from_str("0.0001").unwrap();
    let end = BigDecimal::from_str("0.03").unwrap();
    let num_elements = 50;

    // Calculate the step size
    let step = (end - &start) / BigDecimal::from(num_elements.to_u32().unwrap() - 1);
    // Fill vector with
    let mut out_vals: Vec<BigDecimal> = Vec::with_capacity(num_elements);
    for i in 0..num_elements {
        let value = &start + &step * BigDecimal::from(i.to_u32().unwrap());
        out_vals.push(value);
    }
    println!("{:?}", out_vals);
    out_vals
}
fn main() {
    create_bandit_arms();
}