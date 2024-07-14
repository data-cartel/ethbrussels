use std::str::FromStr;

use bigdecimal::BigDecimal;
use rand_distr::{Pareto, Distribution};

use crate::token::*;

mod test {
    use std::str::FromStr;
    use bigdecimal::BigDecimal;
    use super::{Bandit, create_bandit_arms};

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
