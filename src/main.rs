fn dispatch(p1: fn(), p2: fn(), part: &str) {
    match part {
        "1" => p1(),
        "2" => p2(),
        _ => panic!("There's only two parts"),
    }
}

fn main() {
    let mut vargs = std::env::args().skip(1);
    let its_a_date = vargs.next().expect("Pass an argument man");
    let parte = vargs.next().expect("Pass an argument man");

    match its_a_date.as_ref() {
        "day1" => dispatch(day1::part1, day1::part2, &parte),
        "day2" => dispatch(day2::part1, day2::part2, &parte),
        "day3" => dispatch(day3::part1, day3::part2, &parte),
        "day4" => dispatch(day4::part1, day4::part2, &parte),
        "day5" => dispatch(day5::part1, day5::part2, &parte),
        "day6" => dispatch(day6::part1, day6::part2, &parte),
        "fetch" => {
            let session =
                std::fs::read_to_string("session.txt").expect("session.txt with session key");

            let input = ureq::get(&format!("https://adventofcode.com/2024/day/{parte}/input"))
                .set("Cookie", &format!("session={key}", key = session.trim()))
                .call()
                .unwrap()
                .into_string()
                .unwrap();

            std::fs::write(format!("inputs/day{parte}.txt"), input).expect("write to file");
        }
        _ => panic!("No day that matches"),
    }
}
