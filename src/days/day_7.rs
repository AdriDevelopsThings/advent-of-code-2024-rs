fn get_possible_operators(n: usize, variants: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<Vec<usize>> = Vec::new();
    let c = (variants as u32).pow(n as u32);

    while v.len() < c as usize {
        let last = v.last();
        if let Some(last) = last {
            let mut cloned = last.clone();
            {
                let m = cloned.last_mut().unwrap();
                *m += 1;
            }
            let mut carry = 0;
            if cloned[cloned.len() - 1] == variants {
                let m = cloned.last_mut().unwrap();
                *m = 0;
                carry = 1;
            }
            for i in (0..cloned.len() - 1).rev() {
                cloned[i] += carry;
                if cloned[i] == variants {
                    cloned[i] = 0;
                } else {
                    carry = 0;
                }
            }
            v.push(cloned);
        } else {
            v.push(vec![0; n]);
        }
    }

    v
}

pub fn first(input: String) -> String {
    let sum = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|equation| {
            let splitted = equation.split(": ").collect::<Vec<_>>();
            let test_value = splitted[0].parse::<u64>().unwrap();
            let equation_values = splitted[1]
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let possible_operators = get_possible_operators(equation_values.len() - 1, 2);
            for operators in possible_operators {
                let mut a = equation_values[0];
                for (i, v) in equation_values.iter().skip(1).enumerate() {
                    let operator = operators[i];
                    if operator == 0 {
                        a += v;
                    } else {
                        // operator == 1
                        a *= v;
                    }
                }
                if a == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum::<u64>();
    sum.to_string()
}

pub fn second(input: String) -> String {
    let sum = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|equation| {
            let splitted = equation.split(": ").collect::<Vec<_>>();
            let test_value = splitted[0].parse::<u64>().unwrap();
            let equation_values = splitted[1]
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let possible_operators = get_possible_operators(equation_values.len() - 1, 3);
            for operators in possible_operators {
                let mut a = equation_values[0];
                for (i, v) in equation_values.iter().skip(1).enumerate() {
                    let operator = operators[i];
                    if operator == 0 {
                        a += v;
                    } else if operator == 1 {
                        a *= v;
                    } else {
                        // operator == 2, concat
                        a = format!("{a}{v}").parse().unwrap();
                    }
                }
                if a == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum::<u64>();
    sum.to_string()
}
