use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct AndroidPath {
    pub value: String,
}
