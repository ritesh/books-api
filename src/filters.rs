use super::models::{Book, Db, ListOptions};
use crate::handlers;

use warp::Filter;

pub fn welcome() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let host = warp::header::optional::<String>("host");
    warp::path::end()
        .and(warp::get())
        .and(host)
        .and_then(handlers::welcome)
}

/// The 4 books filters combined
pub fn books(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    welcome()
        .or(books_list(db.clone()))
        .or(books_create(db.clone()))
        .or(books_update(db.clone()))
        .or(books_delete(db))
}

/// GET /books?offset=3&limit=5
pub fn books_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_books)
}

/// POST /books with JSON body
pub fn books_create(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_book)
}

/// PUT /books/:id with JSON body
pub fn books_update(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_book)
}

/// DELETE /books/:id
pub fn books_delete(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("books" / u64)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /todos/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(admin_only)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_book)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Book,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
