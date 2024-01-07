// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function Perimeter
trait Perimeter {
    fn calculate(&self);
}

struct Square {
    side: i32
}

impl Perimeter for Square {
    fn calculate(&self) {
        println!("perimeter {:?}", self.side * 4);
    }
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Perimeter for Triangle {
    fn calculate(&self) {
        println!("perimeter {:?}", self.a + self.b + self.c);
    }
}

fn cal(shape : impl Perimeter) {
    shape.calculate();
}
fn main() {
    let sq = Square {side : 5};
    let tri = Triangle {a : 3, b : 4, c : 5};

    cal(sq);
    cal(tri);
}
