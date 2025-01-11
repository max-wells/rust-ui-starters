use actix_web::{web, HttpResponse, Responder, Scope};

use crate::models::model_todos::{CreateTodoBody, Todo, UpdateTodoBody};
use crate::models::model_utils::MyResponse;
use crate::AppState;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      🦀 ENTRYPOINT 🦀                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_todos() -> Scope {
    web::scope("/todos")
        .route("", web::get().to(get_all_todos))
        .route("", web::post().to(create_todo))
        .route("/{todo_id}", web::get().to(get_todo))
        .route("/{todo_id}", web::put().to(update_todo))
        .route("/{todo_id}", web::delete().to(delete_todo))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨  QUERIES  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

async fn get_all_todos(app_state: web::Data<AppState>) -> impl Responder {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY id ASC")
        .fetch_all(&app_state.pool)
        .await;

    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            eprintln!("Failed to fetch todos: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to fetch todos".to_string(),
            })
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

async fn create_todo(
    body: web::Json<CreateTodoBody>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, description, completed)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        body.title,
        body.description,
        body.completed.unwrap_or(false)
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(e) => {
            eprintln!("Failed to create todo: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to create todo: {}", e),
            })
        }
    }
}

async fn get_todo(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let todo_id: i32 = path.into_inner();

    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", todo_id)
        .fetch_optional(&app_state.pool)
        .await;

    match todo {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().json(MyResponse {
            message: "Todo not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "An error occurred while retrieving the todo".to_string(),
            })
        }
    }
}

async fn update_todo(
    path: web::Path<i32>,
    body: web::Json<UpdateTodoBody>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner();

    let result = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            completed = COALESCE($3, completed)
        WHERE id = $4
        RETURNING *
        "#,
        body.title,
        body.description,
        body.completed,
        todo_id
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(updated_todo) => HttpResponse::Ok().json(updated_todo),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(MyResponse {
            message: "Todo not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to update todo: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to update todo: {}", e),
            })
        }
    }
}

async fn delete_todo(todo_id: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", todo_id.into_inner())
        .execute(&app_state.pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(MyResponse {
                    message: "Todo not found".to_string(),
                })
            } else {
                HttpResponse::Ok().json(MyResponse {
                    message: "✅ Todo deleted successfully".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Failed to delete todo: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to delete todo".to_string(),
            })
        }
    }
}
