use rand;
// Define an enum called "Exercise" with different exercise types

#[derive(Debug, Clone)]
enum Coordinate {
    Lat(f64),
    Lng(f64),
    Point(f64, f64),
    Polygon(Vec<f64>),
}

macro_rules! vectorize {
    ($size: expr, $type: ty) => {{
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut numbers = Vec::new();
        for _ in 0..$size {
            numbers.push(rng.gen::<$type>());
        }
        numbers
    }};
}

///
/// Generate random coordinates
///
/// random_coordintes();
///
fn random_coordintes() -> Vec<Coordinate> {
    let mut coords: Vec<Coordinate> = Vec::new();
    for i in 0..100 {
        coords.push(Coordinate::Lat(rand::random::<f64>()));
        coords.push(Coordinate::Lng(rand::random::<f64>() * i as f64));
        coords.push(Coordinate::Point(
            rand::random::<f64>(),
            rand::random::<f64>(),
        ));
        coords.push(Coordinate::Polygon(vectorize!(rand::random::<i8>(), f64)))
    }
    coords
}

fn simulate_enums() {
    let coords = random_coordintes();

    let mut lats: i32 = 0;
    let mut lngs = 0;
    let mut points = 0;
    let mut polygons = 0;
    for coo in coords {
        match coo {
            Coordinate::Lat(_) => lats += 1,
            Coordinate::Lng(_) => lngs += 1,
            Coordinate::Point(_, _) => points += 1,
            Coordinate::Polygon(_) => polygons += 1,
            #[allow(unreachable_patterns)]
            _ => println!("REST"), // will not be executed since all conditions are exhausted
        }
    }

    println!("lats {lats}");
    println!("lngs {lngs}");
    println!("points {points}");
    println!("polygons {polygons}");
}

fn simulate_options() {
    let n = Some(5);

    println!("Is N Some? {}", n.is_some());
    match n {
        Some(_) => println!("n is Some"),
        None => println!("n is None"),
    }

    let result = if n.is_none() {
        String::from("No result")
    } else {
        String::from("Some result")
    };
    println!("n result = {result}")
}
pub fn simulate() {
    simulate_enums();
    simulate_options();
}

pub(crate) use vectorize;
