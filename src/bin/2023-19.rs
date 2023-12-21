use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Index<Category> for Part {
    type Output = u64;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl IndexMut<Category> for Part {
    fn index_mut(&mut self, category: Category) -> &mut Self::Output {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    GreaterThan(Category, u64, Decision),
    LessThan(Category, u64, Decision),
    Unconditional(Decision),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Decision {
    SendTo(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_workflow(input: &str) -> Workflow {
    let (name, rules) = input.split_once('{').unwrap();
    let name = name.trim();
    let rules = rules
        .trim_end_matches('}')
        .split(',')
        .map(parse_rule)
        .collect();
    Workflow {
        name: name.to_string(),
        rules,
    }
}

fn parse_rule(input: &str) -> Rule {
    match input {
        "A" => Rule::Unconditional(Decision::Accept),
        "R" => Rule::Unconditional(Decision::Reject),
        workflow_name if !workflow_name.contains(':') => {
            Rule::Unconditional(Decision::SendTo(workflow_name.to_string()))
        }
        _ => {
            let (condition, decision) = input.split_once(':').unwrap();
            let (category, operator, value) = (&condition[..1], &condition[1..2], &condition[2..]);
            let category = match category {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                _ => unreachable!(),
            };
            let value = value.parse().unwrap();
            let decision = match decision {
                "A" => Decision::Accept,
                "R" => Decision::Reject,
                _ => Decision::SendTo(decision.to_string()),
            };
            match operator {
                "<" => Rule::LessThan(category, value, decision),
                ">" => Rule::GreaterThan(category, value, decision),
                _ => unreachable!(),
            }
        }
    }
}

fn parse_part(input: &str) -> Part {
    let mut iter = input
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|s| s.split_once('=').unwrap().1);
    Part {
        x: iter.next().unwrap().parse().unwrap(),
        m: iter.next().unwrap().parse().unwrap(),
        a: iter.next().unwrap().parse().unwrap(),
        s: iter.next().unwrap().parse().unwrap(),
    }
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow).collect();
    let parts = parts.lines().map(parse_part).collect();
    (workflows, parts)
}

fn organize(part: &Part, name_to_workflow: &HashMap<String, &Workflow>) -> Decision {
    let mut workflow_name = "in".to_string();
    loop {
        let workflow = name_to_workflow.get(&workflow_name).unwrap();
        for rule in workflow.rules.iter() {
            let decision = match rule.clone() {
                Rule::Unconditional(decision) => Some(decision),
                Rule::GreaterThan(category, value, decision) => {
                    if part[category] > value {
                        Some(decision)
                    } else {
                        None
                    }
                }
                Rule::LessThan(category, value, decision) => {
                    if part[category] < value {
                        Some(decision)
                    } else {
                        None
                    }
                }
            };
            match decision {
                Some(Decision::SendTo(name)) => {
                    workflow_name = name;
                    break;
                }
                Some(decision) => {
                    return decision;
                }
                None => (),
            }
        }
    }
}

fn part_one(input: &(Vec<Workflow>, Vec<Part>)) -> u64 {
    let (workflows, parts) = input;
    let name_to_workflow = workflows
        .iter()
        .map(|workflow| (workflow.name.to_string(), workflow))
        .collect::<HashMap<_, _>>();
    parts
        .iter()
        .filter(|part| organize(part, &name_to_workflow) == Decision::Accept)
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn part_two(workflows: &Vec<Workflow>) -> u64 {
    let name_to_workflow = workflows
        .iter()
        .map(|workflow| (workflow.name.to_string(), workflow))
        .collect::<HashMap<_, _>>();
    let mut stack = vec![(
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
        Decision::SendTo("in".to_string()),
        0,
    )];
    let mut accepted = Vec::new();
    while let Some(range) = stack.pop() {
        let (x, m, a, s, decision, rule_key) = range;

        if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
            continue;
        }

        match decision {
            Decision::Accept => {
                accepted.push((x, m, a, s));
                continue;
            }
            Decision::Reject => continue,
            Decision::SendTo(workflow_name) => {
                let workflow = name_to_workflow.get(&workflow_name).unwrap();
                let rule = &workflow.rules[rule_key];
                match rule.clone() {
                    Rule::Unconditional(decision) => {
                        stack.push((x, m, a, s, decision, 0));
                    }
                    Rule::GreaterThan(category, value, decision) => match category {
                        Category::X => {
                            stack.push(((value + 1, x.1), m, a, s, decision.clone(), 0));
                            stack.push((
                                (x.0, value),
                                m,
                                a,
                                s,
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                        Category::M => {
                            stack.push((x, (value + 1, m.1), a, s, decision.clone(), 0));
                            stack.push((
                                x,
                                (m.0, value),
                                a,
                                s,
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                        Category::A => {
                            stack.push((x, m, (value + 1, a.1), s, decision.clone(), 0));
                            stack.push((
                                x,
                                m,
                                (a.0, value),
                                s,
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                        Category::S => {
                            stack.push((x, m, a, (value + 1, s.1), decision.clone(), 0));
                            stack.push((
                                x,
                                m,
                                a,
                                (s.0, value),
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                    },
                    Rule::LessThan(category, value, decision) => match category {
                        Category::X => {
                            stack.push(((x.0, value - 1), m, a, s, decision.clone(), 0));
                            stack.push((
                                (value, x.1),
                                m,
                                a,
                                s,
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                        Category::M => {
                            stack.push((x, (m.0, value - 1), a, s, decision.clone(), 0));
                            stack.push((
                                x,
                                (value, m.1),
                                a,
                                s,
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                        Category::A => {
                            stack.push((x, m, (a.0, value - 1), s, decision.clone(), 0));
                            stack.push((
                                x,
                                m,
                                (value, a.1),
                                s,
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                        Category::S => {
                            stack.push((x, m, a, (s.0, value - 1), decision.clone(), 0));
                            stack.push((
                                x,
                                m,
                                a,
                                (value, s.1),
                                Decision::SendTo(workflow_name),
                                rule_key + 1,
                            ));
                        }
                    },
                };
            }
        }
    }

    accepted
        .iter()
        .map(|(x, m, a, s)| {
            let x = x.1 - x.0 + 1;
            let m = m.1 - m.0 + 1;
            let a = a.1 - a.0 + 1;
            let s = s.1 - s.0 + 1;
            x * m * a * s
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input/2023-19.txt"));

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input.0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 19114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE).0), 167409079868000);
    }
}
