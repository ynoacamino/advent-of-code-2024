pub fn query1(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if j + k < data[i].len() {
            word += data[i][j + k];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query2(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if j as i32 - k as i32 >= 0 {
            word += data[i][j - k];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query3(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if i + k < data.len() {
            word += data[i + k][j];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query4(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if i as i32 - k as i32 >= 0 {
            word += data[i - k][j];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query5(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if j + k < data[i].len() && i + k < data.len() {
            word += data[i + k][j + k];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query6(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if j as i32 - k as i32 >= 0 && i as i32 - k as i32 >= 0 {
            word += data[i - k][j - k];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query7(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if j + k < data[i].len() && i as i32 - k as i32 >= 0 {
            word += data[i - k][j + k];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn query8(data: &Vec<Vec<&str>>, i: usize, j: usize) -> usize {
    let word_match = "XMAS";
    let mut word = "".to_string();
    for k in 0..word_match.len() {
        if j as i32 - k as i32 >= 0 && i + k < data.len() {
            word += data[i + k][j - k];
        }
    }

    if word == word_match {
        return 1;
    }

    0
}

pub fn day4_1(input: Vec<String>) {
    let mut data: Vec<Vec<&str>> = Vec::new();

    input.iter().for_each(|line| {
        let mut row: Vec<&str> = Vec::new();

        line.split("").for_each(|c| {
            row.push(c);
        });

        data.push(row);
    });

    let mut count: usize = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == "X" {
                count += query1(&data.clone(), i, j);
                count += query2(&data.clone(), i, j);
                count += query3(&data.clone(), i, j);
                count += query4(&data, i, j);
                count += query5(&data.clone(), i, j);
                count += query6(&data.clone(), i, j);
                count += query7(&data.clone(), i, j);
                count += query8(&data.clone(), i, j);
            }
        }
    }

    println!("{}aw", count);
}

pub fn day4_2(input: Vec<String>) {
    let mut data: Vec<Vec<&str>> = Vec::new();

    input.iter().for_each(|line| {
        let mut row: Vec<&str> = Vec::new();

        line.split("").for_each(|c| {
            row.push(c);
        });

        data.push(row);
    });

    let mut count: usize = 0;

    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            if data[i][j] == "A" {
                let trys = vec![
                    vec!["M", "M", "S", "S"],
                    vec!["M", "S", "M", "S"],
                    vec!["S", "M", "S", "M"],
                    vec!["S", "S", "M", "M"],
                ];

                for try_ in trys {
                    if data[i - 1][j - 1] == try_[0]
                        && data[i - 1][j + 1] == try_[1]
                        && data[i + 1][j - 1] == try_[2]
                        && data[i + 1][j + 1] == try_[3]
                    {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}aw", count);
}
