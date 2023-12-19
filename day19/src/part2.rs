use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

pub fn solve(input: &str) -> usize {
    let (workflows, _parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(Workflow::parse)
        .map(|workflow| (workflow.id, workflow))
        .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::from([State::new()]);
    let mut result = 0;

    while let Some(state) = queue.pop_front() {
        workflows
            .get(state.workflow_id)
            .unwrap()
            .rules
            .iter()
            .try_fold(state, |state, rule| {
                let (pass, fail) = state.split(&rule.condition);
                if let Some(mut state) = pass {
                    match rule.action {
                        Action::Accept => result += state.combinations(),
                        Action::Reject => (),
                        Action::NextWorkflow(id) => {
                            state.workflow_id = id;
                            queue.push_back(state);
                        }
                    }
                }
                fail
            });
    }

    result
}

enum Condition {
    None,
    XGreaterThan(u16),
    MGreaterThan(u16),
    AGreaterThan(u16),
    SGreaterThan(u16),
    XLessThan(u16),
    MLessThan(u16),
    ALessThan(u16),
    SLessThan(u16),
}

enum Action<'a> {
    Accept,
    Reject,
    NextWorkflow(&'a str),
}

impl<'a> Action<'a> {
    fn parse(action: &'a str) -> Self {
        match action {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::NextWorkflow(action),
        }
    }
}

struct Rule<'a> {
    condition: Condition,
    action: Action<'a>,
}

struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse(line: &'a str) -> Self {
        let (id, rules) = line.trim_end_matches('}').split_once('{').unwrap();
        let rules = rules
            .split(',')
            .map(|rule| {
                if let Some((condition, action)) = rule.split_once(':') {
                    let (lhs, rhs) = condition.split_at(2);
                    let value = rhs.parse().expect("condition value could not be read");
                    let condition = match lhs {
                        "x>" => Condition::XGreaterThan(value),
                        "m>" => Condition::MGreaterThan(value),
                        "a>" => Condition::AGreaterThan(value),
                        "s>" => Condition::SGreaterThan(value),
                        "x<" => Condition::XLessThan(value),
                        "m<" => Condition::MLessThan(value),
                        "a<" => Condition::ALessThan(value),
                        "s<" => Condition::SLessThan(value),
                        _ => panic!("unknown condition: {}", condition),
                    };
                    Rule {
                        condition,
                        action: Action::parse(action),
                    }
                } else {
                    Rule {
                        condition: Condition::None,
                        action: Action::parse(rule),
                    }
                }
            })
            .collect::<Vec<_>>();
        Self { id, rules }
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
    workflow_id: &'a str,
}

impl<'a> State<'a> {
    fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
            workflow_id: "in",
        }
    }

    fn combinations(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn split(self, condition: &Condition) -> (Option<Self>, Option<Self>) {
        match *condition {
            Condition::None => (Some(self), None),
            Condition::XGreaterThan(value) => {
                if self.x.start > value {
                    (Some(self), None)
                } else if self.x.end - 1 <= value {
                    (None, Some(self))
                } else {
                    (
                        Some(Self {
                            x: value + 1..self.x.end,
                            ..self.clone()
                        }),
                        Some(Self {
                            x: self.x.start..value + 1,
                            ..self
                        }),
                    )
                }
            }
            Condition::MGreaterThan(value) => {
                if self.m.start > value {
                    (Some(self), None)
                } else if self.m.end - 1 <= value {
                    (None, Some(self))
                } else {
                    (
                        Some(Self {
                            m: value + 1..self.m.end,
                            ..self.clone()
                        }),
                        Some(Self {
                            m: self.m.start..value + 1,
                            ..self
                        }),
                    )
                }
            }
            Condition::AGreaterThan(value) => {
                if self.a.start > value {
                    (Some(self), None)
                } else if self.a.end - 1 <= value {
                    (None, Some(self))
                } else {
                    (
                        Some(Self {
                            a: value + 1..self.a.end,
                            ..self.clone()
                        }),
                        Some(Self {
                            a: self.a.start..value + 1,
                            ..self
                        }),
                    )
                }
            }
            Condition::SGreaterThan(value) => {
                if self.s.start > value {
                    (Some(self), None)
                } else if self.s.end - 1 <= value {
                    (None, Some(self))
                } else {
                    (
                        Some(Self {
                            s: value + 1..self.s.end,
                            ..self.clone()
                        }),
                        Some(Self {
                            s: self.s.start..value + 1,
                            ..self
                        }),
                    )
                }
            }
            Condition::XLessThan(value) => {
                if self.x.start >= value {
                    (None, Some(self))
                } else if self.x.end <= value {
                    (Some(self), None)
                } else {
                    (
                        Some(Self {
                            x: self.x.start..value,
                            ..self.clone()
                        }),
                        Some(Self {
                            x: value..self.x.end,
                            ..self
                        }),
                    )
                }
            }
            Condition::MLessThan(value) => {
                if self.m.start >= value {
                    (None, Some(self))
                } else if self.m.end <= value {
                    (Some(self), None)
                } else {
                    (
                        Some(Self {
                            m: self.m.start..value,
                            ..self.clone()
                        }),
                        Some(Self {
                            m: value..self.m.end,
                            ..self
                        }),
                    )
                }
            }
            Condition::ALessThan(value) => {
                if self.a.start >= value {
                    (None, Some(self))
                } else if self.a.end <= value {
                    (Some(self), None)
                } else {
                    (
                        Some(Self {
                            a: self.a.start..value,
                            ..self.clone()
                        }),
                        Some(Self {
                            a: value..self.a.end,
                            ..self
                        }),
                    )
                }
            }
            Condition::SLessThan(value) => {
                if self.s.start >= value {
                    (None, Some(self))
                } else if self.s.end <= value {
                    (Some(self), None)
                } else {
                    (
                        Some(Self {
                            s: self.s.start..value,
                            ..self.clone()
                        }),
                        Some(Self {
                            s: value..self.s.end,
                            ..self
                        }),
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 123331556462603);
    }
}
