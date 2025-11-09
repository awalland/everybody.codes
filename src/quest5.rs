use std::cmp::Ordering::Equal;
use std::cmp::{Ordering, max, min};
use std::fmt;

fn construct_fish_bone(numbers: Vec<i64>) -> Vec<Row> {
    let mut fish_bone: Vec<Row> = vec![Row::center(numbers[0])];
    for number in numbers.iter().skip(1) {
        let mut placed = false;
        for row in fish_bone.iter_mut() {
            if number < &row.center && row.left.is_none() {
                row.left = Some(*number);
                placed = true;
                break;
            }
            if number > &row.center && row.right.is_none() {
                row.right = Some(*number);
                placed = true;
                break;
            }
        }
        if !placed {
            fish_bone.push(Row::center(*number))
        }
    }
    fish_bone
}

pub fn construct_sword(line: &str) -> Sword {
    let mut parts = line.split(":");
    let id = parts.next().unwrap();
    let numbers = parts
        .flat_map(|nrs| nrs.split(","))
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    Sword {
        id: id.parse::<i64>().unwrap(),
        fish_bone: construct_fish_bone(numbers),
    }
}

pub struct Sword {
    id: i64,
    fish_bone: Vec<Row>,
}

impl Sword {
    fn spine(&self) -> i64 {
        self.fish_bone
            .iter()
            .fold(String::new(), |acc, item| {
                acc + item.center.to_string().as_str()
            })
            .parse::<i64>()
            .unwrap()
    }
}

impl fmt::Debug for Sword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sword")
            .field("id", &self.id)
            .field("spine", &self.spine())
            // .field("fish_bone", &self.fish_bone)
            .finish()
    }
}
impl Ord for Sword {
    fn cmp(&self, other: &Self) -> Ordering {
        let spine_cmp = self.spine().cmp(&other.spine());
        if spine_cmp != Equal {
            return spine_cmp;
        }

        let my_fish_bone = &self
            .fish_bone
            .iter()
            .map(|row| row.score())
            .collect::<Vec<i64>>();
        let other_fish_bone = &other
            .fish_bone
            .iter()
            .map(|row| row.score())
            .collect::<Vec<i64>>();

        for i in 0..min(my_fish_bone.len(), other_fish_bone.len()) {
            let comp_result = my_fish_bone[i].cmp(&other_fish_bone[i]);
            if comp_result != Equal {
                return comp_result;
            }
        }

        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for Sword {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.spine() == other.spine()
    }
}
impl Eq for Sword {}

#[derive(Debug)]
pub struct Row {
    left: Option<i64>,
    center: i64,
    right: Option<i64>,
}

impl Row {
    pub fn center(center: i64) -> Self {
        Self {
            left: None,
            center,
            right: None,
        }
    }

    pub fn score(&self) -> i64 {
        let score_string = format!(
            "{}{}{}",
            self.left.map(|x| x.to_string()).unwrap_or(String::new()),
            self.center.to_string().as_str(),
            self.right.map(|x| x.to_string()).unwrap_or(String::new())
        );

        score_string.parse::<i64>().unwrap()
    }
}

#[cfg(test)]
#[test]
fn check_ordering() {
    let sword1 = construct_sword("1:5,3,7,8,1,10,9,5,7,8");
    let sword2 = construct_sword("2:5,3,7,8,1,10,9,4,7,9");
    assert_eq!(sword1.cmp(&sword2), Ordering::Greater)
}

pub mod part1 {
    use crate::quest5::construct_fish_bone;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        /*
        To construct a fishbone from the listed numbers, the first element from the list should be
        placed as the first segment of the spine. Next, we add the subsequent numbers one by one, according to the following rules:

        Check all segments of the spine, starting from the top.
        If your number is less than the one on the spine segment and the left side of the segment is free - place it on the left.
        If your number is greater than the one on the spine segment and the right side of the segment is free - place it on the right.
        If no suitable place is found at any segment, create a new spine segment from your number and place it as the last one.
         */
        let mut file =
            File::open(Path::new("notes/quest5/part1")).expect("Input file does not exist");
        let mut numbers_string = String::new();
        file.read_to_string(&mut numbers_string)
            .expect("could not read file");

        let numbers = numbers_string
            .split(":")
            .skip(1)
            .flat_map(|nrs| nrs.split(","))
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let fish_bone = construct_fish_bone(numbers);

        println!("Fishbone: {:?}", fish_bone);
        let spine = fish_bone.iter().fold(String::new(), |acc, item| {
            acc + item.center.to_string().as_str()
        });

        println!("The spine is {}", spine)
        // 8572853643
    }
}

pub mod part2 {
    use crate::quest5::{Sword, construct_sword};
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        /*
        What is the quality difference between the best and the weakest sword on the given list?
        */
        let mut file =
            File::open(Path::new("notes/quest5/part2")).expect("Input file does not exist");
        let mut numbers_string = String::new();
        file.read_to_string(&mut numbers_string)
            .expect("could not read file");

        let mut swords = numbers_string
            .lines()
            .map(construct_sword)
            .collect::<Vec<Sword>>();

        swords.sort();

        // println!("Swords: {:?}", swords)

        let spine_diff = swords.last().unwrap().spine() - swords.first().unwrap().spine();
        println!("Spine diff: {}", spine_diff)
    }
}

pub mod part3 {
    use crate::quest5::{Sword, construct_sword};
    use std::cmp::Ordering;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        /*
        Sort all the swords from the best to the weakest and provide the resulting checksum.
         To calculate the checksum, you need to multiply the sword identifiers by their position
          on the sorted list, starting from 1, and sum the resulting values.
         */
        let mut file =
            File::open(Path::new("notes/quest5/part3")).expect("Input file does not exist");
        let mut numbers_string = String::new();
        file.read_to_string(&mut numbers_string)
            .expect("could not read file");

        let mut swords = numbers_string
            .lines()
            .map(construct_sword)
            .collect::<Vec<Sword>>();

        swords.sort();
        swords.reverse();
        println!("Swords: {:?}", swords);

        let sum = swords
            .into_iter()
            .enumerate()
            .map(|(idx, item)| {
                let position = idx as i64 + 1;
                let id = item.id;
                println!("Position: {}, Id: {}", position, id);
                id * position
            })
            .sum::<i64>();

        println!("Checksum: {:?}", sum);
    }
}
