use input::*;
use mr_kaffee_aoc::{Puzzle, Star};

/// the puzzle
pub fn puzzle() -> Puzzle<'static, PuzzleData, usize, usize, usize, usize> {
    Puzzle {
        year: 2022,
        day: 4,
        input: include_str!("../input.txt"),
        star1: Some(Star {
            name: "Star 1",
            f: &star_1,
            exp: Some(576),
        }),
        star2: Some(Star {
            name: "Star 2",
            f: &star_2,
            exp: Some(905),
        }),
    }
}

// tag::input[]
pub mod input {
    use mr_kaffee_aoc::err::PuzzleError;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub range_pairs: Vec<((usize, usize), (usize, usize))>,
    }

    fn parse_pair<'a, T: Iterator<Item = &'a str>>(
        iter: &mut T,
    ) -> Result<((usize, usize), (usize, usize)), PuzzleError> {
        parse_tuple(iter).and_then(|t1| parse_tuple(iter).map(|t2| (t1, t2)))
    }

    fn parse_tuple<'a, T: Iterator<Item = &'a str>>(
        iter: &mut T,
    ) -> Result<(usize, usize), PuzzleError> {
        iter.next()
            .ok_or_else(|| PuzzleError::from("Missing first item"))
            .and_then(|a| a.parse::<usize>().map_err(|e| e.into()))
            .and_then(|a| {
                iter.next()
                    .ok_or_else(|| PuzzleError::from("Missing second item"))
                    .and_then(|b| b.parse::<usize>().map(|b| (a, b)).map_err(|e| e.into()))
            })
    }

    impl TryFrom<&'static str> for PuzzleData {
        type Error = PuzzleError;

        /// parse the puzzle input
        fn try_from(s: &'static str) -> Result<Self, Self::Error> {
            s.lines()
                .map(|l| parse_pair(&mut l.split(|c: char| c == '-' || c == ',')))
                .collect::<Result<Vec<_>, _>>()
                .map(|range_pairs| Self { range_pairs })
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(data: &PuzzleData) -> usize {
    // count how often one range is contained in the other
    data.range_pairs
        .iter()
        .filter(|((start1, end1), (start2, end2))| {
            (start1 <= start2 && end2 <= end1) || (start2 <= start1 && end1 <= end2)
        })
        .count()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> usize {
    // count how often ranges overlap, i.e., start of one range is contained in the other range
    data.range_pairs
        .iter()
        .filter(|((start1, end1), (start2, end2))| {
            (start1 <= start2 && start2 <= end1) || (start2 <= start1 && start1 <= end2)
        })
        .count()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;
    use mr_kaffee_aoc::err::PuzzleError;

    const CONTENT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    pub fn test_star_1() -> Result<(), PuzzleError> {
        let data = PuzzleData::try_from(CONTENT)?;
        assert_eq!(2, star_1(&data));
        Ok(())
    }

    #[test]
    pub fn test_star_2() -> Result<(), PuzzleError> {
        let data = PuzzleData::try_from(CONTENT)?;
        assert_eq!(4, star_2(&data));
        Ok(())
    }
}
// end::tests[]
