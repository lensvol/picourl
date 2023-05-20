use axum::{extract::Path, response::Redirect, routing::get, Router};

#[derive(Debug)]
pub struct Redirection {
    pub slug: &'static str,
    pub url: &'static str,
}

const REDIRECTIONS: &'static [Redirection] =
    &include!(concat!(env!("OUT_DIR"), "/generated_redirections.rs"));

const FALLBACK_URL: &'static str = "https://lensvol.dev";

async fn redirector(Path(slug): Path<String>) -> Redirect {
    if let Some(redirect) = REDIRECTIONS.iter().find(|r| r.slug == slug.as_str()) {
        return Redirect::temporary(redirect.url);
    };

    Redirect::temporary(FALLBACK_URL)
}

async fn fallback() -> Redirect {
    Redirect::temporary(FALLBACK_URL)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:slug", get(redirector))
        .route("/", get(fallback));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
