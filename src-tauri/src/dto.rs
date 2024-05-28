#![allow(non_snake_case)]

#[derive(Clone, serde::Serialize)]
pub struct ConfirmationModalInputPayload {
    pub title: String,
    pub message: String,
    pub loading: bool,
    pub parent: String,
}

#[derive(Clone, serde::Serialize)]
pub struct ConfirmationModalOutputPayload {
    pub button: i32,
}

#[derive(Clone, serde::Serialize)]
pub struct InputModalInputPayload {
    pub title: String,
    pub label: String,
    pub value: Option<String>,
    pub placeholder: String,
    pub buttonText: String,
    pub loading: bool,
    pub parent: String,
}

#[derive(Clone, serde::Serialize)]
pub struct InputModalOutputPayload {
    pub input: String,
}
