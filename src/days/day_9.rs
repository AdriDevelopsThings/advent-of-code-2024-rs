#[derive(Clone, PartialEq)]
enum Block {
    Filesystem(usize),
    Free,
}

pub fn first(input: String) -> String {
    let mut blocks = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                vec![Block::Filesystem(i / 2); c.to_string().parse::<usize>().unwrap()]
            } else {
                vec![Block::Free; c.to_string().parse::<usize>().unwrap()]
            }
        })
        .collect::<Vec<Block>>();

    loop {
        let (last_block_pos, last_block) = blocks
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| matches!(b, Block::Filesystem(_)))
            .unwrap();

        let free_pos = blocks
            .iter()
            .enumerate()
            .find(|(_, a)| matches!(a, Block::Free))
            .unwrap()
            .0;

        if last_block_pos < free_pos {
            break;
        }

        blocks[free_pos] = last_block.clone();
        blocks[last_block_pos] = Block::Free;
    }

    let sum = blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if let Block::Filesystem(id) = b {
                return Some(i * id);
            }
            None
        })
        .sum::<usize>();

    sum.to_string()
}

pub fn second(_input: String) -> String {
    String::new()
}
