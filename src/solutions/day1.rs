pub fn day1_1(input: Vec<String>) {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    input.iter().for_each(|str| {
        let arr: Vec<&str> = str.split("   ").collect();
        list1.push(arr[0].parse().unwrap());
        list2.push(arr[1].parse().unwrap());
    });

    list1.sort();
    list2.sort();

    let mut response: usize = 0;

    for i in 0..list1.len() {
        response += list1[i].abs_diff(list2[i]);
    }

    println!("The response is {}", response);
}

pub fn day1_2(input: Vec<String>) {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    input.iter().for_each(|str| {
        let arr: Vec<&str> = str.split("   ").collect();
        list1.push(arr[0].parse().unwrap());
        list2.push(arr[1].parse().unwrap());
    });

    let mut memo: Vec<usize> = vec![0; 100000];

    for i in list2 {
        memo[i] += 1;
    }

    let mut response = 0;

    for i in 0..list1.len() {
        response += list1[i] * memo[list1[i]];
    }

    println!("{}", response)
}
