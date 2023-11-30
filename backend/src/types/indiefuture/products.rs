use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductComplex {
    pub product: Product,

    pub favorited_product: Option<serde_json::Value>,

    pub classification: Option<serde_json::Value>,

    pub best_price: Option<serde_json::Value>,

    pub shop: Option<serde_json::Value>,

    pub cover_image: Option<serde_json::Value>,

    pub activated_chain_ids: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: i32,
    pub sku: String,
    pub title: String,
    pub description_short: Option<String>, // Allowing for nullable column
    pub description_long: Option<String>,  // Allowing for nullable column
    //pub parent_public_address: String,
    pub parent_shop_id: i32,
    pub payment_beacon_id: Option<i32>, // Allowing for nullable foreign key column
    pub product_classification_id: Option<i32>, // Allowing for nullable foreign key column

    pub requires_payment: bool,
    pub fulfillment_type: String,

    pub enabled: bool,
    pub visible: bool,

    pub cover_image_hash: Option<String>, // Allowing for nullable foreign key column
    pub banner_image_hash: Option<String>, // Allowing for nullable foreign key column
    pub stock_quantity: i32,
    // pub created_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductOutput {
    pub product_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductClassification {
    pub id: i32,
    pub name: String,
    pub label: String,
    //   pub created_at: ChronoDateTime,
}
