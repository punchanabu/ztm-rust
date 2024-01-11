// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors



trait Body {}
trait Color {}

struct Truck {}
struct Car {}
struct Scooter {}

impl Body for Truck {}
impl Body for Car {}
impl Body for Scooter {}

struct Red;
struct Blue;
struct Green;

impl Color for Red {}
impl Color for Blue {}
impl Color for Green {}

struct Vehicle<B: Body, C: Color> {
    body : B,
    color : C,
}

impl<B : Body, C: Color> Vehicle<B,C> {
    pub fn new(body : B, color: C) -> Self {
        Vehicle { body, color }
    }
}



fn main() {
    let red_truck: Vehicle<Car, Red> = Vehicle::new(Car, Red);
    println!("Created a vehicle red truck");
    
    let white_scooter = Vehicle::new(Scooter, Green);
    println!("Created a vehicle!")

}
