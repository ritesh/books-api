use crate::models::{Book, Db, FixedResponse, ListOptions};
use hostname;
use std::convert::Infallible;
use std::ffi::OsString;

use warp::http::StatusCode;

pub async fn welcome(addr: Option<String>) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&FixedResponse {
        status: StatusCode::OK.to_string(),
        remote_address: String::from(addr.unwrap_or("unknown".into())),
        hostname: hostname::get()
            .unwrap_or(OsString::new())
            .into_string()
            .unwrap(),
    }))
}

pub async fn list_books(opts: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of books, applying the limit and offset.
    let t = db.lock().await;
    let books: Vec<Book> = t
        .clone()
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .collect();
    Ok(warp::reply::json(&books))
}

pub async fn create_book(create: Book, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("create_book: {:?}", create);

    let mut vec = db.lock().await;

    for book in vec.iter() {
        if book.id == create.id {
            log::debug!("    -> id already exists: {}", create.id);
            // book with id already exists, return `400 BadRequest`.
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    // No existing book with id, so insert and return `201 Created`.
    vec.push(create);
    Ok(StatusCode::CREATED)
}

pub async fn update_book(id: u64, update: Book, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_book: id={}, book={:?}", id, update);
    let mut vec = db.lock().await;

    // Look for the specified book...
    for book in vec.iter_mut() {
        if book.id == id {
            *book = update;
            return Ok(StatusCode::OK);
        }
    }

    log::debug!("    -> book id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_book(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_book: id={}", id);

    let mut vec = db.lock().await;

    let len = vec.len();
    vec.retain(|book| {
        // Retain all books that aren't this id...
        // In other words, remove all that *are* this id...
        book.id != id
    });

    // If the vec is smaller, we found and deleted a book!
    let deleted = vec.len() != len;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("    -> book id not found!");
        Ok(StatusCode::NOT_FOUND)
    }
}
