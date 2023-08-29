use rand::Rng;
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

pub fn simulate() {
    let coords = random_coordintes();
    for coo in coords {
        println!("{coo:?}");
    }
}
