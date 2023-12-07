struct Map {
    sections: Vec<MapSection>,
}
struct MapSection {
    source: MapRange,
    dest: MapRange,
}

impl TryFrom<String> for MapSection {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let sections = value.split_whitespace().collect::<Vec<&str>>();
        let source = sections[1].parse::<u64>().map_err(|_| ())?;
        let dest = sections[0].parse::<u64>().map_err(|_| ())?;
        let range = sections[2].parse::<u64>().map_err(|_| ())?;
        Ok(Self {
            source: MapRange {
                start: source,
                end: source + range,
            },
            dest: MapRange {
                start: dest,
                end: dest + range,
            },
        })
    }
}

impl Map {
    fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    fn map(&self, input: u64) -> u64 {
        for section in &self.sections {
            if section.contains(input) {
                return section.translate(input);
            }
        }
        input
    }
}

struct MapRange {
    start: u64,
    end: u64,
}

impl MapSection {
    fn contains(&self, input: u64) -> bool {
        input >= self.source.start && input <= self.source.end
    }

    fn translate(&self, input: u64) -> u64 {
        input + self.dest.start - self.source.start
    }
}

struct MapSet {
    seed2soil: Map,
    soil2fertilizer: Map,
    fertilizer2water: Map,
    water2light: Map,
    light2temperature: Map,
    temperature2humidity: Map,
    humidity2location: Map,
}

fn load<I: Iterator<Item = String>>(source: I) -> (Vec<u64>, MapSet) {
    let mut seeds = Vec::new();
    let mut seed2soil = Map::new();
    let mut soil2fertilizer = Map::new();
    let mut fertilizer2water = Map::new();
    let mut water2light = Map::new();
    let mut light2temperature = Map::new();
    let mut temperature2humidity = Map::new();
    let mut humidity2location = Map::new();
    let mut mode = 0;
    for s in source {
        if s.is_empty() {
            mode += 1;
            continue;
        }
        match mode {
            0 => {
                seeds.extend(
                    s.split_whitespace()
                        .map(|s| s.parse::<u64>())
                        .filter_map(Result::ok),
                );
            }
            1 => {
                seed2soil.sections.extend(MapSection::try_from(s).ok());
            }
            2 => {
                soil2fertilizer
                    .sections
                    .extend(MapSection::try_from(s).ok());
            }
            3 => {
                fertilizer2water
                    .sections
                    .extend(MapSection::try_from(s).ok());
            }
            4 => {
                water2light.sections.extend(MapSection::try_from(s).ok());
            }
            5 => {
                light2temperature
                    .sections
                    .extend(MapSection::try_from(s).ok());
            }
            6 => {
                temperature2humidity
                    .sections
                    .extend(MapSection::try_from(s).ok());
            }
            7 => {
                humidity2location
                    .sections
                    .extend(MapSection::try_from(s).ok());
            }

            _ => panic!("Hit undefined mode {mode}"),
        }
    }
    (
        seeds,
        MapSet {
            seed2soil,
            soil2fertilizer,
            fertilizer2water,
            water2light,
            light2temperature,
            temperature2humidity,
            humidity2location,
        },
    )
}

impl MapSet {
    fn seed2loc(&self, seed: u64) -> u64 {
        dbg!(seed);
        let soil = self.seed2soil.map(seed);
        dbg!(soil);
        let fertilizer = self.soil2fertilizer.map(soil);
        dbg!(fertilizer);
        let water = self.fertilizer2water.map(fertilizer);
        dbg!(water);
        let light = self.water2light.map(water);
        dbg!(light);
        let temp = self.light2temperature.map(light);
        dbg!(temp);
        let hum = self.temperature2humidity.map(temp);
        dbg!(hum);
        let loc = self.humidity2location.map(hum);
        dbg!(loc);
        loc
    }
}

fn main() {
    let (seeds, map_set) = load(std::io::stdin().lines().filter_map(Result::ok));
    let mut min_loc = u64::MAX;
    for seed in seeds {
        let loc = map_set.seed2loc(seed);
        min_loc = min_loc.min(loc);
    }
    println!("min_loc: {min_loc}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mapping() {
        let data = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        let (seeds, map_set) = load(data.into_iter().map(|s| s.to_owned()));
        let locations = seeds
            .into_iter()
            .map(|s| map_set.seed2loc(s))
            .collect::<Vec<u64>>();
        assert_eq!(locations, vec![82, 43, 86, 35]);
    }
}
