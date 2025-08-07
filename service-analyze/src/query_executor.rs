use std::fmt;
use crate::create_query::{Query, QueryType};
use async_trait::async_trait;
use repository_click_house_read::context;
use std::sync::Arc;

struct QueryError {
    query: Query
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Network error: {}", self.query.query)
    }
}

#[async_trait]
pub trait QueryExecutor: Send + Sync {
    async fn execute(&self, query: Query) -> Result<String, QueryError>;
}

pub struct QueryExecutorFacade {
    list_query_executor: ListQueryExecutor,
}

#[async_trait]
impl QueryExecutor for QueryExecutorFacade {
    async fn execute(&self, query: Query) -> Result<String, QueryError> {
        if query.query_type == QueryType::List {
            return self.list_query_executor.execute(query).await
        }

        Err(QueryError {
            query,
        })
    }
}

impl QueryExecutorFacade {
    pub fn new(db_context: Arc<context::DbContext>) -> Self {
        Self {
            list_query_executor: ListQueryExecutor::new(db_context.clone()),
        }
    }
}

pub struct ListQueryExecutor {
    db_context: Arc<context::DbContext>,
}

impl ListQueryExecutor {
    pub fn new(db_context: Arc<context::DbContext>) -> Self {
        Self { db_context }
    }
}

#[async_trait]
impl QueryExecutor for ListQueryExecutor {
    async fn execute(&self, query: Query) -> Result<String, QueryError> {
        let response = self.db_context
            .query(query.query.clone())
            .await
            .map_err(|e| QueryError { query })?;

        Ok(response)
    }
}
