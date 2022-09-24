use std::{env, ops::Not};

mod auth;
mod banking;
mod co2;
mod fuel;
mod hanger;
mod utilities;

fn main() {
    let username: String = env::var("USER").unwrap_or_default();
    let password: String = env::var("PASS").unwrap_or_default();

    let cookies: String = auth::login(&username, &password);

    if cookies.contains("PHPSESSID=").not() {
        panic!("Login failed!");
    }

    let balance: i64 = banking::get_balance(&cookies);
    println!("Current Bank Balance is {}", balance);
    println!();

    let (fuel_price, fuel_capacity, fuel_holding, fuel_to_buy): (i16, i32, i32, i32) =
        fuel::get_status(&cookies);
    println!("Current Fuel Price is {}", fuel_price);
    println!(
        "Currently holding {}/{} capacity",
        fuel_holding, fuel_capacity
    );
    println!("Can buy a maximum of {} lbs of fuel", fuel_to_buy);
    println!();

    let (co2_price, co2_capacity, co2_holding, co2_to_buy, airline_status): (
        i16,
        i32,
        i32,
        i32,
        String,
    ) = co2::get_status(&cookies);
    println!("Current co2 Price is {}", co2_price);
    println!(
        "Currently holding {}/{} capacity",
        co2_holding, co2_capacity
    );
    println!("Can buy a maximum of {} lbs of co2", co2_to_buy);
    println!("Airline status is {}", airline_status);
    println!();

    let (pax_hanger_capacity, pax_hanger_in_use, pax_hanger_free): (i16, i16, i16) =
        hanger::get_pax_status(&cookies);
    println!(
        "Current pax hanger usage is {}/{}",
        pax_hanger_in_use, pax_hanger_capacity
    );
    println!("Can buy a maximum of {} pax aircrafts", pax_hanger_free);
    println!();

    let (cargo_hanger_capacity, cargo_hanger_in_use, cargo_hanger_free): (i16, i16, i16) =
        hanger::get_cargo_status(&cookies);
    println!(
        "Current cargo hanger usage is {}/{}",
        cargo_hanger_in_use, cargo_hanger_capacity
    );
    println!("Can buy a maximum of {} cargo aircrafts", cargo_hanger_free);
    println!();

    auth::logout(cookies);
}
