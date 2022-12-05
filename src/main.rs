use std::collections::HashSet;

fn main() {
    let mut score = 0;
    let lines: Vec<String> = std::io::stdin().lines().map(|r| r.unwrap()).collect();
    for i in (0..lines.len()).step_by(3) {
        let a = parse_rucksack(&lines[i]);
        let b = parse_rucksack(&lines[i + 1]);
        let c = parse_rucksack(&lines[i + 2]);
        score += item_priority(find_badge(&a, &b, &c).unwrap());
    }
    println!("Ans: {}", score);
}

fn parse_rucksack(ln: &str) -> HashSet<u8> {
    ln.as_bytes().iter().copied().collect()
}

fn item_priority(item: u8) -> i32 {
    if b'A' <= item && item <= b'Z' {
        return (item as i32) - (b'A' as i32) + 27;
    } else if b'a' <= item && item <= b'z' {
        return (item as i32) - (b'a' as i32) + 1;
    } else {
        panic!();
    }
}

fn find_badge(a: &HashSet<u8>, b: &HashSet<u8>, c: &HashSet<u8>) -> Option<u8> {
    a.intersection(b).find(|item| c.contains(item)).copied()
}
