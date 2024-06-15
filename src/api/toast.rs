use serde::Serialize;

pub enum ToastLength {
    Long,
    Short,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ToastRequest {
    pub message: String,
    pub long: bool,
}

impl ToastRequest {
    pub(super) fn new(message: String, long: bool) -> Self {
        Self { message, long }
    }
}
