use actix_web::{web,App,HttpServer};
use std::io;
use std::sync::Mutex;

// #[path="../handlers.rs"]
// mod handlers;
// #[path ="../routes.rs"]
// mod routes;
// #[path ="../state.rs"]
// mod state;


use tutor_nodb::routes::{course_routes, general_routes};
use tutor_nodb::state::AppState;


#[actix_web::main]
async fn main()->io::Result<()>{
let shared_data = web::Data::new(AppState{
    health_check_response:"I am good, you nigger".to_string(),
    visit_count:Mutex::new(0),
    courses:Mutex::new(vec![]),
});

let app = move || {
    App::new()
        .app_data(shared_data.clone())
        .configure(general_routes)
        .configure(course_routes)
        
};

HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}