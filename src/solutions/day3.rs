use regex::Regex;

pub fn day3_1(input: Vec<String>) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut response = 0;
    input.iter().for_each(|text| {
        for cap in re.captures_iter(&text) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();

            response += x * y;
        }
    });

    println!("{}", response)
}

pub fn day3_2(input: Vec<String>) {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut response = 0;

    input.iter().for_each(|text| {
        let mut dos_end: Vec<usize> = Vec::new();
        let mut donts_end: Vec<usize> = Vec::new();

        dos_end.push(0);

        let mut current_do: usize = 0;
        let mut current_dont: usize = 0;

        for cap in re_do.find_iter(&text) {
            dos_end.push(cap.end());
        }
        for cap in re_dont.find_iter(&text) {
            donts_end.push(cap.end());
        }

        donts_end.push(text.len());

        let mut last_index = 0;

        while current_do < dos_end.len() && current_dont < donts_end.len() {
            if dos_end[current_do] < last_index {
                current_do += 1;
                continue;
            }

            if donts_end[current_dont] < dos_end[current_do] {
                current_dont += 1;
                continue;
            }

            let current_text = &text[dos_end[current_do]..donts_end[current_dont]];

            for cap in re_mul.captures_iter(current_text) {
                println!("{} * {}a", &cap[1], &cap[2]);
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                response += x * y;
            }

            last_index = donts_end[current_dont];
        }
    });

    println!("{}", response)
}
