use crate::back_of_house;
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
