#[cfg(test)]
mod tests {
    use super::super::super::utils::utils::read_input;
    use super::super::day1;
    use super::super::day2;
    use super::super::day3;

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

    #[test]
    fn test_day2_1() {
        let input = read_input("day2.txt");
        day2::day2_1(input);
    }

    #[test]
    fn test_day2_2() {
        let input = read_input("day2.txt");
        day2::day2_2(input);
    }

    #[test]
    fn test_day3_1() {
        let input = read_input("day3.txt");
        day3::day3_1(input);
    }

    #[test]
    fn test_day3_2() {
        let input = read_input("day3_test.txt");
        day3::day3_2(input);
    }
}
