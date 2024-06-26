fn hash1(astr: &str, size: usize) -> usize {
    let mut sum = 0;
    for (i, c) in astr.chars().enumerate() {
        sum += (i + 1) * (c as usize);
    }
    sum % size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash1() {
        let size = 11;
        let s1 = "rust"; let s2 = "Rust";
        let p1 = hash1(s1, size);
        let p2 = hash1(s2, size);
        println!("{} {}", p1, p2);
    }
}
