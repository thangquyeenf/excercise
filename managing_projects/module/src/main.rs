use module::front_of_house::hosting;
use module::kitchen;
use module::back_of_house::Breakfast;

fn main() {
    hosting::add_to_waitlist();
    kitchen::fix_incorrect_order();
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}