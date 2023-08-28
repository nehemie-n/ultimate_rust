fn get_me_string(s: &str) {
    println!("Immutable referencing {s}")
}

pub fn simulate() {
    let s = "Hello There";
    get_me_string(s)
}
