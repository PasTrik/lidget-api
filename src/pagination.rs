use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub next_page: Option<i64>,
    pub total_pages: i64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, limit: i64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as i64;
        let next_page = if page < total_pages { Some(page + 1) } else { None };
        Self {
            data,
            total,
            page,
            limit,
            next_page,
            total_pages,
        }
    }
}