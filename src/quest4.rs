pub mod part1 {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        /*
        How many full turns will the last gear make if the first one turns exactly 2025 times?
         */
        let mut file =
            File::open(Path::new("notes/quest4/part1")).expect("Input file does not exist");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read file");

        let gears = contents
            .split("\n")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let ratios = gears
            .windows(2)
            .map(|window| window[0] as f64 / window[1] as f64)
            .collect::<Vec<f64>>();

        let ratio = ratios
            .into_iter()
            .reduce(|ratio1, ratio2| ratio1 * ratio2)
            .unwrap();

        println!("The overall ratio is: {}", ratio);

        let first_gear = 2025f64;
        let last_gear = (first_gear * ratio) as i64;

        println!("The result is {} revolutions", last_gear)
        // 10074
    }
}

pub mod part2 {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        /*
        To calibrate the second mill, you need to provide the minimum number of full turns that the first gear must make, so that the last one turns at least 10000000000000 times.
        */
        let mut file =
            File::open(Path::new("notes/quest4/part2")).expect("Input file does not exist");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read file");

        let gears = contents
            .split("\n")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let ratios = gears
            .windows(2)
            .map(|window| window[0] as f64 / window[1] as f64)
            .collect::<Vec<f64>>();

        let ratio = ratios
            .into_iter()
            .reduce(|ratio1, ratio2| ratio1 * ratio2)
            .unwrap();

        println!("The overall ratio is: {}", ratio);

        let last_gear = 10000000000000f64;
        let first_gear = (last_gear / ratio).ceil();

        println!(
            "The first gear must spin {} times in order to make the last gear spin {} times.",
            first_gear, last_gear
        )
    }
}

pub mod part3 {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn part() {
        let mut file =
            File::open(Path::new("notes/quest4/part3")).expect("Input file does not exist");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read file");

        let gears = contents
            .split("\n")
            .map(|x| GearPair::from_str(x))
            .collect::<Vec<GearPair>>();

        let ratios = gears
            .windows(2)
            .map(|window| window[0].right as f64 / window[1].left as f64)
            .collect::<Vec<f64>>();

        let ratio = ratios
            .into_iter()
            .reduce(|ratio1, ratio2| ratio1 * ratio2)
            .unwrap();

        println!("The overall ratio is: {}", ratio);

        let first_gear = 100f64;
        let last_gear = (first_gear * ratio) as i64;

        println!("The result is {} revolutions", last_gear);
        // 207184865783
    }

    #[derive(Debug)]
    struct GearPair {
        left: i64,
        right: i64,
    }
    impl GearPair {
        pub fn from_str(pair: &str) -> Self {
            let mut parts = pair.split("|");
            let left = parts
                .next()
                .and_then(|part| part.parse::<i64>().ok())
                .unwrap();
            let right = parts
                .next()
                .and_then(|part| part.parse::<i64>().ok())
                .unwrap_or(left);
            Self {
                left: left,
                right: right,
            }
        }
    }
}
