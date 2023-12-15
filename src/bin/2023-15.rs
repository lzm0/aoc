use std::collections::VecDeque;

fn hash(input: &str) -> u8 {
    let mut current_value = 0;
    for c in input.bytes() {
        let mut temp = current_value as u32 + c as u32;
        temp *= 17;
        current_value = (temp % 256) as u8;
    }
    current_value
}

fn part_one(input: &str) -> u32 {
    input.split(',').map(|step| hash(step) as u32).sum()
}

fn part_two(input: &str) -> u32 {
    let mut boxes: [VecDeque<(&str, u8)>; 256] = std::array::from_fn(|_| VecDeque::new());
    for step in input.split(',') {
        if step.ends_with('-') {
            let label = &step[..step.len() - 1];
            let relevant_box = &mut boxes[hash(label) as usize];
            if let Some(index) = relevant_box.iter().position(|&(l, _)| l == label) {
                relevant_box.remove(index);
            }
        } else {
            let (label, focal_length) = step.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            let relevant_box = &mut boxes[hash(label) as usize];
            if let Some(index) = relevant_box.iter().position(|&(l, _)| l == label) {
                relevant_box[index].1 = focal_length;
            } else {
                relevant_box.push_back((label, focal_length));
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, &(_, f))| (i as u32 + 1) * (j as u32 + 1) * f as u32)
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input/2023-15.txt").lines().next().unwrap();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE), 1320);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE), 145);
    }
}
