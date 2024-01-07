struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait ConveyItem {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: ConveyItem> {
    pub items: Vec<T>,
}

impl<T : ConveyItem> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_string()
        }
    }
}

impl ConveyItem for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width : self.width,
            height : self.height,
            depth : self.depth,
        }
    }
}


fn main() {
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items : vec![] };
    belt.add(CarPart::default());

    // let mut belt = ConveyorBelt { items : vec![]};
    
}