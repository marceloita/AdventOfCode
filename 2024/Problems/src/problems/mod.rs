pub mod problem01;
pub mod problem02;
pub mod problem03;
pub mod problem04;
pub mod problem05;

pub trait AdventProblem {
    fn part1(&self, input: &str) -> i32;
    fn part2(&self, input: &str) -> i32;
}

pub enum Problem {
    P01(problem01::Problem01),
    P02(problem02::Problem02),
    P03(problem03::Problem03),
    P04(problem04::Problem04),
    P05(problem05::Problem05),
}

impl AdventProblem for Problem {
    fn part1(&self, input: &str) -> i32 {
        match self {
            Self::P01(p) => p.part1(input),
            Self::P02(p) => p.part1(input),
            Self::P03(p) => p.part1(input),
            Self::P04(p) => p.part1(input),
            Self::P05(p) => p.part1(input),
        }
    }
    fn part2(&self, input: &str) -> i32 {
        match self {
            Self::P01(p) => p.part2(input),
            Self::P02(p) => p.part2(input),
            Self::P03(p) => p.part2(input),
            Self::P04(p) => p.part2(input),
            Self::P05(p) => p.part2(input),
        }
    }
}

pub fn get_problem(problem_num: u32) -> Option<Problem> {
    match problem_num {
        1 => Some(Problem::P01(problem01::Problem01)),
        2 => Some(Problem::P02(problem02::Problem02)),
        3 => Some(Problem::P03(problem03::Problem03)),
        4 => Some(Problem::P04(problem04::Problem04)),
        5 => Some(Problem::P05(problem05::Problem05)),
        _ => None,
    }
}