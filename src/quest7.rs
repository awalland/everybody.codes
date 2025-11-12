pub mod part1 {
    use crate::common;
    use crate::quest7::Rule;

    pub fn part() {
        let contents_string = common::read_file("notes/quest7/part1");

        let mut line_iterator = contents_string.lines();

        let names: Vec<String> = line_iterator.next().unwrap().split(",").map(|s| s.to_owned()).collect();
        line_iterator.next();
        let rules: Vec<Rule> = line_iterator
            .map(|line| {
                let mut parts = line.split(" > ");
                let position = parts.next().unwrap().to_owned();
                let allowed_next: Vec<String> = parts.next().unwrap().split(",").map(|x| x.to_owned()).collect();

                Rule { position, allowed_next }
            })
            .collect();

        let allowed_name: Vec<String> = names
            .into_iter()
            .filter(|name| {
                for i in 0..name.len() - 1 {
                    let window = &name[i..i + 2];
                    let matches = &rules.iter().any(|rule| rule.matches((&window[0..1], &window[1..2])));
                    if !matches {
                        return false;
                    }
                }
                return true;
            })
            .collect();

        println!("Allowed name: {:?}", allowed_name)
    }
}

pub mod part2 {
    use crate::common;
    use crate::quest7::{Name, Rule};

    pub fn part() {
        let contents_string = common::read_file("notes/quest7/part2");

        let mut line_iterator = contents_string.lines();

        let mut names: Vec<Name> = vec![];
        for (i, name) in line_iterator.next().unwrap().split(",").enumerate() {
            names.push(Name {
                name: name.to_owned(),
                index: (i + 1) as i64,
            })
        }

        line_iterator.next();
        let rules: Vec<Rule> = line_iterator
            .map(|line| {
                let mut parts = line.split(" > ");
                let position = parts.next().unwrap().to_owned();
                let allowed_next: Vec<String> = parts.next().unwrap().split(",").map(|x| x.to_owned()).collect();

                Rule { position, allowed_next }
            })
            .collect();

        let allowed_name: Vec<Name> = names
            .into_iter()
            .filter(|n| {
                let name = &n.name;
                for i in 0..name.len() - 1 {
                    let window = &name[i..i + 2];
                    let matches = &rules.iter().any(|rule| rule.matches((&window[0..1], &window[1..2])));
                    if !matches {
                        return false;
                    }
                }
                return true;
            })
            .collect();

        let sum_of_valid: i64 = allowed_name.iter().map(|name| name.index).sum();

        println!("Allowed names ({}): {:?}", &allowed_name.len(), &allowed_name);
        println!("Sum of valid ids: {}", sum_of_valid)
    }
}

pub mod part3 {
    use crate::common;
    use crate::quest7::{Name, Rule};
    use std::collections::HashSet;
    use std::str::Lines;
    const MIN_LEN: usize = 7;
    const MAX_LEN: usize = 11;
    pub fn part() {
        /*
        Your task is to find all possible unique names that begin with these prefixes and follow the rules.
        Each name must have at least 7 and at most 11 letters.
        Someone announces that the break is about to end, so Veronica suggests finishing the experiment later, b
        ut seeing the result of this puzzle in your mind, you smile and say that it's not necessary.
        Then you write down a quite big number on the task card and the word 'instant :)' next to your result.
         */
        let contents_string = common::read_file("notes/quest7/part3");

        let mut line_iterator = contents_string.lines();
        let mut name_prefixes: Vec<Name> = parse_names(&mut line_iterator);
        line_iterator.next();
        let rules: Vec<Rule> = parse_rules(&mut line_iterator);

        let valid_name_prefixes: Vec<Name> = validate_names(&mut name_prefixes, &rules);

        let generated_names: HashSet<String> = valid_name_prefixes
            .iter()
            .map(|name| &name.name)
            .flat_map(|name| generate_names(name.to_owned(), &rules))
            .collect();

        println!("Generated names:{}", generated_names.len());
        // 9353503
    }

    fn generate_names(name: String, rules: &Vec<Rule>) -> Box<dyn Iterator<Item = String> + '_> {
        if name.len() >= MAX_LEN {
            return Box::new((MIN_LEN..=MAX_LEN).map(move |i| name[0..i].to_string()));
        }

        let last_letter = name.chars().last().unwrap();
        let next_letters: Vec<String> = rules
            .iter()
            .filter(|rule| rule.handles(&last_letter))
            .flat_map(|rule| &rule.allowed_next)
            .cloned()
            .collect();

        let current = if name.len() >= MIN_LEN {
            Some(name.clone())
        } else {
            None
        };

        let continuations = next_letters
            .into_iter()
            .map(move |letter| name.clone() + &letter)
            .flat_map(|new_name| generate_names(new_name, rules));

        Box::new(current.into_iter().chain(continuations))
    }

    fn validate_names(name_prefixes: &mut Vec<Name>, rules: &Vec<Rule>) -> Vec<Name> {
        name_prefixes
            .into_iter()
            .filter(|n| {
                let name = &n.name;
                for i in 0..name.len() - 1 {
                    let window = &name[i..i + 2];
                    let matches = &rules.iter().any(|rule| rule.matches((&window[0..1], &window[1..2])));
                    if !matches {
                        return false;
                    }
                }
                return true;
            })
            .map(|n| n.clone())
            .collect()
    }

    fn parse_names(line_iterator: &mut Lines) -> Vec<Name> {
        let mut name_prefixes: Vec<Name> = vec![];
        for (i, name) in line_iterator.next().unwrap().split(",").enumerate() {
            name_prefixes.push(Name {
                name: name.to_owned(),
                index: (i + 1) as i64,
            })
        }
        name_prefixes
    }

    fn parse_rules(line_iterator: &mut Lines) -> Vec<Rule> {
        line_iterator
            .map(|line| {
                let mut parts = line.split(" > ");
                let position = parts.next().unwrap().to_owned();
                let allowed_next: Vec<String> = parts.next().unwrap().split(",").map(|x| x.to_owned()).collect();

                Rule { position, allowed_next }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Name {
    name: String,
    index: i64,
}
struct Rule {
    position: String,
    allowed_next: Vec<String>,
}

impl Rule {
    fn matches(&self, (letter1, letter2): (&str, &str)) -> bool {
        if letter1 != self.position {
            return false;
        }

        self.allowed_next.contains(&letter2.to_owned())
    }

    fn handles(&self, position: &char) -> bool {
        *position == self.position.chars().next().unwrap()
    }
}
