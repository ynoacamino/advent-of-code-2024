#[cfg(test)]
mod tests {
    use super::super::super::utils::utils::read_input;
    use super::super::day1;

    #[test]
    fn test_day1_1() {
        let input = read_input("day1.txt");
        day1::day1_1(input);
    }

    #[test]
    fn test_day1_2() {
        let input = read_input("day1.txt");
        day1::day1_2(input);
    }
}
