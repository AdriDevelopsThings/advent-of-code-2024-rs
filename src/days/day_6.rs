use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Clone, Copy)]
enum MapElement {
    Block,
    Guard(Facing),
    Empty(bool), // Empty(false) if guard wasn't there yet, Empty(true) if guard was there
}

impl Facing {
    fn next_coord(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::Up => (x - 1, y),
            Self::Right => (x, y + 1),
            Self::Left => (x, y - 1),
            Self::Down => (x + 1, y),
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
        };
    }
}

impl MapElement {
    fn from_char(c: char) -> MapElement {
        match c {
            '#' => MapElement::Block,
            '.' => MapElement::Empty(false),
            '^' => MapElement::Guard(Facing::Up),
            '>' => MapElement::Guard(Facing::Right),
            '<' => MapElement::Guard(Facing::Left),
            'v' => MapElement::Guard(Facing::Down),
            _ => panic!("character '{c}' is an invalid map element"),
        }
    }
}

fn map_from_input(input: String) -> ((usize, usize), Vec<Vec<MapElement>>) {
    let mut guard_position: (usize, usize) = (0, 0);
    let map = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| {
                    let element = MapElement::from_char(c);
                    if matches!(element, MapElement::Guard(_)) {
                        guard_position = (x, y);
                    }
                    element
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (guard_position, map)
}

fn guard_run(guard_position: &mut (usize, usize), map: &mut [Vec<MapElement>]) -> bool {
    let rows = map.len();
    let cols = map[0].len();

    let mut coords: HashSet<(usize, usize, Facing)> = HashSet::new();

    loop {
        if let MapElement::Guard(facing) = map[guard_position.0][guard_position.1] {
            let (next_x, next_y) =
                facing.next_coord(guard_position.0 as i32, guard_position.1 as i32);
            if next_x < 0 || next_x == rows as i32 || next_y < 0 || next_y == cols as i32 {
                break;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;
            if matches!(map[next_x][next_y], MapElement::Block) {
                let mut facing = facing;
                facing.turn_right();
                map[guard_position.0][guard_position.1] = MapElement::Guard(facing);
            } else {
                if coords.contains(&(next_x, next_y, facing)) {
                    // endless loop detected
                    return false;
                }

                map[guard_position.0][guard_position.1] = MapElement::Empty(true);
                map[next_x][next_y] = MapElement::Guard(facing);
                *guard_position = (next_x, next_y);
                coords.insert((next_x, next_y, facing));
            }
        } else {
            panic!("invalid guard position");
        }
    }
    true
}

pub fn first(input: String) -> String {
    let (mut guard_position, mut map) = map_from_input(input);
    guard_run(&mut guard_position, &mut map);

    let sum = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|col| match col {
                    MapElement::Empty(b) => match b {
                        true => 1,
                        false => 0,
                    },
                    MapElement::Guard(_) => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    sum.to_string()
}

pub fn second(input: String) -> String {
    let (guard_position, map) = map_from_input(input);
    let rows = map.len();
    let cols = map[0].len();
    let mut sum: u32 = 0;
    for x in 0..rows {
        for y in 0..cols {
            if !matches!(map[x][y], MapElement::Empty(_)) {
                continue;
            }
            let mut cloned_map = map.clone();
            let mut cloned_guard_position = guard_position;
            cloned_map[x][y] = MapElement::Block;
            if !guard_run(&mut cloned_guard_position, &mut cloned_map) {
                // endless loop
                sum += 1;
            }
        }
    }

    sum.to_string()
}
