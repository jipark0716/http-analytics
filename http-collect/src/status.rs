use std::sync::Arc;

pub struct AppStatus {
    pub session_service: Arc<dyn service_collect::session::SessionService>,
}

impl AppStatus {
    pub fn new() -> Self {
        let click_house_db_context = repository_click_house::context::DbContext::new();
        let session_repository = repository_click_house::session::SessionRepositoryImpl::new(click_house_db_context);

        Self {
            session_service: service_collect::session::SessionServiceImpl::new(
                session_repository.clone(),
            ),
        }
    }
}
