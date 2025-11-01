// Route: GET "/health-check"
pub async fn health_check<'a>() -> &'a str {
    "OK"
}
