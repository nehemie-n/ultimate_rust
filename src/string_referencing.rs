fn get_me_i32(num: &mut i32) {
    *num = 54;
}

fn get_me_string(s: &str) {
    println!("Immutable referencing {s}")
}

pub fn simulate() {
    // Mutable borrowing
    let mut number: i32 = 32;
    println!("Before mutable borrowing {number}");
    get_me_i32(&mut number);
    println!("After mutable borrowing {number}");

    // &str
    let smut = "Hello There";
    get_me_string(smut)
}
