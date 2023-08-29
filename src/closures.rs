use crate::enum_match::vectorize;

fn add_sm(num1: i8, num2: i8) -> i8 {
    let add = |x: i8, y: i8| x + y;
    return add(num1, num2);
}

pub fn simulate() {
    // simulate add
    let n1 = rand::random::<i8>();
    let n2 = rand::random::<i8>();
    println!("SUM {n1}+{n2} = {}", add_sm(n1, n2));

    // simulate vector functional programming manipulation
    let nums = vectorize!(20, i8);

    let nums_modified: Vec<i8> = nums
        .iter()
        .map(|n| n * 1.1 as i8)
        .filter(|n| n % 2 as i8 == 0 as i8)
        .filter(|n| n < &0)
        .collect();
    println!("Numbers {nums_modified:?}")
}
