pub fn day2_1(input: Vec<String>) {
    let mut data: Vec<Vec<usize>> = Vec::new();

    input.iter().for_each(|str| {
        let line_arr = str
            .split(" ")
            .map(|str_num| str_num.parse().unwrap())
            .collect();

        data.push(line_arr);
    });

    let mut response = 0;

    data.iter().for_each(|line| {
        let mut direction = 0;

        if line[1] > line[0] {
            direction = 1;
        } else if line[1] < line[0] {
            direction = -1;
        }

        let mut result: i32 = 1;

        for i in 0..line.len() {
            if i <= line.len() - 2 {
                let current_direction;

                if line[i + 1] > line[i] {
                    current_direction = 1;
                } else {
                    current_direction = -1;
                }

                if current_direction != direction {
                    result = 0;
                    break;
                }

                if line[i + 1].abs_diff(line[i]) == 0 || line[i + 1].abs_diff(line[i]) >= 4 {
                    result = 0;
                    break;
                }
            }
        }

        response += result;
    });

    print!("{}", response);
}

pub fn day2_2(input: Vec<String>) {
    let mut data: Vec<Vec<Vec<usize>>> = Vec::new();

    input.iter().for_each(|str| {
        let mut variants_line: Vec<Vec<usize>> = Vec::new();

        let line_arr: Vec<usize> = str
            .split(" ")
            .map(|str_num| str_num.parse().unwrap())
            .collect();

        variants_line.push(line_arr.clone());

        for i in 0..line_arr.len() {
            let mut copy_line_arr = line_arr.clone();
            copy_line_arr.remove(i);
            variants_line.push(copy_line_arr);
        }

        data.push(variants_line);
    });

    let mut response = 0;

    data.iter().for_each(|variants| {
        for line in variants {
            let mut direction = 0;

            if line[1] > line[0] {
                direction = 1;
            } else if line[1] < line[0] {
                direction = -1;
            }

            let mut result: i32 = 1;

            for i in 0..line.len() {
                if i <= line.len() - 2 {
                    let current_direction;

                    if line[i + 1] > line[i] {
                        current_direction = 1;
                    } else {
                        current_direction = -1;
                    }

                    if current_direction != direction {
                        result = 0;
                        break;
                    }

                    if line[i + 1].abs_diff(line[i]) == 0 || line[i + 1].abs_diff(line[i]) >= 4 {
                        result = 0;
                        break;
                    }
                }
            }

            if result == 1 {
                response += 1;
                break;
            }
        }
    });

    println!("{}", response);
}
