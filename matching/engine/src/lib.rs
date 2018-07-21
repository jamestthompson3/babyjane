#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct RequestAction {
    class: u8,
    price: usize,
    quantity: usize,
}

pub fn decode_text(text: &str) -> Result<(), Error> {
    let p: RequestAction = serde_json::from_str(text)?;
    println!(
        "text decoded {} requested at {} price and {} quantity",
        p.class, p.price, p.quantity
    );
    Ok(())
}
