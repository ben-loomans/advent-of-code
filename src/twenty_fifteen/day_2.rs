pub mod day_2 {
    use std::{fs::File, io::{BufReader, BufRead}};

    struct Net {
        h: u32,
        w: u32,
        l: u32,
    }

    impl Net {
        fn area(&self) -> u32 {
            2*self.l*self.w + 2*self.w*self.h + 2*self.h*self.l + self.smallest_side()
        }

        fn volume(&self) -> u32 {
            self.l * self.w * self.h
        }

        fn ribbon(&self) -> u32 {
            let mut lens = Vec::new();

            lens.push(self.h);
            lens.push(self.w);
            lens.push(self.l);

            lens.sort();

            2*lens.get(0).unwrap() + 2*lens.get(1).unwrap() + self.volume()
        }

        fn smallest_side(&self) -> u32 {
            let mut lens = Vec::new();

            lens.push(self.h);
            lens.push(self.w);
            lens.push(self.l);

            lens.sort();

            lens.get(0).unwrap() * lens.get(1).unwrap()
        }
    }

    impl From<&str> for Net {

        fn from(value: &str) -> Self {
            let dims: Vec<&str> = value.split('x').collect();

            Net {
                h: dims.get(0).unwrap().parse::<u32>().unwrap(),
                w: dims.get(1).unwrap().parse::<u32>().unwrap(),
                l: dims.get(2).unwrap().parse::<u32>().unwrap(),
            }
        }
    }

    pub fn part_one() {
        let file = File::open("src/twenty_fifteen/input/day_2.txt").expect("Couldn't find file");
        let buf = BufReader::new(&file);

        let mut sum = 0;

        buf.lines().into_iter().for_each(|line| {
            let net = Net::from(line.unwrap().as_str());
            sum += net.area();
        });

        println!("Sum of area = {}", sum);
    }

    pub fn part_two() {
        let file = File::open("src/twenty_fifteen/input/day_2.txt").expect("Couldn't find file");
        let buf = BufReader::new(&file);

        let mut sum = 0;

        buf.lines().into_iter().for_each(|line| {
            let net = Net::from(line.unwrap().as_str());
            sum += net.ribbon();
        });

        println!("Sum of ribbon = {}", sum);
    }
}