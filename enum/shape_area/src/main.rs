enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r.powi(2),
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(w, h) => 0.5 * (w * h),
    }
}

fn main() {
    let circle = Shape::Circle(2.0);
    println!("Area of circle shape is {}", area(&circle));
    let rectangle = Shape::Rectangle(5.0, 3.4);
    println!("Area of rectangle shape is {}", area(&rectangle));
    let triangle = Shape::Triangle(5.0, 3.4);
    println!("Area of triangle shape is {}", area(&triangle));
}
