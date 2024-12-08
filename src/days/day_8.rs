use std::collections::HashSet;

fn get_map_antennas_by_input(input: String) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut antennas: Vec<(usize, usize)> = Vec::new();
    let map = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c != '.' {
                        antennas.push((x, y));
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (map, antennas)
}

pub fn first(input: String) -> String {
    let (map, antennas) = get_map_antennas_by_input(input);

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let mut cords: HashSet<(i32, i32)> = HashSet::new();

    for antenna_1 in &antennas {
        for antenna_2 in &antennas {
            if (antenna_1.0 == antenna_2.0 && antenna_1.1 == antenna_2.1)
                || map[antenna_1.0][antenna_1.1] != map[antenna_2.0][antenna_2.1]
            {
                continue;
            }

            let x_distance = (antenna_1.0 as i32) - (antenna_2.0 as i32);
            let y_distance = (antenna_1.1 as i32) - (antenna_2.1 as i32);

            let a_1_x = (antenna_1.0 as i32) + x_distance;
            let a_1_y = (antenna_1.1 as i32) + y_distance;

            let a_2_x = (antenna_2.0 as i32) - x_distance;
            let a_2_y = (antenna_2.1 as i32) - y_distance;

            if a_1_x >= 0 && a_1_x < rows && a_1_y >= 0 && a_1_y < cols {
                cords.insert((a_1_x, a_1_y));
            }

            if a_2_x >= 0 && a_2_x < rows && a_2_y >= 0 && a_2_y < cols {
                cords.insert((a_2_x, a_2_y));
            }
        }
    }
    cords.len().to_string()
}

pub fn second(input: String) -> String {
    let (map, antennas) = get_map_antennas_by_input(input);

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let mut cords: HashSet<(i32, i32)> = HashSet::new();

    for antenna_1 in &antennas {
        for antenna_2 in &antennas {
            if (antenna_1.0 == antenna_2.0 && antenna_1.1 == antenna_2.1)
                || map[antenna_1.0][antenna_1.1] != map[antenna_2.0][antenna_2.1]
            {
                continue;
            }
            cords.insert((antenna_1.0 as i32, antenna_1.1 as i32));
            cords.insert((antenna_2.0 as i32, antenna_2.1 as i32));

            let x_distance = (antenna_1.0 as i32) - (antenna_2.0 as i32);
            let y_distance = (antenna_1.1 as i32) - (antenna_2.1 as i32);

            let mut a_1_x = (antenna_1.0 as i32) + x_distance;
            let mut a_1_y = (antenna_1.1 as i32) + y_distance;

            while a_1_x >= 0 && a_1_x < rows && a_1_y >= 0 && a_1_y < cols {
                cords.insert((a_1_x, a_1_y));
                a_1_x += x_distance;
                a_1_y += y_distance;
            }

            let mut a_2_x = (antenna_2.0 as i32) - x_distance;
            let mut a_2_y = (antenna_2.1 as i32) - y_distance;

            while a_2_x >= 0 && a_2_x < rows && a_2_y >= 0 && a_2_y < cols {
                cords.insert((a_2_x, a_2_y));
                a_2_x -= x_distance;
                a_2_y -= y_distance;
            }
        }
    }
    cords.len().to_string()
}
