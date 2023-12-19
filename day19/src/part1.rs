use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(Workflow::parse)
        .map(|workflow| (workflow.id, workflow))
        .collect::<HashMap<_, _>>();

    parts
        .lines()
        .map(Part::parse)
        .filter(|part| part.is_accepted(&workflows))
        .map(|part| part.ratings_total())
        .sum()
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

struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn parse(line: &str) -> Self {
        let mut categories = line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|category| category.split_once('=').unwrap());

        let (x_label, x) = categories.next().unwrap();
        debug_assert!(x_label == "x");
        let x = x.parse().unwrap();

        let (m_label, m) = categories.next().unwrap();
        debug_assert!(m_label == "m");
        let m = m.parse().unwrap();

        let (a_label, a) = categories.next().unwrap();
        debug_assert!(a_label == "a");
        let a = a.parse().unwrap();

        let (s_label, s) = categories.next().unwrap();
        debug_assert!(s_label == "s");
        let s = s.parse().unwrap();

        Self { x, m, a, s }
    }

    fn is_accepted(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut next_workflow = "in";
        while let Some(workflow) = workflows.get(next_workflow) {
            for rule in &workflow.rules {
                if match rule.condition {
                    Condition::None => true,
                    Condition::XGreaterThan(value) => self.x > value,
                    Condition::MGreaterThan(value) => self.m > value,
                    Condition::AGreaterThan(value) => self.a > value,
                    Condition::SGreaterThan(value) => self.s > value,
                    Condition::XLessThan(value) => self.x < value,
                    Condition::MLessThan(value) => self.m < value,
                    Condition::ALessThan(value) => self.a < value,
                    Condition::SLessThan(value) => self.s < value,
                } {
                    match rule.action {
                        Action::Accept => return true,
                        Action::Reject => return false,
                        Action::NextWorkflow(id) => {
                            next_workflow = id;
                            break;
                        }
                    }
                }
            }
        }
        unreachable!("Workflow not found: {}", next_workflow)
    }

    fn ratings_total(&self) -> usize {
        self.x as usize + self.m as usize + self.a as usize + self.s as usize
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
        assert_eq!(result, 19114);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 418498);
    }
}
