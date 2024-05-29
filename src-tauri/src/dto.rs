#![allow(non_snake_case)]

#[derive(Clone, serde::Serialize)]
pub struct ConfirmationModalInputPayload {
    pub id: Option<String>,
    pub title: String,
    pub message: String,
    pub loading: Option<bool>,
    pub parent: String,
}

#[derive(Clone, serde::Serialize)]
pub struct ConfirmationModalOutputPayload {
    pub id: Option<String>,
    pub button: i32,
}

#[derive(Clone, serde::Serialize)]
pub struct InputModalInputPayload {
    pub id: Option<String>,
    pub title: String,
    pub label: String,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub buttonText: Option<String>,
    pub loading: Option<bool>,
    pub parent: String,
}

#[derive(Clone, serde::Serialize)]
pub struct InputModalOutputPayload {
    pub id: Option<String>,
    pub input: String,
}
