mod front_of_house; //older style, using module/mod.rs

mod back_of_house; // newer style, using module.rs

mod customer;

fn deliver_order() {
    customer::eat_at_restaurant();
}
