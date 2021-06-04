use chrono::Local;
use weekly;

fn main() {
    let foo = weekly::weeks_for_month(Local::now().date());

    println!("{:?}", foo);

    println!("Hello, world!");
}
