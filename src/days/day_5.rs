use std::{cmp::Ordering, collections::HashMap};

enum Order {
    Before,
    After,
}

type RulePages = (HashMap<String, Vec<(Order, String)>>, Vec<String>);

fn get_rules_pages_from_input(input: String) -> RulePages {
    let splitted = input.split("\n\n").collect::<Vec<_>>();
    let mut rules: HashMap<String, Vec<(Order, String)>> = HashMap::new();
    for rule in splitted[0].split('\n') {
        let rule = rule.split('|').collect::<Vec<_>>();
        let a = rule[0];
        let b = rule[1];

        let a_rules = vec![(Order::Before, b.to_string())];
        if let Some(a_rules_hm) = rules.get_mut(a) {
            a_rules_hm.extend(a_rules);
        } else {
            rules.insert(a.to_string(), a_rules);
        }

        let b_rules = vec![(Order::After, a.to_string())];
        if let Some(b_rules_hm) = rules.get_mut(b) {
            b_rules_hm.extend(b_rules);
        } else {
            rules.insert(b.to_string(), b_rules);
        }
    }
    (
        rules,
        splitted[1]
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|l| l.to_string())
            .collect(),
    )
}

pub fn first(input: String) -> String {
    let (rules, lines) = get_rules_pages_from_input(input);
    let mut sum: u32 = 0;
    for pages in lines {
        let pages = pages.split(',').collect::<Vec<_>>();
        let mut parsed_pages: Vec<&str> = Vec::with_capacity(pages.len());
        'out: for page in &pages {
            if let Some(rules) = rules.get(*page) {
                for (order, b) in rules {
                    if parsed_pages.iter().any(|p| *p == *b) && !matches!(order, Order::After) {
                        break 'out;
                    }
                }
            }
            parsed_pages.push(page);
        }
        if parsed_pages.len() == pages.len() {
            let middle = parsed_pages[parsed_pages.len() / 2].parse::<u32>().unwrap();
            sum += middle;
        }
    }
    sum.to_string()
}

pub fn second(input: String) -> String {
    let (rules, lines) = get_rules_pages_from_input(input);
    let mut sum: u32 = 0;
    for pages in lines {
        let mut pages = pages.split(',').collect::<Vec<_>>();
        let mut parsed_pages: Vec<&str> = Vec::with_capacity(pages.len());
        'out: for page in &pages {
            if let Some(rules) = rules.get(*page) {
                for (order, b) in rules {
                    if parsed_pages.iter().any(|p| *p == *b) && !matches!(order, Order::After) {
                        break 'out;
                    }
                }
            }
            parsed_pages.push(page);
        }
        if parsed_pages.len() != pages.len() {
            pages.sort_by(|a, b| {
                if let Some(rules) = rules.get(*a) {
                    if let Some(rule) = rules.iter().find(|r| r.1 == *b) {
                        return match rule.0 {
                            Order::Before => Ordering::Greater,
                            Order::After => Ordering::Less,
                        };
                    }
                }
                Ordering::Equal
            });
            let middle = pages[pages.len() / 2].parse::<u32>().unwrap();
            sum += middle;
        }
    }
    sum.to_string()
}
