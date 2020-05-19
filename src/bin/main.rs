use rusty_shop::*;

fn main() {
    let conn = database::establish_connection();
    let products = database::get_all_products(&conn, 10);
    println!("Products:\n{:?}", products);
}
