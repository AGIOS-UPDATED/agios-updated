use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub total_pages: u32,
    pub total_items: u64,
    pub has_more: bool,
}

impl PaginationParams {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.limit
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.page == 0 {
            return Err("Page must be greater than 0");
        }
        if self.limit == 0 {
            return Err("Limit must be greater than 0");
        }
        if self.limit > 100 {
            return Err("Limit must be less than or equal to 100");
        }
        Ok(())
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, limit: u32, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (limit as f64)).ceil() as u32;
        let has_more = page < total_pages;

        Self {
            data,
            page,
            total_pages,
            total_items,
            has_more,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params() {
        let params = PaginationParams {
            page: 2,
            limit: 10,
        };
        assert_eq!(params.offset(), 10);
    }

    #[test]
    fn test_pagination_validation() {
        let valid_params = PaginationParams {
            page: 1,
            limit: 20,
        };
        assert!(valid_params.validate().is_ok());

        let invalid_page = PaginationParams {
            page: 0,
            limit: 20,
        };
        assert!(invalid_page.validate().is_err());

        let invalid_limit = PaginationParams {
            page: 1,
            limit: 101,
        };
        assert!(invalid_limit.validate().is_err());
    }

    #[test]
    fn test_paginated_response() {
        let data = vec![1, 2, 3];
        let response = PaginatedResponse::new(data, 1, 3, 10);
        
        assert_eq!(response.page, 1);
        assert_eq!(response.total_pages, 4);
        assert_eq!(response.total_items, 10);
        assert!(response.has_more);
    }
}
