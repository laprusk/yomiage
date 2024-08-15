use crate::generate::generate;

#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Config {
    pub min_digit: u32,
    pub max_digit: u32,
    pub length: u32,
    pub subtractions: u32,
    pub allow_negative: bool,
}

impl Config {
    pub fn check(&self) -> Result<(), String> {
        if self.min_digit < 1 {
            return Err("min_digit must be greater than or equal to 1".to_string());
        }
        if self.max_digit < self.min_digit {
            return Err("max_digit must be greater than or equal to min_digit".to_string());
        }
        if self.length < 1 {
            return Err("length must be greater than or equal to 1".to_string());
        }
        if !self.allow_negative && self.subtractions > self.length / 2 {
            return Err("subtracts must be less than or equal to length / 2 when allow_negative is true".to_string());
        }
        if self.subtractions >= self.length {
            return Err("subtracts must be less than length".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub problem: Vec<i128>,
    pub answer: i128,
    pub config: Config,
}

impl Problem {
    pub fn new(config: Config) -> Result<Self, String> {
        config.check()?;

        let problem = generate(config);
        let answer = problem.iter().sum();

        Ok(Self {
            problem,
            answer,
            config,
        })
    }

    pub fn script_meta(&self) -> String {
        format!(
            "{}桁から{}桁、{}口、{}です。ねがいましては。",
            self.config.min_digit,
            self.config.max_digit,
            self.config.length,
            if self.config.subtractions == 0 {
                "加算"
            } else {
                "加減算"
            }
        )
    }

    pub fn script_problem(&self) -> String {
        let mut script = String::new();
        let mut prev_op = 1;

        for num in &self.problem {
            let op = if *num >= 0 { 1 } else { 0 };
            if op != prev_op {
                script.push_str(if op == 1 { "加えて" } else { "引いては" });
            }
            script += format!("{}円", num.abs()).as_str();
            if num == self.problem.iter().last().unwrap() {
                script.push_str("では。");
            } else {
                script.push_str("なり、");
            }
            prev_op = op;
        }

        script
    }

    pub fn script_answer(&self) -> String {
        format!("その答え、{}{}円です。",
            if self.answer >= 0 { "" } else { "マイナス" },
            self.answer.abs()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_yomiage_problem_new() {
        let config = Config {
            min_digit: 1,
            max_digit: 2,
            length: 10,
            subtractions: 3,
            allow_negative: false,
        };
        let minimum = 10_i128.pow(config.min_digit - 1);
        let maximum = 10_i128.pow(config.max_digit) - 1;
        for _ in 0..10 {
            let yp = Problem::new(config).unwrap();
            println!("{:?}", yp.problem);
            println!("{:?}", yp.answer);
            assert_eq!(yp.problem.len(), config.length as usize);
            assert_eq!(yp.answer, yp.problem.iter().sum());
            assert!(yp.problem.iter().map(|&x| x.abs()).min().unwrap() >= minimum);
            assert!(yp.problem.iter().map(|&x| x.abs()).max().unwrap() <= maximum);
        }
    }

    #[test]
    fn test_yomiage_problem_script_meta() {
        let config = Config {
            min_digit: 1,
            max_digit: 2,
            length: 7,
            subtractions: 0,
            allow_negative: false,
        };
        let yp = Problem::new(config).unwrap();
        let script_meta = yp.script_meta();
        println!("{}", script_meta);

        let config = Config {
            min_digit: 7,
            max_digit: 12,
            length: 10,
            subtractions: 3,
            allow_negative: false,
        };
        let yp = Problem::new(config).unwrap();
        let script_meta = yp.script_meta();
        println!("{}", script_meta);
    }

    #[test]
    fn test_yomiage_problem_script_problem() {
        let config = Config {
            min_digit: 3,
            max_digit: 6,
            length: 10,
            subtractions: 3,
            allow_negative: false,
        };
        let yp = Problem::new(config).unwrap();
        let script_problem = yp.script_problem();
        println!("{}", script_problem);

        let config = Config {
            min_digit: 7,
            max_digit: 12,
            length: 10,
            subtractions: 3,
            allow_negative: false,
        };
        let yp = Problem::new(config).unwrap();
        let script_problem = yp.script_problem();
        println!("{}", script_problem);
    }

    #[test]
    fn test_yomiage_problem_script_answer() {
        let config = Config {
            min_digit: 1,
            max_digit: 2,
            length: 7,
            subtractions: 0,
            allow_negative: false,
        };
        let yp = Problem::new(config).unwrap();
        let script_answer = yp.script_answer();
        println!("{}", script_answer);

        let config = Config {
            min_digit: 7,
            max_digit: 12,
            length: 10,
            subtractions: 9,
            allow_negative: true,
        };
        let yp = Problem::new(config).unwrap();
        let script_answer = yp.script_answer();
        println!("{}", script_answer);
    }

    #[test]
    fn test_yomiage_config_check() {
        // OK
        let config = Config {
            min_digit: 3,
            max_digit: 6,
            length: 10,
            subtractions: 5,
            allow_negative: false,
        };
        assert!(config.check().is_ok());

        // min_digit < 1
        let config = Config {
            min_digit: 0,
            max_digit: 6,
            length: 10,
            subtractions: 3,
            allow_negative: false,
        };
        assert!(config.check().is_err());

        // max_digit < min_digit
        let config = Config {
            min_digit: 3,
            max_digit: 1,
            length: 10,
            subtractions: 3,
            allow_negative: false,
        };
        assert!(config.check().is_err());

        // length < 1
        let config = Config {
            min_digit: 3,
            max_digit: 6,
            length: 0,
            subtractions: 3,
            allow_negative: false,
        };
        assert!(config.check().is_err());

        // subtractions >= length
        let config = Config {
            min_digit: 3,
            max_digit: 6,
            length: 10,
            subtractions: 6,
            allow_negative: false,
        };
        assert!(config.check().is_err());

        // subtractions >= length
        let config = Config {
            min_digit: 3,
            max_digit: 6,
            length: 10,
            subtractions: 10,
            allow_negative: true,
        };
        assert!(config.check().is_err());

    }
}
