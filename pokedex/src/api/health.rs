use super::Response;

pub fn serve() -> rouille::Response {
    rouille::Response::json(&Response {
        message: "Ok".to_string(),
    })
}
