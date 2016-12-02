fn parse_coords(input: &str) {
    //   N
    // W   E
    //   S
    enum Direction {
        North,
        East,
        South,
        West
    };

    #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
    struct Coord2D {
        x: i32,
        y: i32
    };
    impl Coord2D {
        fn new(x: i32, y: i32) -> Coord2D {
            Coord2D {
                x: x,
                y: y
            }
        }

        fn dist(&self) -> i32 {
            self.x.abs() + self.y.abs()
        }
    };

    let mut direction = Direction::North;
    let mut coordinates = vec![Coord2D::new(0, 0)];
    let mut part2 = false;

    let coords: Vec<&str> = input.split(", ").collect();
    for coord in coords {
        let dir = &coord[0..1];
        match dir {
            // Rotate 90 degrees to left
            "L" => {
                match direction {
                    Direction::North => direction = Direction::West,
                    Direction::West => direction = Direction::South,
                    Direction::South => direction = Direction::East,
                    Direction::East => direction = Direction::North,
                }
            },
            // Rotate 90 degrees to right
            "R" => {
                match direction {
                    Direction::North => direction = Direction::East,
                    Direction::East => direction = Direction::South,
                    Direction::South => direction = Direction::West,
                    Direction::West => direction = Direction::North
                }
            },
            _ => println!("unknown coordinate"),
        };

        let amt: i32 = String::from(&coord[1..]).parse().unwrap();
        let mut cur_coord: Coord2D = coordinates.last().unwrap().clone();
        match direction {
            Direction::North => cur_coord.y += amt,
            Direction::East => cur_coord.x += amt,
            Direction::South => cur_coord.y -= amt,
            Direction::West => cur_coord.x -= amt
        };

        if coordinates.contains(&cur_coord) && !part2 {
            part2 = true;
            println!("Part 2: {}", &cur_coord.dist());
        }
        coordinates.push(cur_coord.clone());
    }

    println!("Part 1: {}", coordinates.last().unwrap().dist());
}

fn main() {
    parse_coords("L1, R3, R1, L5, L2, L5, R4, L2, R2, R2, L2, R1, L5, R3, L4, L1, L2, R3, R5, L2, R5, L1, R2, L5, R4, R2, R2, L1, L1, R1, L3, L1, R1, L3, R5, R3, R3, L4, R4, L2, L4, R1, R1, L193, R2, L1, R54, R1, L1, R71, L4, R3, R191, R3, R2, L4, R3, R2, L2, L4, L5, R4, R1, L2, L2, L3, L2, L1, R4, R1, R5, R3, L5, R3, R4, L2, R3, L1, L3, L3, L5, L1, L3, L3, L1, R3, L3, L2, R1, L3, L1, R5, R4, R3, R2, R3, L1, L2, R4, L3, R1, L1, L1, R5, R2, R4, R5, L1, L1, R1, L2, L4, R3, L1, L3, R5, R4, R3, R3, L2, R2, L1, R4, R2, L3, L4, L2, R2, R2, L4, R3, R5, L2, R2, R4, R5, L2, L3, L2, R5, L4, L2, R3, L5, R2, L1, R1, R3, R3, L5, L2, L2, R5");
}