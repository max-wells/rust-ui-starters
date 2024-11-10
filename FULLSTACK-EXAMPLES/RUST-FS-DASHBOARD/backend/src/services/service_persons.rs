use actix_web::{web, HttpResponse, Responder, Scope};

use crate::models::model_person::Person;
use crate::models::model_utils::MyResponse;
use crate::AppState;

// curl -X DELETE http://localhost:8000/persons/10
// curl -X POST http://localhost:8000/persons -H "Content-Type: application/json" -d '{"name": "Maxime", "title": "Software Engineer", "level": "Senior", "compensation": 75000, "joined_date": "2023-10-15"}'

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_persons() -> Scope {
    web::scope("/persons")
        .route("", web::get().to(get_all_persons))
        .route("", web::post().to(create_person))
        .route("/{person_id}", web::get().to(get_person))
        .route("/{person_id}", web::delete().to(delete_person))
        .route("/{person_id}", web::put().to(update_person))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨  QUERIES  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn get_all_persons(app_state: web::Data<AppState>) -> impl Responder {
    let persons = sqlx::query_as!(Person, "SELECT * FROM persons ORDER BY id ASC")
        .fetch_all(&app_state.pool)
        .await
        .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(persons)
}

pub async fn get_person(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let person_id: i32 = path.into_inner();

    let person = sqlx::query_as!(Person, "SELECT * FROM persons WHERE id = $1", person_id)
        .fetch_optional(&app_state.pool)
        .await;

    match person {
        Ok(Some(person)) => HttpResponse::Ok().json(person),
        Ok(None) => HttpResponse::NotFound().json(MyResponse {
            message: "Person not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "An error occurred while retrieving the person".to_string(),
            })
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn create_person(
    body: web::Json<Person>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Person,
        "INSERT INTO persons (title, name, level, compensation, joined_date) VALUES ($1, $2, $3, $4, $5) RETURNING id, title, name, level, compensation, joined_date",
        body.title,
        body.name,
        body.level,
        body.compensation,
        body.joined_date
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(e) => {
            eprintln!("Failed to create person: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to create person: {}", e),
            })
        }
    }
}

pub async fn delete_person(
    book_id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM persons WHERE id = $1", book_id.into_inner())
        .execute(&app_state.pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(MyResponse {
                    message: "Person not found".to_string(),
                })
            } else {
                HttpResponse::Ok().json(MyResponse {
                    message: "✅ Person deleted successfully".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Failed to delete person: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to delete person".to_string(),
            })
        }
    }
}

pub async fn update_person(
    path: web::Path<i32>,
    body: web::Json<Person>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let book_id = path.into_inner();

    let result = sqlx::query_as!(
        Person,
        "UPDATE persons SET title = $1, name = $2, level = $3, compensation = $4, joined_date = $5 WHERE id = $6 RETURNING id, title, name, level, compensation, joined_date",
        body.title,
        body.name,
        body.level,
        body.compensation,
        body.joined_date,
        book_id
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(updated_book) => HttpResponse::Ok().json(updated_book),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(MyResponse {
            message: "Person not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to update person: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to update person: {}", e),
            })
        }
    }
}
