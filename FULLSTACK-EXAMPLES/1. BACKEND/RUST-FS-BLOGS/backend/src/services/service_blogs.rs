use actix_web::{web, HttpResponse, Result, Scope};

use crate::models::model_blogs::{
    Blog, BlogResponse, BlogWithParsedTags, BodyPatchBlog, CreateBlogBody, CreateTag, Tag,
};
use crate::models::model_utils::MyResponse;
use crate::AppState;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      🦀 ENTRYPOINT 🦀                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// curl -X POST http://localhost:8000/blogs -H "Content-Type: application/json" -d '{"title": "New Blog Post", "content": "This is the content of the new blog post.", "author": "Alice Johnson", "image_url": "https://example.com/image3.jpg", "tags": [1, 2]}'

pub fn service_blogs() -> Scope {
    web::scope("/blogs")
        .route("", web::get().to(get_all_blogs))
        .route("", web::post().to(create_blog))
        .route("/tags", web::get().to(get_all_tags))
        .route("/tags", web::post().to(create_tag))
        .route("/tags/{tag_id}", web::delete().to(delete_tag))
        .route("/{blog_id}", web::get().to(get_blog))
        .route("/{blog_id}", web::delete().to(delete_blog))
        .route("/{blog_id}", web::patch().to(patch_blog))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨  QUERIES  ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn get_all_blogs(app_state: web::Data<AppState>) -> HttpResponse {
    let blogs = sqlx::query_as!(
        Blog,
        r#"
        SELECT b.id, b.title, b.content, b.author, b.image_url,
               STRING_AGG(CAST(t.id AS TEXT), ',') as tags
        FROM blogs b
        LEFT JOIN blog_tags bt ON b.id = bt.blog_id
        LEFT JOIN tags t ON bt.tag_id = t.id
        GROUP BY b.id
        ORDER BY b.id ASC
        "#
    )
    .fetch_all(&app_state.pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let blogs_with_parsed_tags: Vec<BlogWithParsedTags> = blogs
        .into_iter()
        .map(|blog| {
            let parsed_tags = blog
                .tags
                .map(|tags_str| {
                    tags_str
                        .split(',')
                        .filter_map(|tag_id| tag_id.parse().ok())
                        .collect()
                })
                .unwrap_or_default();

            BlogWithParsedTags {
                id: blog.id,
                title: blog.title,
                content: blog.content,
                author: blog.author,
                image_url: blog.image_url,
                tags: parsed_tags,
            }
        })
        .collect();

    HttpResponse::Ok().json(blogs_with_parsed_tags)
}

pub async fn get_blog(
    path: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let blog_id: i32 = path.into_inner();

    let blog = sqlx::query_as!(
        Blog,
        r#"
        SELECT b.id, b.title, b.content, b.author, b.image_url,
               STRING_AGG(CAST(t.id AS TEXT), ',') as tags
        FROM blogs b
        LEFT JOIN blog_tags bt ON b.id = bt.blog_id
        LEFT JOIN tags t ON bt.tag_id = t.id
        WHERE b.id = $1
        GROUP BY b.id
        "#,
        blog_id
    )
    .fetch_optional(&app_state.pool)
    .await;

    match blog {
        Ok(Some(blog)) => {
            let parsed_tags = blog
                .tags
                .map(|tags_str| {
                    tags_str
                        .split(',')
                        .filter_map(|tag_id| tag_id.parse().ok())
                        .collect()
                })
                .unwrap_or_default();

            let blog_with_parsed_tags = BlogWithParsedTags {
                id: blog.id,
                title: blog.title,
                content: blog.content,
                author: blog.author,
                image_url: blog.image_url,
                tags: parsed_tags,
            };

            Ok(HttpResponse::Ok().json(blog_with_parsed_tags))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(BlogResponse {
            blog: BlogWithParsedTags {
                id: 0,
                title: String::new(),
                content: String::new(),
                author: String::new(),
                image_url: None,
                tags: vec![],
            },
            message: "Blog not found".to_string(),
        })),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(BlogResponse {
                blog: BlogWithParsedTags {
                    id: 0,
                    title: String::new(),
                    content: String::new(),
                    author: String::new(),
                    image_url: None,
                    tags: vec![],
                },
                message: "An error occurred while retrieving the blog".to_string(),
            }))
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ MUTATIONS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn get_all_tags(app_state: web::Data<AppState>) -> HttpResponse {
    let tags = sqlx::query_as!(Tag, "SELECT * FROM tags")
        .fetch_all(&app_state.pool)
        .await;

    match tags {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => {
            eprintln!("Failed to retrieve tags: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to retrieve tags".to_string(),
            })
        }
    }
}

pub async fn create_blog(
    body: web::Json<CreateBlogBody>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let mut tx = match app_state.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to start transaction".to_string(),
            });
        }
    };

    let blog_result = sqlx::query_as!(
        Blog,
        r#"
        INSERT INTO blogs (title, content, author, image_url)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, content, author, image_url, NULL as tags
        "#,
        body.title,
        body.content,
        body.author,
        body.image_url
    )
    .fetch_one(&mut *tx)
    .await;

    match blog_result {
        Ok(blog) => {
            let mut inserted_tags = Vec::new();
            if let Some(tags) = &body.tags {
                for tag_id in tags {
                    if let Err(e) = sqlx::query!(
                        "INSERT INTO blog_tags (blog_id, tag_id) VALUES ($1, $2)",
                        blog.id,
                        tag_id
                    )
                    .execute(&mut *tx)
                    .await
                    {
                        let _ = tx.rollback().await;
                        eprintln!("Failed to add tag to blog: {}", e);
                        return HttpResponse::InternalServerError().json(MyResponse {
                            message: format!("Failed to create blog with tags: {}", e),
                        });
                    }
                    inserted_tags.push(*tag_id);
                }
            }

            if let Err(e) = tx.commit().await {
                eprintln!("Failed to commit transaction: {}", e);
                return HttpResponse::InternalServerError().json(MyResponse {
                    message: format!("Failed to commit transaction: {}", e),
                });
            }

            HttpResponse::Ok().json(BlogResponse {
                blog: BlogWithParsedTags {
                    id: blog.id,
                    title: blog.title,
                    content: blog.content,
                    author: blog.author,
                    image_url: blog.image_url,
                    tags: inserted_tags,
                },
                message: format!("Created blog with id: {}", blog.id),
            })
        }
        Err(e) => {
            let _ = tx.rollback().await;
            eprintln!("Failed to create blog: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to create blog: {}", e),
            })
        }
    }
}

pub async fn delete_blog(blog_id: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let result = sqlx::query!("DELETE FROM blogs WHERE id = $1", blog_id.into_inner())
        .execute(&app_state.pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(MyResponse {
                    message: "Blog not found".to_string(),
                })
            } else {
                HttpResponse::Ok().json(MyResponse {
                    message: "✅ Blog deleted successfully".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Failed to delete blog: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to delete blog".to_string(),
            })
        }
    }
}

pub async fn patch_blog(
    path: web::Path<i32>,
    body: web::Json<BodyPatchBlog>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let blog_id = path.into_inner();
    let mut tx = match app_state.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to start transaction".to_string(),
            });
        }
    };

    if body.title.is_none()
        && body.content.is_none()
        && body.author.is_none()
        && body.image_url.is_none()
        && body.tags.is_none()
    {
        return HttpResponse::BadRequest().json(MyResponse {
            message: "No fields to update".to_string(),
        });
    }

    if let Some(title) = &body.title {
        if let Err(e) = sqlx::query!("UPDATE blogs SET title = $1 WHERE id = $2", title, blog_id)
            .execute(&mut *tx)
            .await
        {
            let _ = tx.rollback().await;
            eprintln!("Failed to update blog title: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to update blog title".to_string(),
            });
        }
    }

    if let Some(content) = &body.content {
        if let Err(e) = sqlx::query!(
            "UPDATE blogs SET content = $1 WHERE id = $2",
            content,
            blog_id
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            eprintln!("Failed to update blog content: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to update blog content".to_string(),
            });
        }
    }

    if let Some(author) = &body.author {
        if let Err(e) = sqlx::query!(
            "UPDATE blogs SET author = $1 WHERE id = $2",
            author,
            blog_id
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            eprintln!("Failed to update blog author: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to update blog author".to_string(),
            });
        }
    }

    if let Some(image_url) = &body.image_url {
        if let Err(e) = sqlx::query!(
            "UPDATE blogs SET image_url = $1 WHERE id = $2",
            image_url,
            blog_id
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            eprintln!("Failed to update blog image_url: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to update blog image_url".to_string(),
            });
        }
    }

    if let Some(tags) = &body.tags {
        if let Err(e) = sqlx::query!("DELETE FROM blog_tags WHERE blog_id = $1", blog_id)
            .execute(&mut *tx)
            .await
        {
            let _ = tx.rollback().await;
            eprintln!("Failed to delete existing tags: {}", e);
            return HttpResponse::InternalServerError().json(MyResponse {
                message: "Failed to delete existing tags".to_string(),
            });
        }

        for tag_id in tags {
            if let Err(e) = sqlx::query!(
                "INSERT INTO blog_tags (blog_id, tag_id) VALUES ($1, $2)",
                blog_id,
                tag_id
            )
            .execute(&mut *tx)
            .await
            {
                let _ = tx.rollback().await;
                eprintln!("Failed to insert new tag: {}", e);
                return HttpResponse::InternalServerError().json(MyResponse {
                    message: "Failed to insert new tag".to_string(),
                });
            }
        }
    }

    if let Err(e) = tx.commit().await {
        eprintln!("Failed to commit transaction: {}", e);
        return HttpResponse::InternalServerError().json(MyResponse {
            message: "Failed to commit transaction".to_string(),
        });
    }

    HttpResponse::Ok().json(MyResponse {
        message: "Blog updated successfully".to_string(),
    })
}

pub async fn create_tag(
    body: web::Json<CreateTag>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let result = sqlx::query_as!(
        Tag,
        "INSERT INTO tags (name) VALUES ($1) RETURNING id, name",
        body.name
    )
    .fetch_one(&app_state.pool)
    .await;

    match result {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(e) => {
            eprintln!("Failed to create tag: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to create tag: {}", e),
            })
        }
    }
}

pub async fn delete_tag(path: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let tag_id = path.into_inner();

    let result = sqlx::query!("DELETE FROM tags WHERE id = $1", tag_id)
        .execute(&app_state.pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(MyResponse {
                    message: "Tag not found".to_string(),
                })
            } else {
                HttpResponse::Ok().json(MyResponse {
                    message: "✅ Tag deleted successfully".to_string(),
                })
            }
        }
        Err(e) => {
            eprintln!("Failed to delete tag: {}", e);
            HttpResponse::InternalServerError().json(MyResponse {
                message: format!("Failed to delete tag: {}", e),
            })
        }
    }
}
