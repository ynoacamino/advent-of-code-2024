fn include(a: &Vec<usize>, e: usize) -> bool {
    if a.len() == 0 {
        return false;
    }
    for i in a {
        if *i == e {
            return true;
        }
    }

    false
}

pub fn day5_1(input: Vec<String>) {
    let data1: Vec<Vec<usize>> = input
        .iter()
        .filter(|x| x.len() == 5)
        .map(|line| line.split("|").map(|n| n.parse().unwrap()).collect())
        .collect();

    let data2: Vec<Vec<usize>> = input
        .iter()
        .filter(|x| x.len() > 5)
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut nums_after: Vec<Vec<usize>> = vec![Vec::new(); 100];

    for line in data1.iter() {
        nums_after[line[0]].push(line[1]);
    }

    let mut response = 0;

    for row in data2 {
        let mut correct = true;

        for i in 0..row.len() {
            for j in i + 1..row.len() {
                if !include(&nums_after[row[i]], row[j]) {
                    correct = false;
                    break;
                }
            }
            for j in 0..i {
                if include(&nums_after[row[i]], row[j]) {
                    correct = false;
                    break;
                }
            }

            if !correct {
                break;
            }
        }
        if correct {
            let half = (row.len() - 1) / 2;
            response += row[half];
        }
    }

    println!("RESPONSE: {}", response);
}

pub fn day5_2(input: Vec<String>) {
    let data1: Vec<Vec<usize>> = input
        .iter()
        .filter(|x| x.len() == 5)
        .map(|line| line.split("|").map(|n| n.parse().unwrap()).collect())
        .collect();

    let data2: Vec<Vec<usize>> = input
        .iter()
        .filter(|x| x.len() > 5)
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut nums_after: Vec<Vec<usize>> = vec![Vec::new(); 100];

    for line in data1.iter() {
        nums_after[line[0]].push(line[1]);
    }

    let mut incorrect_rows: Vec<Vec<usize>> = Vec::new();

    for row in data2 {
        let mut correct = true;

        for i in 0..row.len() {
            for j in i + 1..row.len() {
                if !include(&nums_after[row[i]], row[j]) {
                    correct = false;
                    break;
                }
            }
            for j in 0..i {
                if include(&nums_after[row[i]], row[j]) {
                    correct = false;
                    break;
                }
            }

            if !correct {
                break;
            }
        }
        if !correct {
            incorrect_rows.push(row);
        }
    }

    let mut response = 0;

    incorrect_rows.iter().for_each(|row| {
        let mut current_row = row.clone();
        current_row.sort_by(|&a, &b| {
            if include(&nums_after[a], b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        let half = (current_row.len() - 1) / 2;
        response += current_row[half];
    });

    println!("RESPONSE: {}", response);
}
