use warp::http::StatusCode;
use warp::test::request;

use super::{
    filters,
    models::{self, Book},
};

#[tokio::test]
async fn test_welcome() {
    let db = models::blank_db();
    let api = filters::books(db);
    let resp = request()
        .method("GET")
        .path("/")
        //.header("X-Forwarded-For", "127.0.0.1")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}
#[tokio::test]
async fn test_post() {
    let db = models::blank_db();
    let api = filters::books(db);

    let resp = request()
        .method("POST")
        .path("/books")
        .json(&Book {
            id: 1 as u64,
            title: "War and Peace".into(),
            author: "Leo Tolstoy".into(),
        })
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_post_conflict() {
    let db = models::blank_db();
    db.lock().await.push(book1());
    let api = filters::books(db);

    let resp = request()
        .method("POST")
        .path("/books")
        .json(&book1())
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_put_unknown() {
    let _ = pretty_env_logger::try_init();
    let db = models::blank_db();
    let api = filters::books(db);

    let resp = request()
        .method("PUT")
        .path("/books/1")
        .header("authorization", "Bearer admin")
        .json(&book1())
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

fn book1() -> Book {
    Book {
        id: 1 as u64,
        title: "War and Peace".into(),
        author: "Leo Tolstoy".into(),
    }
}
