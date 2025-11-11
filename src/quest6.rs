pub mod part1 {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        /*
        What is the total number of possible novice-mentor pairs in the sword fighting category?

        Each letter means a different profession:
         A a means sword fighting, B b means archery and C c specialises in magic. Any knight who matches
         the novice's letter and appears earlier in the list can mentor them. The captain explains
         that this procedure is just a draft method, and first, he wants to check only the
         sword fighting category. For each novice in this profession a, determine the total number of possible mentors
         A, and calculate the overall number of possible novice-mentor pairs in this category.
         */
        let mut file =
            File::open(Path::new("notes/quest6/part1")).expect("Input file does not exist");
        let mut contents_string = String::new();
        file.read_to_string(&mut contents_string)
            .expect("could not read file");
        let sum_of_all_pairs: i64 = contents_string
            .lines()
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                let mut num_pairs: i64 = 0;
                for i in 0..line.len() {
                    let c = chars[i];
                    if c.is_uppercase() {
                        let c_lower = c.to_lowercase().next().unwrap();
                        if c_lower != 'a' {
                            continue;
                        }

                        let pairs = chars[i + 1..]
                            .iter()
                            .filter(|&&the_char| c_lower == the_char)
                            .count();
                        num_pairs += pairs as i64
                    }
                }
                println!("Number of pairs: {}", num_pairs);
                return num_pairs;
            })
            .sum();
    }
}

pub mod part2 {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        let mut file =
            File::open(Path::new("notes/quest6/part2")).expect("Input file does not exist");
        let mut contents_string = String::new();
        file.read_to_string(&mut contents_string)
            .expect("could not read file");
        let sum_of_all_pairs: i64 = contents_string
            .lines()
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                let mut num_pairs: i64 = 0;
                for i in 0..line.len() {
                    let c = chars[i];
                    if c.is_uppercase() {
                        let c_lower = c.to_lowercase().next().unwrap();

                        let pairs = chars[i + 1..]
                            .iter()
                            .filter(|&&the_char| c_lower == the_char)
                            .count();
                        num_pairs += pairs as i64
                    }
                }
                println!("Number of pairs: {}", num_pairs);
                return num_pairs;
            })
            .sum();
    }
}

pub mod part3 {
    use std::cmp::{max, min};

    pub fn part() {
        use std::fs::File;
        use std::io::Read;
        use std::path::Path;

        let mut file =
            File::open(Path::new("notes/quest6/part3")).expect("Input file does not exist");
        let mut contents_string = String::new();
        file.read_to_string(&mut contents_string)
            .expect("could not read file");

        contents_string.lines().for_each(|line| {
            let repeat = 1000;
            let distance_limit = 1000;
            let full_line = line.repeat(repeat);

            let chars: Vec<char> = full_line.chars().collect();
            let mut num_pairs: i64 = 0;
            for i in 0..full_line.len() {
                let c = chars[i];
                if c.is_uppercase() {
                    let c_lower = c.to_lowercase().next().unwrap();

                    let right = chars[i + 1..min(i + 1 + distance_limit, full_line.len())].to_vec();
                    let right_pairs = right
                        .iter()
                        .filter(|&&the_char| c_lower == the_char)
                        .count();
                    num_pairs += right_pairs as i64;


                    let left = chars[max(0, i.saturating_sub(distance_limit))..i].to_vec();
                    let left_pairs = left
                        .iter()
                        .filter(|&&the_char| c_lower == the_char)
                        .count();
                    num_pairs += left_pairs as i64;
                }
            }
            println!("Number of pairs: {}", num_pairs);
        });
    }
}
