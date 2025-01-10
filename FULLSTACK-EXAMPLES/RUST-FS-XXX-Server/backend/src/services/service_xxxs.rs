use actix_web::{web, HttpResponse, Responder, Scope};

use crate::models::model_utils::MyResponse;
use crate::models::model_xxx::Xxx;
use crate::AppState;

// curl -X DELETE http://localhost:8000/xxxs/10
// curl -X POST http://localhost:8000/xxxs -H "Content-Type: application/json" -d '{"field_one": "My field one", "field_two": "My field two"}'

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_xxxs() -> Scope {
    web::scope("/xxxs")
        .route("", web::get().to(get_all_xxxs))
        .route("", web::post().to(create_xxx))
        .route("/{xxx_id}", web::get().to(get_xxx))
        .route("/{xxx_id}", web::delete().to(delete_xxx))
        .route("/{xxx_id}", web::put().to(update_xxx))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨  QUERIES  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn get_all_xxxs(app_state: web::Data<AppState>) -> impl Responder {
    let all_xxxs = sqlx::query_as!(Xxx, "SELECT * FROM xxxs ORDER BY id ASC")
        .fetch_all(&app_state.pool)
        .await
        .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(all_xxxs)
}

pub async fn get_xxx(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let xxx_id: i32 = path.into_inner();

    let xxx = sqlx::query_as!(Xxx, "SELECT * FROM xxxs WHERE id = $1", xxx_id)
        .fetch_optional(&app_state.pool)
        .await;

    match xxx {
        Ok(Some(xxx)) => HttpResponse::Ok().json(xxx),
        Ok(None) => HttpResponse::NotFound().json(MyResponse {
            message: "xxx not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "An error occurred while retrieving the xxx".to_string(),
            })
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn create_xxx(body: web::Json<Xxx>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        Xxx,
        "INSERT INTO xxxs (field_one, field_two) VALUES ($1, $2) RETURNING id, field_one, field_two",
        body.field_one,
        body.field_two
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(xxx) => HttpResponse::Ok().json(xxx),
        Err(e) => {
            eprintln!("Failed to create xxx: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to create xxx: {}", e),
            })
        }
    }
}

pub async fn delete_xxx(xxx_id: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM xxxs WHERE id = $1", xxx_id.into_inner())
        .execute(&app_state.pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(MyResponse {
                    message: "xxx not found".to_string(),
                })
            } else {
                HttpResponse::Ok().json(MyResponse {
                    message: "✅ xxx deleted successfully".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Failed to delete xxx: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to delete xxx".to_string(),
            })
        }
    }
}

pub async fn update_xxx(
    path: web::Path<i32>,
    body: web::Json<Xxx>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let xxx_id = path.into_inner();

    let result = sqlx::query_as!(
        Xxx,
        "UPDATE xxxs SET field_one = $1, field_two = $2 WHERE id = $3 RETURNING id, field_one, field_two",
        body.field_one,
        body.field_two,
        xxx_id
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(updated_xxx) => HttpResponse::Ok().json(updated_xxx),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(MyResponse {
            message: "xxx not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to update xxx: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to update xxx: {}", e),
            })
        }
    }
}
