use std::{fs::read_to_string, iter::zip};

const FILE_PATH: &str = "data/day1.txt";

fn main() {
    a();
    b();
}

fn a() {
    let (mut left_vec, mut right_vec) = parse_input();
    left_vec.sort();
    right_vec.sort();
    let res: i32 = zip(left_vec, right_vec)
        .map(|(left, right)| (left - right).abs())
        .sum();
    println!("a: {res}");
}

fn b() {
    let (left_vec, right_vec) = parse_input();
    let res: i32 = left_vec
        .iter()
        .map(|number| number * get_times_appeared(number, &right_vec))
        .sum();
    println!("b: {res}");
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut left_vec = vec![];
    let mut right_vec = vec![];
    read_to_string(FILE_PATH)
        .expect(format!("Couldn't read file at {FILE_PATH}").as_str())
        .lines()
        .map(|l| l.split("   "))
        .for_each(|mut split| {
            let left = split
                .next()
                .expect("Couldn't get left number")
                .parse::<i32>()
                .expect("Couldn't pass string to number");
            let right = split
                .next()
                .expect("Couldn't get right number")
                .parse::<i32>()
                .expect("Couldn't pass string to number");
            left_vec.push(left);
            right_vec.push(right);
        });

    (left_vec, right_vec)
}

fn get_times_appeared(number: &i32, list: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for other in list.iter() {
        if other.eq(&number) {
            counter += 1;
        }
    }
    counter
}
