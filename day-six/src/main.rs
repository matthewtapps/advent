use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
fn main() {
    let mut map = parse_file("input.txt");
    let (current_coordinate, current_direction) = map.find_current_coordinate_and_direction();
    let mut position_data = PositionData {
        map,
        current_coordinate,
        current_direction,
        left_map: false,
    };

    while position_data.left_map == false {
        position_data.movement();
    }

    let result = position_data.map.count_visited();

    println!("{}", result)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PositionData {
    map: Map,
    current_coordinate: Coordinate,
    current_direction: Direction,
    left_map: bool,
}

impl PositionData {
    fn movement(&mut self) {
        let next_location_type = self
            .current_direction
            .to_location_type(&self.current_coordinate, &self.map);
        match next_location_type {
            LocationType::Obstacle => self.turn(),
            LocationType::OutOfBounds => self.finished(),
            LocationType::Clear => self.forwards(),
            LocationType::Explored => self.forwards(),
            LocationType::Me => self.forwards(),
        };
    }

    fn turn(&mut self) {
        let current_direction = self.current_direction;
        let new_direction = match current_direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        self.current_direction = new_direction;
    }

    fn forwards(&mut self) {
        let next_coordinate =
            Direction::to_coordinate(&self.current_direction, &self.current_coordinate);
        self.map.visit_coordinate(&next_coordinate);
        self.current_coordinate = next_coordinate;
    }

    fn finished(&mut self) {
        self.left_map = true
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Map(Vec<String>);

impl Map {
    fn location_type_from_coordinate(&self, coordinate: &Coordinate) -> LocationType {
        let x = coordinate.x;
        let y = coordinate.y;
        let (maximum_x, maximum_y) = self.get_maximum_bounds();
        let map = match self {
            Map(map) => map,
        };

        if x > -1 && y > -1 && x <= maximum_x && y <= maximum_y {
            let index_y = usize::try_from(y).unwrap();
            let index_x = usize::try_from(x).unwrap();
            return LocationType::from_character(&map[index_y][index_x..index_x + 1]);
        } else {
            return LocationType::OutOfBounds;
        }
    }

    fn get_maximum_bounds(&self) -> (i32, i32) {
        let map = match self {
            Map(map) => map,
        };
        let maximum_x = i32::try_from(map[0].len()).unwrap() - 1;
        let maximum_y = i32::try_from(map.len()).unwrap() - 1;
        return (maximum_x, maximum_y);
    }

    fn find_current_coordinate_and_direction(&mut self) -> (Coordinate, Direction) {
        let map = match &self {
            Map(map) => map,
        };

        for (y_index, line) in map.iter().enumerate() {
            for x_index in 0..line.len() {
                let current_coordinate = Coordinate {
                    x: i32::try_from(x_index).unwrap(),
                    y: i32::try_from(y_index).unwrap(),
                };
                let location_type = self.location_type_from_coordinate(&current_coordinate);

                if location_type == LocationType::Me {
                    self.visit_coordinate(&current_coordinate);
                    return (current_coordinate, Direction::North);
                }
            }
        }

        panic!()
    }

    fn count_visited(&self) -> i32 {
        let map = match self {
            Map(map) => map,
        };

        let mut total = 0;

        for (y_index, line) in map.iter().enumerate() {
            for x_index in 0..line.len() {
                let current_coordinate = Coordinate {
                    x: i32::try_from(x_index).unwrap(),
                    y: i32::try_from(y_index).unwrap(),
                };
                let location_type = self.location_type_from_coordinate(&current_coordinate);

                if location_type == LocationType::Explored || location_type == LocationType::Me {
                    total += 1;
                }
            }
        }

        return total;
    }

    fn visit_coordinate(&mut self, coordinate: &Coordinate) -> Self {
        let x = usize::try_from(coordinate.x).unwrap();
        let y = usize::try_from(coordinate.y).unwrap();
        let map = match self {
            Map(map) => map,
        };

        let mut map_line = map[y].clone();

        map_line.replace_range(
            x..x + 1,
            LocationType::to_character(&LocationType::Explored),
        );

        map[y] = map_line;

        Self(map.clone())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq)]
enum LocationType {
    Obstacle,
    Clear,
    OutOfBounds,
    Explored,
    Me,
}

impl LocationType {
    fn from_character(character: &str) -> LocationType {
        match character {
            "#" => LocationType::Obstacle,
            "." => LocationType::Clear,
            "^" => LocationType::Me,
            "X" => LocationType::Explored,
            _ => LocationType::OutOfBounds,
        }
    }

    fn to_character(location_type: &LocationType) -> &str {
        match location_type {
            LocationType::Obstacle => "#",
            LocationType::Clear => ".",
            LocationType::Explored => "X",
            LocationType::OutOfBounds => "L",
            LocationType::Me => "^",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_coordinate(&self, current_coordinate: &Coordinate) -> Coordinate {
        let x = current_coordinate.x;
        let y = current_coordinate.y;

        match self {
            Direction::North => Coordinate { x, y: y - 1 },
            Direction::South => Coordinate { x, y: y + 1 },
            Direction::East => Coordinate { x: x + 1, y },
            Direction::West => Coordinate { x: x - 1, y },
        }
    }

    fn to_location_type(&self, coordinate: &Coordinate, map: &Map) -> LocationType {
        let next_coordinate = self.to_coordinate(&coordinate);

        return map.location_type_from_coordinate(&next_coordinate);
    }
}

// thanks stackoverflow
fn parse_file(file_path: impl AsRef<Path>) -> Map {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    let lines: Vec<String> = buf
        .lines()
        .map(|line| line.expect("could not parse line"))
        .collect();
    return Map(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test_one() {
        let result = parse_file("./mocks/mock1.txt");
        let want = Map(vec!["....#.....".to_string()]);
        assert_eq!(want, result);
    }

    #[test]
    fn movement_test() {
        let mut position_data = PositionData {
            map: Map(vec![
                "X...#.....".to_string(),
                ".........#".to_string(),
                "..........".to_string(),
                "..#.......".to_string(),
                ".......#..".to_string(),
                "..........".to_string(),
                ".#........".to_string(),
                "........#.".to_string(),
                "#.........".to_string(),
                "......#...".to_string(),
            ]),
            current_coordinate: Coordinate { x: 0, y: 0 },
            current_direction: Direction::South,
            left_map: false,
        };

        position_data.movement();
        position_data.movement();

        let want = PositionData {
            map: Map(vec![
                "X...#.....".to_string(),
                "X........#".to_string(),
                "X.........".to_string(),
                "..#.......".to_string(),
                ".......#..".to_string(),
                "..........".to_string(),
                ".#........".to_string(),
                "........#.".to_string(),
                "#.........".to_string(),
                "......#...".to_string(),
            ]),
            current_coordinate: Coordinate { x: 0, y: 2 },
            current_direction: Direction::South,
            left_map: false,
        };

        assert_eq!(want, position_data)
    }

    #[test]
    fn visit_coordinate_test() {
        let mut map = Map(vec![
            "X...#.....".to_string(),
            "X........#".to_string(),
            "X.........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#........".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ]);

        map.visit_coordinate(&Coordinate { x: 4, y: 2 });

        let want = Map(vec![
            "X...#.....".to_string(),
            "X........#".to_string(),
            "X...X.....".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#........".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ]);

        assert_eq!(want, map)
    }

    #[test]
    fn parse_position_test() {
        let mut map = parse_file("./mocks/example.txt");
        let (result, _current_direction) = map.find_current_coordinate_and_direction();
        let want = Coordinate { x: 4, y: 6 };

        assert_eq!(want, result)
    }

    #[test]
    fn parse_position_and_move_test() {
        let mut map = parse_file("./mocks/example.txt");
        let (current_coordinate, current_direction) = map.find_current_coordinate_and_direction();
        let mut position_data = PositionData {
            map,
            current_coordinate,
            current_direction,
            left_map: false,
        };

        position_data.movement();

        let result = position_data.current_coordinate;

        let want = Coordinate { x: 4, y: 5 };

        assert_eq!(want, result)
    }

    #[test]
    fn part_one_acceptance_test() {
        let mut map = parse_file("./mocks/example.txt");
        let (current_coordinate, current_direction) = map.find_current_coordinate_and_direction();
        let mut position_data = PositionData {
            map,
            current_coordinate,
            current_direction,
            left_map: false,
        };

        while position_data.left_map == false {
            position_data.movement();
        }

        let result = position_data.map.count_visited();

        println!("{:?}", position_data);
        let want = 41;

        assert_eq!(want, result)
    }
}
