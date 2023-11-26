use std::{collections::HashMap, fs, str::FromStr};

fn get_input() -> String {
    fs::read_to_string("input/day07.txt").unwrap()
}

fn solve_2(input: &str) -> usize {
    let mut rules: HashMap<String, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let rule = Rule::from_str(line).unwrap();
        rules.insert(rule.name, rule.links);
    }

    for (name, links) in rules.iter() {
        if name == "shiny gold" {
            return count(&links, &rules);
        }
    }
    0
}

fn count(links: &Vec<(String, usize)>, map: &HashMap<String, Vec<(String, usize)>>) -> usize {
    let mut sum = 0;
    links.iter().for_each(|(name, amount)| {
        sum += amount;
        if let Some(links) = map.get(name) {
            let sum2 = count(links, map);
            sum += amount * sum2;
        }
    });
    sum
}

fn solve_1(input: &str) -> usize {
    let mut rules: HashMap<String, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let rule = Rule::from_str(line).unwrap();
        rules.insert(rule.name, rule.links);
    }

    let mut counter = 0;
    for (_, links) in rules.iter() {
        if lookup("shiny gold", &links, &rules) {
            counter += 1;
        }
    }

    counter
}

fn lookup(
    needle: &str,
    links: &Vec<(String, usize)>,
    map: &HashMap<String, Vec<(String, usize)>>,
) -> bool {
    links.iter().any(|(name, _)| {
        if name == needle {
            return true;
        }

        if let Some(links) = map.get(name) {
            return lookup(needle, links, map);
        } else {
            false
        }
    })
}

pub fn answer_1() {
    let input = get_input();
    println!("day07 part1: {}", solve_1(&input));
}

pub fn answer_2() {
    let input = get_input();
    println!("day07 part2: {}", solve_2(&input));
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    links: Vec<(String, usize)>,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut links = Vec::new();
        let words: Vec<_> = s.split_whitespace().map(|s| s.to_string()).collect();

        let name = words[0..2].join(" ");

        let mut take = 0;
        let mut amount = 0;
        let mut curr_name = String::new();
        for word in words {
            if word.len() == 1 {
                if word.chars().all(|c| c.is_numeric()) {
                    amount = word.parse()?;
                    take = 2;
                }
                continue;
            }
            if take > 0 {
                curr_name.push_str(&word);
                take -= 1;
                if take == 1 {
                    curr_name.push_str(" ");
                }
                continue;
            }
            if take == 0 && curr_name.is_empty() == false {
                links.push((curr_name.clone(), amount));
                curr_name.clear();
                amount = 0;
            }
        }

        Ok(Rule { name, links })
    }
}

#[test]
fn test1() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";
    assert_eq!(4, solve_1(input));
}

#[test]
fn test2() {
    let input = "pale beige bags contain 4 wavy crimson bags, 5 shiny gold bags.
        dull purple bags contain 1 plaid green bag, 5 shiny gold bags.
        dim maroon bags contain 4 faded beige bags, 2 bright turquoise bags, 4 dull purple bags, 4 vibrant olive bags.
        dull tomato bags contain 1 shiny gold bag, 4 pale blue bags.
        striped green bags contain 2 striped fuchsia bags, 5 dull tomato bags, 2 posh crimson bags.
        drab plum bags contain 1 shiny gold bag, 5 vibrant tan bags, 3 light gold bags.
        shiny orange bags contain 5 vibrant chartreuse bags, 3 shiny gold bags.
        faded plum bags contain 5 shiny orange bags.
        shiny gold bags contain 1 vibrant chartreuse bag.
        dull red bags contain 1 dim coral bag, 1 faded plum bag, 4 plaid red bags.
        shiny green bags contain 1 vibrant chartreuse bag, 4 bright lavender bags, 3 wavy crimson bags, 4 dull red bags.
        mirrored white bags contain 2 pale gold bags, 3 shiny green bags, 5 striped white bags.";
    assert_eq!(11, solve_1(input));
}

#[test]
fn test3() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";
    assert_eq!(32, solve_2(input));
}

#[test]
fn test4() {
    let input = "shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.";
    assert_eq!(126, solve_2(input));
}
