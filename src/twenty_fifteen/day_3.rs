pub mod day_3 {
    use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

    use advent_of_code::problem::Problem;

    pub struct Day3 {
        path: String
    }

    impl Problem for Day3 {
        fn new(input: &str) -> Self {
            Self {
                path: input.to_string()
            }
        }

        fn part_one(&self) {
            let file = File::open(&self.path).expect("Couldn't find file");
            let buffer = BufReader::new(&file);

            let mut set = HashSet::new();

            let (mut x, mut y) = (0, 0);
            set.insert((x, y));

            buffer.lines()
                .into_iter()
                .for_each(|line| {
                    line.unwrap()
                        .as_bytes()
                        .into_iter()
                        .for_each(|byte| {
                            match byte {
                                b'^' => y += 1,
                                b'v' => y -= 1,
                                b'>' => x += 1,
                                b'<' => x -= 1,
                                _ => ()
                            }

                            set.insert((x, y));
                        })
                });
            
            println!("unique houses = {}", set.len());
        }

        fn part_two(&self) {
            let file = File::open(&self.path).expect("Couldn't find file");
            let buffer = BufReader::new(&file);

            let mut set = HashSet::new();

            let mut santas = [[0, 0], [0, 0]];

            set.insert(santas[0]);

            buffer.lines()
                .into_iter()
                .for_each(|line| {
                    line.unwrap()
                        .as_bytes()
                        .into_iter()
                        .enumerate()
                        .for_each(|(index, byte)| {
                            let parity = index % 2;
                            match byte {
                                b'^' => santas[parity][1] += 1,
                                b'v' => santas[parity][1] -= 1,
                                b'>' => santas[parity][0] += 1,
                                b'<' => santas[parity][0] -= 1,
                                _ => ()
                            }

                            set.insert(santas[parity]);
                        })
                });
            
            println!("unique houses = {}", set.len());
        }
    }
}