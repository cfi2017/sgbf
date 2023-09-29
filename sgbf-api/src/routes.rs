pub mod reservation;
pub mod members;

pub async fn status() -> &'static str {
    // todo: better status
    "OK"
}
