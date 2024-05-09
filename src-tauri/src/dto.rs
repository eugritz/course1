#[derive(Clone, serde::Serialize)]
pub struct ConfirmationModalInputPayload {
    pub title: String,
    pub message: String,
    pub loading: bool,
}

#[derive(Clone, serde::Serialize)]
pub struct ConfirmationModalOutputPayload {
    pub button: i32,
}
