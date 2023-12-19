use std::{collections::HashMap, ops::Index};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Index<Category> for Part {
    type Output = u32;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    GreaterThan(Category, u32, Decision),
    LessThan(Category, u32, Decision),
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

fn part_one(input: &(Vec<Workflow>, Vec<Part>)) -> u32 {
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

fn part_two(input: &(Vec<Workflow>, Vec<Part>)) -> u32 {
    0
}

fn main() {
    let input = parse(include_str!("../input/2023-19.txt"));

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
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

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(&parse(EXAMPLE)), 952408144115);
    // }
}
