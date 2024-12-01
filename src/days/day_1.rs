fn input_to_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split("   ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut first_list: Vec<i32> = Vec::with_capacity(lines.len());
    let mut second_list: Vec<i32> = Vec::with_capacity(lines.len());
    for line in lines {
        first_list.push(line[0].parse().unwrap());
        second_list.push(line[1].parse().unwrap());
    }
    (first_list, second_list)
}

pub fn first(input: String) -> String {
    let (first_list, second_list) = input_to_lists(input);

    let sum = first_list
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v - second_list[i]).abs())
        .sum::<i32>();

    sum.to_string()
}

pub fn second(input: String) -> String {
    let (first_list, second_list) = input_to_lists(input);
    let sum = first_list
        .into_iter()
        .map(|value| {
            let count = second_list
                .iter()
                .filter(|v| **v == value)
                .collect::<Vec<_>>()
                .len();
            value * (count as i32)
        })
        .sum::<i32>();
    sum.to_string()
}
