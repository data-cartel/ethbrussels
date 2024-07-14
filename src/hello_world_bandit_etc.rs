use std::collections::HashMap;
use std::ops::{Add, Mul, Sub, Div};
use std::str::FromStr;

use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use rand::distributions::Distribution;
use crate::token::*;

#[derive(Debug)]
struct Bandit {
    arms: Vec<BigDecimal>,
    rewards: HashMap<BigDecimal,(BigDecimal, BigDecimal)>,
    mean_gain: HashMap<BigDecimal, BigDecimal>,
    inventory: HashMap<Token, BigDecimal>,
    trade_size: BigDecimal,
}

impl Bandit {
    pub fn new() -> Self {
        let trade_size = BigDecimal::from_str("0.1").unwrap();
        let mut rewards = HashMap::new();
        let mut mean_gain = HashMap::new();
        let mut inventory = HashMap::new();
        // Create initial state
        inventory.insert(Token::NEAR(NEAR::new()), BigDecimal::from(200));
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
            trade_size: trade_size.clone(),
        }
    }
    // Takes as input a percentage gain obtained by a 3-hop swap starting and ending in NEAR
    // denominated in NEAR
    pub fn tick(&mut self, gain: BigDecimal) {
        let arm = self.get_arm(gain.clone());
        let expected_gain = self.mean_gain.get(&arm).unwrap();
        if self.rewards.len() >= 111 {
            if expected_gain.to_f64().unwrap() > 0.0 {
                let mut near_owned = self.inventory.get(&Token::NEAR(NEAR::new())).unwrap();
                let pnl = self.trade(near_owned.mul(&self.trade_size), gain.clone());
                &near_owned.add(pnl);
            }
        }
        else {
            let mut near_owned = self.inventory.get(&Token::NEAR(NEAR::new())).unwrap();
            let pnl = self.trade(near_owned.mul(&self.trade_size), gain);
            near_owned = &near_owned.add(pnl);
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
    fn trade(&self, amount: BigDecimal, expected_gain: BigDecimal) -> BigDecimal {
        // Placeholder for trading logic
        let scale = 1.0; // 'xm' parameter in the Pareto distribution
        let shape = 1.5; // 'alpha' parameter in the Pareto distribution
        // Create a Pareto distribution with the given scale and shape parameters
        let pareto = Pareto::new(scale, shape).unwrap();

        // Create a random number generator
        let mut rng = rand::thread_rng();
        let gas = BigDecimal::from_str(& pareto.sample(&mut rng).to_string()).unwrap();
        amount.mul(expected_gain).sub(gas)
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
    out_vals
}

mod test {
    use std::str::FromStr;
    use bigdecimal::BigDecimal;
    use crate::hello_world_bandit_etc::{Bandit, create_bandit_arms};
    #[test]
    fn test_create_bandit_arms() {
        let arms = create_bandit_arms();
        println!("{:?}", arms);
        assert_eq!(arms.len(), 50);

    }
    #[test]
    fn test_tick() {
        let mut bandit = Bandit::new();
        bandit.tick(BigDecimal::from_str("0.0001").unwrap());



    }


}