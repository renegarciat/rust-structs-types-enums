enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64)
}

impl Shape {
    fn get_area(&self) -> f64 {
        /* Area forumula for the given 2D shapes */
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(length) => length * length / 2.0
        }
    }
    fn get_shape_type(&self) -> String {
        match self {
            Shape::Circle(radius) => format!("Circle with radius {}",radius),
            Shape::Square(length) => format!("Square with side length {}",length),
            Shape::Triangle(length) => format!("Triangle with side length {}",length)
        }
    }
}

fn get_areas(shapes: &Vec<Shape>) -> Vec<f64> {
    let areas = shapes
    .iter()
    .map(|shape| shape.get_area())
    .collect();
    areas
}
fn get_max_area(shapes: Vec<Shape>) -> Option<f64> {
    if shapes.is_empty() {
        None
    } else {
        let area = get_areas(&shapes)
        .into_iter()
        .reduce(f64::max)
        .unwrap();
        Some(area)
    }
}
fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| shape.get_area())
        .sum();

    println!("Sum of all areas: {} sq. units", total_area);
    println!("Print areas: {:?} sq. units",get_areas(&shapes));

    match get_max_area(shapes) {
        Some (max) => println!("Max area value: {} sq. units",max),
        None => println!("The vector is empty!"),
    }
}
