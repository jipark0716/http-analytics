use std::sync::Arc;

pub struct AppStatus {
    session_service: dyn service_collect::session::SessionService,
}

impl AppStatus {
    fn new() -> Arc<Self> {
        let click_house_db_context = repository_click_house::context::DbContext::new();

        Arc::new(Self {
            session_service: service_collect::session::SessionServiceImpl::new(
                click_house_db_context,
            ),
        })
    }
}
