fn main() {
    let mut sums: Vec<i32> = read_elves().iter().map(|elf| elf.iter().sum()).collect();
    sums.sort_unstable();
    sums.reverse();
    let total: i32 = sums.iter().take(3).sum();
    println!("{}", total);
}

fn read_elves() -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    for line_ in std::io::stdin().lines() {
        let line = line_.unwrap();
        if let Ok(num) = line.parse::<i32>() {
            current.push(num);
        } else if !current.is_empty() {
            result.push(current);
            current = Vec::new();
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}
