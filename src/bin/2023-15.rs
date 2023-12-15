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

fn main() {
    let input = include_str!("../input/2023-15.txt").lines().next().unwrap();

    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}
