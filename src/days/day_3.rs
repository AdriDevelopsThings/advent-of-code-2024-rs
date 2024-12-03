use regex::Regex;

pub fn first(input: String) -> String {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum = regex
        .captures_iter(&input)
        .map(|c| {
            let first = c[1].parse::<u32>().unwrap();
            let second = c[2].parse::<u32>().unwrap();
            first * second
        })
        .sum::<u32>();
    sum.to_string()
}

pub fn second(input: String) -> String {
    let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let sum = regex
        .captures_iter(&input)
        .map(|c| {
            if c.get(1).is_some() {
                // mul instruction
                let first = c[2].parse::<u32>().unwrap();
                let second = c[3].parse::<u32>().unwrap();
                if enabled {
                    return first * second;
                }
            } else if c.get(4).is_some() {
                // do instruction
                enabled = true;
            } else if c.get(5).is_some() {
                // don't instruction
                enabled = false
            }
            0
        })
        .sum::<u32>();
    sum.to_string()
}
