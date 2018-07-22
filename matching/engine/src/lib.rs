#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde_json::Error;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct RequestAction {
    action: u32,
    price: f32,
    quantity: u32,
    id: u32,
}

pub struct Match((u32, f32, u32), (u32, f32, u32));

pub fn decode_text(text: &str) -> Result<(), Error> {
    let mut candidates: Vec<RequestAction> = Vec::new();
    let mut matches: Vec<Match> = Vec::new();
    let incoming_request: RequestAction = serde_json::from_str(text)?;
    candidates.push(incoming_request);
    // FIXME  WORKING UP TO HERE
    for candidate in candidates {
        println!("candidate {}", candidate.id);
        if candidate.action == incoming_request.action {
            matches.push(Match(
                (candidate.id, candidate.price, candidate.quantity),
                (
                    incoming_request.id,
                    incoming_request.price,
                    incoming_request.quantity,
                ),
            ));
        }
    }
    println!("Action complete");
    Ok(())
}
