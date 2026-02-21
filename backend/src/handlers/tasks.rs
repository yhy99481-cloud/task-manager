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

    // Build query
    let mut sql = "SELECT * FROM tasks WHERE user_id = ?".to_string();
    let mut count_sql = "SELECT COUNT(*) as count FROM tasks WHERE user_id = ?".to_string();

    let mut params: Vec<String> = vec![user_id.clone()];

    // Filter by status
    if let Some(status) = &query.status {
        sql.push_str(" AND status = ?");
        count_sql.push_str(" AND status = ?");
        params.push(format!("{:?}", status).to_lowercase());
    }

    // Search by title
    if let Some(search) = &query.search {
        sql.push_str(" AND title LIKE ?");
        count_sql.push_str(" AND title LIKE ?");
        params.push(format!("%{}%", search));
    }

    // Add ordering and pagination
    sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    // Execute count query
    let count_query = format!("{} AND user_id = ?", count_sql.trim_end_matches(&format!(" AND user_id = ?")));
    let mut count_query_builder = sqlx::query_as::<_, (i64,)>(&count_query);
    count_query_builder = count_query_builder.bind(&user_id);

    if let Some(status) = &query.status {
        count_query_builder = count_query_builder.bind(format!("{:?}", status).to_lowercase());
    }
    if let Some(search) = &query.search {
        count_query_builder = count_query_builder.bind(format!("%{}%", search));
    }

    let (total,) = count_query_builder
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Execute tasks query
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
