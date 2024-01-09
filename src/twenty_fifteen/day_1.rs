pub mod day_1 {
    use std::{fs::File, io::{BufReader, BufRead}};

    pub fn part_one() {
        let file = File::open("src/twenty_fifteen/input/day_1.txt").expect("Couldn't find file");

        let buffer = BufReader::new(&file);

        let mut count = 0;

        buffer
            .lines()
            .for_each(|line| {
            line
                .unwrap()
                .as_bytes()
                .into_iter()
                .for_each(|c| {
                    match c {
                        b'(' => count += 1,
                        b')' => count -= 1,
                        _ => (),
                    }
            })
        });

        println!("Floor = {}", count);
    }

    pub fn part_two() {
        let file = File::open("src/twenty_fifteen/input/day_1.txt").expect("Couldn't find file");

        let buffer = BufReader::new(&file);

        let mut count = 0;

        buffer
            .lines()
            .for_each(|line| {
            let mut basement = line
                .unwrap()
                .as_bytes()
                .into_iter()
                .enumerate()
                .filter_map(|(index, c)| {
                    match c {
                        b'(' => count += 1,
                        b')' => count -= 1,
                        _ => (),
                    }

                    match count {
                        -1 => {
                            Some(index)
                        },
                        _ => None
                    }
            }).next().unwrap();

            println!("basement = {}", basement + 1);
        });
    }
}