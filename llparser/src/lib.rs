use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
}

#[derive(Debug, PartialEq)]
pub struct Grammar {
    rules: HashMap<String, Vec<Vec<Symbol>>>,
}

impl Grammar {
    pub fn is_ll(&self) -> bool {
        for (left, right) in &self.rules {
            if self.rule_starts_with(right, &Symbol::NonTerminal(left.to_string())) {
                return false;
            }
        }
        true
    }

    fn rule_starts_with(&self, rule: &Vec<Vec<Symbol>>, symbol: &Symbol) -> bool {
        for rule in rule {
            if rule.first().unwrap() == symbol {
                return true;
            } else if let Symbol::NonTerminal(sym) = &rule[0] {
                if self.rule_starts_with(self.rules.get(sym).unwrap(), symbol) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use macros::grammar;

    use super::*;

    #[test]
    fn is_ll() {
        let llgrammar = Grammar {
            rules: HashMap::from([
                ("P".into(), vec![vec![Symbol::NonTerminal("E".into())]]),
                (
                    "E".into(),
                    vec![vec![
                        Symbol::NonTerminal("T".into()),
                        Symbol::NonTerminal("E'".into()),
                    ]],
                ),
                (
                    "E'".into(),
                    vec![
                        vec![
                            Symbol::Terminal("+".into()),
                            Symbol::NonTerminal("T".into()),
                            Symbol::NonTerminal("E'".into()),
                        ],
                        vec![Symbol::Terminal(String::new())],
                    ],
                ),
                (
                    "T".into(),
                    vec![
                        vec![Symbol::Terminal("ident".into())],
                        vec![Symbol::Terminal("int".into())],
                    ],
                ),
            ]),
        };
        assert!(llgrammar.is_ll());

        let nonllgrammar = Grammar {
            rules: HashMap::from([
                ("P".into(), vec![vec![Symbol::NonTerminal("E".into())]]),
                (
                    "E".into(),
                    vec![
                        vec![Symbol::NonTerminal("T".into())],
                        vec![
                            Symbol::NonTerminal("E".into()),
                            Symbol::Terminal("+".into()),
                            Symbol::NonTerminal("T".into()),
                        ],
                    ],
                ),
                (
                    "T".into(),
                    vec![
                        vec![Symbol::Terminal("ident".into())],
                        vec![Symbol::Terminal("int".into())],
                    ],
                ),
            ]),
        };
        assert!(!nonllgrammar.is_ll());
    }

    #[test]
    fn macros() {
        let nonllgrammar = grammar! {
            "P" -> "E";
            "E" -> "T";
            "E" -> "E + T";
            "T" -> "ident";
            "T" -> "int";
        };
        let nonllgrammar2 = Grammar {
            rules: HashMap::from([
                ("P".into(), vec![vec![Symbol::NonTerminal("E".into())]]),
                (
                    "E".into(),
                    vec![
                        vec![Symbol::NonTerminal("T".into())],
                        vec![
                            Symbol::NonTerminal("E".into()),
                            Symbol::Terminal("+".into()),
                            Symbol::NonTerminal("T".into()),
                        ],
                    ],
                ),
                (
                    "T".into(),
                    vec![
                        vec![Symbol::Terminal("ident".into())],
                        vec![Symbol::Terminal("int".into())],
                    ],
                ),
            ]),
        };
        assert_eq!(nonllgrammar, nonllgrammar2);
    }
}
