use snx::router::Router;

struct App {}

impl snx::App for App {
    fn with_routes() -> Router {
        Router::builder()
            .get("/", |_req| "welcome to my portfolio")
            .build()
            .unwrap()
    }
}

fn main() {
    snx::boot::<App>()
}
