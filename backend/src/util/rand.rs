// rand::Rng;
//use rand::distributions::{Alphanumeric,DistString};

//use the uuid crate
use uuid::Uuid;

use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_random_uuid() -> String {
    //Alphanumeric.sample_string(&mut rand::thread_rng(), length)

    //use the uuid crate
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string();
    let uuid_string = uuid_string.replace("-", "");
    //let uuid_string = uuid_string[0..length].to_string();
    uuid_string
}

pub fn generate_random_alphanumeric(size: usize) -> String {
    let random_bytes: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size) // You can change the size
        .collect();

    base64::encode(&random_bytes)
}
