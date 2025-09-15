
// fn get_str() -> &String {
//   let s = String::from("get str");
//   &s
// }

fn get_str_fix() -> String {
  String::from("HADJIW")
}

use::std::borrow::Cow;
fn get_str_fix_use_cow<'a>() -> Cow<'a, str> {
  Cow::Owned(String::from("Using Cow"))
}

fn main() {
    // let s = get_str();
    let s = get_str_fix();
    println!("{}", s);
    let s1 = get_str_fix_use_cow();
    println!("{}", s1);
}
