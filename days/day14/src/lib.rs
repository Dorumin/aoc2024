const INPUT: &str = include_str!("../../../inputs/day14.txt");

type Offset = (usize, usize);
type Velocity = (isize, isize);

#[derive(Debug)]
struct Bathroom {
    width: usize,
    height: usize,
    robots: Vec<BunBot>,
}

#[derive(Debug)]
struct BunBot {
    position: Offset,
    velocity: Velocity,
}

// This should be in std already
fn strip(hays: &str, matcher: impl FnMut(char) -> bool) -> Option<(&str, &str)> {
    let rest = hays.trim_start_matches(matcher);
    let stripped = &hays[0..(hays.len() - rest.len())];

    if stripped.is_empty() {
        return None;
    }

    Some((stripped, rest))
}

fn int_matcher() -> impl FnMut(char) -> bool {
    let mut index = 0;

    move |c| {
        let matched = (index == 0 && c == '-') || c.is_ascii_digit();
        index += 1;

        matched
    }
}

impl BunBot {
    fn from_line(line: &str) -> Option<Self> {
        let line = line.strip_prefix("p=").unwrap();
        let (x, line) = strip(line, |c| c.is_ascii_digit()).unwrap();
        let line = line.strip_prefix(",").unwrap();
        let (y, line) = strip(line, |c| c.is_ascii_digit()).unwrap();
        let line = line.strip_prefix(" v=").unwrap();
        let (dx, line) = strip(line, int_matcher()).unwrap();
        let line = line.strip_prefix(",")?;
        let (dy, _) = strip(line, int_matcher()).unwrap();
        // /tease

        Some(BunBot {
            position: (x.parse().ok()?, y.parse().ok()?),
            velocity: (dx.parse().ok()?, dy.parse().ok()?),
        })
    }

    fn tick(&mut self, width: usize, height: usize) {
        self.position.0 = ((self.position.0 + width) as isize + self.velocity.0)
            .try_into()
            .unwrap();
        self.position.1 = ((self.position.1 + height) as isize + self.velocity.1)
            .try_into()
            .unwrap();
        self.position.0 %= width;
        self.position.1 %= height;
    }
}

impl Bathroom {
    fn from_str(input: &str, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            robots: input.lines().map(|line| BunBot::from_line(line).unwrap()).collect(),
        }
    }

    fn tick(&mut self) {
        self.robots
            .iter_mut()
            .for_each(|bot| bot.tick(self.width, self.height));
    }

    fn count_quads(&self) -> u64 {
        let mut cleft = 0;
        let mut cright = 0;
        let mut dreft = 0;
        let mut dright = 0;

        let mw = self.width / 2;
        let mh = self.height / 2;

        for bot in self.robots.iter() {
            if bot.position.0 < mw && bot.position.1 < mh {
                cleft += 1;
            } else if bot.position.0 > mw && bot.position.1 < mh {
                cright += 1;
            } else if bot.position.0 < mw && bot.position.1 > mh {
                dreft += 1;
            } else if bot.position.0 > mw && bot.position.1 > mh {
                dright += 1;
            }
        }

        cleft * cright * dreft * dright
    }
}

pub fn part1() {
    let mut bathroom = Bathroom::from_str(INPUT, 101, 103);

    for _ in 0..100 {
        bathroom.tick();
    }

    dbg!(bathroom.count_quads());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut bathroom = Bathroom::from_str(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            11,
            7,
        );

        for _ in 0..100 {
            bathroom.tick();
        }

        assert_eq!(bathroom.count_quads(), 12);
    }
}
