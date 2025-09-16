struct Color(i32, i32, i32);

struct Marker;

fn print_color(color: &Color) -> String {
  format!("({},{},{})", color.0, color.1, color.2)
}

fn main() {
    let color = Color(255, 0, 255);
    let color_str = print_color(&color);
    println!("{}", color_str);
}
