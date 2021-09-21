use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Conversion {
    pub id: i32,
    pub name: String,
    pub formula: String
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct ConversionRequest {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ConversionResponse {
    pub id: i32,
    pub name: String,
    pub formula: String
}

impl ConversionResponse {
    pub fn of(conversion: Conversion) -> ConversionResponse {
        ConversionResponse {
            id: conversion.id,
            name: conversion.name,
            formula: conversion.formula,
        }
    }
}
