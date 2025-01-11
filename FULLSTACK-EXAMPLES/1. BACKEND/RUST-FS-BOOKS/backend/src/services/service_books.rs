use actix_web::{web, HttpResponse, Responder, Scope};

use crate::models::model_books::Book;
use crate::models::model_utils::MyResponse;
use crate::AppState;

// curl -X DELETE http://localhost:8000/books/10
// curl -X POST http://localhost:8000/books -H "Content-Type: application/json" -d '{"title": "Your Book Title", "author": "Author Name"}'

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_books() -> Scope {
    web::scope("/books")
        .route("", web::get().to(get_all_books))
        .route("", web::post().to(create_book))
        .route("/{book_id}", web::get().to(get_book))
        .route("/{book_id}", web::delete().to(delete_book))
        .route("/{book_id}", web::put().to(update_book))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨  QUERIES  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn get_all_books(app_state: web::Data<AppState>) -> impl Responder {
    let books = sqlx::query_as!(Book, "SELECT * FROM books ORDER BY id ASC")
        .fetch_all(&app_state.pool)
        .await
        .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(books)
}

pub async fn get_book(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let book_id: i32 = path.into_inner();

    let book = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", book_id)
        .fetch_optional(&app_state.pool)
        .await;

    match book {
        Ok(Some(book)) => HttpResponse::Ok().json(book),
        Ok(None) => HttpResponse::NotFound().json(MyResponse {
            message: "Book not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "An error occurred while retrieving the book".to_string(),
            })
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn create_book(body: web::Json<Book>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        Book,
        "INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id, title, author",
        body.title,
        body.author
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => {
            eprintln!("Failed to create book: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to create book: {}", e),
            })
        }
    }
}

pub async fn delete_book(
    book_id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM books WHERE id = $1", book_id.into_inner())
        .execute(&app_state.pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(MyResponse {
                    message: "Book not found".to_string(),
                })
            } else {
                HttpResponse::Ok().json(MyResponse {
                    message: "✅ Book deleted successfully".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Failed to delete book: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to delete book".to_string(),
            })
        }
    }
}

pub async fn update_book(
    path: web::Path<i32>,
    body: web::Json<Book>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let book_id = path.into_inner();

    let result = sqlx::query_as!(
        Book,
        "UPDATE books SET title = $1, author = $2 WHERE id = $3 RETURNING id, title, author",
        body.title,
        body.author,
        book_id
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(updated_book) => HttpResponse::Ok().json(updated_book),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(MyResponse {
            message: "Book not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to update book: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to update book: {}", e),
            })
        }
    }
}
