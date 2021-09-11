use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Conversion {
    pub id: i32,
    pub name: String,
    pub formula: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct ConversionRequest {
    pub id: i32,
    pub name: String
}

impl OwnerResponse {
    pub fn of(owner: Owner) -> OwnerResponse {
        OwnerResponse {
            id: owner.id,
            name: owner.name,
            formula: owner.formula,
        }
    }
}
