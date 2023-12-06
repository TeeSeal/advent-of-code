use std::fs;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
struct ParseMapperError;
#[derive(Debug)]
struct ParseRangeError;

struct Mapper {
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl FromStr for Mapper {
    type Err = ParseMapperError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .skip(1)
            .map(parse_ranges)
            .collect::<Result<_, _>>()
            .map_err(|_| ParseMapperError)?;

        Ok(Mapper { ranges })
    }
}

fn parse_ranges(s: &str) -> Result<(Range<u64>, Range<u64>), ParseRangeError> {
    let [target_start, source_start, size] = s
        .split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return Err(ParseRangeError);
    };

    Ok((
        source_start..source_start + size,
        target_start..target_start + size,
    ))
}

impl Mapper {
    fn map(&self, input: &u64) -> u64 {
        for (source, target) in &self.ranges {
            if !source.contains(&input) {
                continue;
            }

            return target.start + (input - source.start);
        }

        input.to_owned()
    }

    fn map_range(&self, input: &Range<u64>) -> Vec<Range<u64>> {
        let mut result = Vec::new();

        for (source, target) in &self.ranges {
            if input.end <= source.start || input.start >= source.end {
                continue;
            }

            let start_offset = if source.contains(&input.start) {
                input.start - source.start
            } else {
                0
            };

            let end_offset = if source.contains(&input.end) {
                source.end - input.end
            } else {
                0
            };

            if input.start < source.start {
                result.append(&mut self.map_range(&(input.start..source.start)));
            }

            result.push(target.start + start_offset..target.end - end_offset);

            if input.end > source.end {
                result.append(&mut self.map_range(&(source.end..input.end)));
            }

            return result;
        }

        result.push(input.clone());
        result
    }

    fn map_ranges(&self, input_ranges: &Vec<Range<u64>>) -> Vec<Range<u64>> {
        input_ranges
            .iter()
            .flat_map(|range| self.map_range(range))
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let seeds: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let pipeline: Vec<Mapper> = input
        .split("\n\n")
        .skip(1)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let part_1 = seeds
        .iter()
        .map(|&seed| pipeline.iter().fold(seed, |res, mapper| mapper.map(&res)))
        .min()
        .unwrap();

    println!("Part 1: {}", part_1);

    let seed_ranges: Vec<Range<u64>> = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();

    let part_2 = pipeline
        .iter()
        .fold(seed_ranges, |res, mapper| mapper.map_ranges(&res))
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("Part 2: {}", part_2);
}
