use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct AndroidPath {
    pub path: String,
}
