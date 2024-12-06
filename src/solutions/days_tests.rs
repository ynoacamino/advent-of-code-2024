#[cfg(test)]
mod tests {
    use super::super::super::utils::utils::read_input;
    use super::super::day1;
    use super::super::day2;
    use super::super::day3;
    use super::super::day4;
    use super::super::day5;

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

    #[test]
    fn test_day4_1() {
        let input = read_input("day4.txt");
        day4::day4_1(input);
    }

    #[test]
    fn test_day4_2() {
        let input = read_input("day4.txt");
        day4::day4_2(input);
    }

    #[test]
    fn test_day5_1() {
        let input = read_input("day5.txt");
        day5::day5_1(input);
    }

    #[test]
    fn test_day5_2() {
        let input = read_input("day5.txt");
        day5::day5_2(input);
    }
}
