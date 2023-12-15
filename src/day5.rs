use std::ops::Range;

#[derive(Debug)]
struct Map {
    src_dest_map: Vec<(Range<usize>, Range<usize>)>,
}

impl Map {
    fn from_categories(mut src_dest_map: Vec<(Range<usize>, Range<usize>)>) -> Self {
        // since none of the categories are overlapping, they will NOT have the same start,
        // so sorting by start is fine.

        src_dest_map.sort_by_key(|(src, _)| src.start);

        Self { src_dest_map }
    }

    fn get(&self, key: usize) -> usize {
        // binary search the key's range first.
        let ind = self
            .src_dest_map
            .partition_point(|(src, _)| src.start <= key)
            .wrapping_sub(1);

        self.src_dest_map
            .get(ind)
            .and_then(|(src, dest)| (key <= src.end).then_some(dest.start + (key - src.start)))
            .unwrap_or(key)
    }
}

pub fn level1(s: &str) {
    let (seeds, rest) = s.split_once("\n\n").unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap());

    let maps: Vec<Map> = rest
        .split("\n\n")
        .map(|map_str| {
            Map::from_categories(
                map_str
                    .lines()
                    .skip(1)
                    .map(|src_dest_mapping| {
                        let src_dest_mapping = src_dest_mapping.split_whitespace();
                        let src_dest_n = src_dest_mapping.take(3);

                        let src_dest_n = src_dest_n.map(|x| x.parse::<usize>().unwrap());

                        let [dest, src, n]: [usize; 3] =
                            src_dest_n.collect::<Vec<usize>>().try_into().unwrap();

                        (src..src + n, dest..dest + n)
                    })
                    .collect(),
            )
        })
        .collect();

    let answer = seeds
        .inspect(|seed| println!("Working for {seed}..."))
        .map(|seed| maps.iter().fold(seed, |curr_seed, map| map.get(curr_seed)))
        .inspect(|location| println!("We got the location {location}.\n"))
        .min()
        .unwrap();

    println!("{answer}");
}
