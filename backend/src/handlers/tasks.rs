use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use serde::Deserialize;

use crate::models::{
    CreateTaskRequest, Task, TaskResponse, TaskStatus, UpdateTaskRequest,
    UpdateTaskStatusRequest,
};

// AppState is defined in auth.rs and shared
pub use crate::handlers::auth::AppState;

#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub status: Option<TaskStatus>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn get_tasks(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Query(query): Query<TaskQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;

    // Build query conditions
    let mut conditions = vec!["user_id = ?"];
    let mut params: Vec<String> = vec![user_id.clone()];

    // Filter by status
    if let Some(status) = &query.status {
        conditions.push("status = ?");
        params.push(format!("{:?}", status).to_lowercase());
    }

    // Search by title
    if let Some(search) = &query.search {
        conditions.push("title LIKE ?");
        params.push(format!("%{}%", search));
    }

    let where_clause = conditions.join(" AND ");

    // Execute count query
    let count_sql = format!("SELECT COUNT(*) as count FROM tasks WHERE {}", where_clause);
    let mut count_query_builder = sqlx::query_as::<_, (i64,)>(&count_sql);
    for param in &params {
        count_query_builder = count_query_builder.bind(param);
    }

    let (total,) = count_query_builder
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Count query error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Execute tasks query
    let sql = format!(
        "SELECT * FROM tasks WHERE {} ORDER BY created_at DESC LIMIT {} OFFSET {}",
        where_clause, limit, offset
    );
    let mut tasks_query = sqlx::query_as::<_, Task>(&sql);
    for param in &params {
        tasks_query = tasks_query.bind(param);
    }

    let tasks = tasks_query
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task_responses: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();

    let response = serde_json::json!({
        "tasks": task_responses,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "total_pages": (total as f64 / limit as f64).ceil() as u32,
        }
    });

    Ok(Json(response))
}

pub async fn create_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if payload.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let task = Task::new(user_id, payload.title, payload.description);

    sqlx::query(
        "INSERT INTO tasks (id, user_id, title, description, status, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&task.id)
    .bind(&task.user_id)
    .bind(&task.title)
    .bind(&task.description)
    .bind(format!("{:?}", task.status).to_lowercase())
    .bind(task.created_at)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(TaskResponse::from(task))))
}

pub async fn get_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = task.ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(TaskResponse::from(task)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if task exists and belongs to user
    let existing = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = existing.ok_or(StatusCode::NOT_FOUND)?;

    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.unwrap_or(existing.description);
    let status = payload.status.unwrap_or(existing.status);

    if title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query(
        "UPDATE tasks SET title = ?, description = ?, status = ? WHERE id = ? AND user_id = ?"
    )
    .bind(&title)
    .bind(&description)
    .bind(format!("{:?}", status).to_lowercase())
    .bind(&id)
    .bind(&user_id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch and return updated task
    let updated = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TaskResponse::from(updated)))
}

pub async fn update_task_status(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTaskStatusRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if task exists and belongs to user
    let exists = sqlx::query("SELECT 1 FROM tasks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    exists.ok_or(StatusCode::NOT_FOUND)?;

    sqlx::query("UPDATE tasks SET status = ? WHERE id = ?")
        .bind(format!("{:?}", payload.status).to_lowercase())
        .bind(&id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch and return updated task
    let updated = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TaskResponse::from(updated)))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
