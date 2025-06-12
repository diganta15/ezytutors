pub mod state;
pub mod handlers;
pub mod routes;
pub mod models;



use state::AppState; // ← does this compile?

fn _test() {
    let _ = AppState {
        health_check_response: "hi".to_string(),
        visit_count: std::sync::Mutex::new(0),
    };
}