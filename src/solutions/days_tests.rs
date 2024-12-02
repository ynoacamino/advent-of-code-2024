#[cfg(test)]
mod tests {
    use super::super::super::utils::utils::read_input;
    use super::super::day1;

    #[test]
    fn test_day1_1() {
        read_input("day1.txt")
            .iter()
            .for_each(|s| println!("{}", s));
        day1::day1_1();
    }
}
