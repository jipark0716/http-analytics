use repository_click_house::event::EventRepositoryImpl;
use repository_click_house::session::SessionRepositoryImpl;
use service_collect::event::{CollectService, CollectServiceImpl};
use service_collect::session::{SessionService, SessionServiceImpl};
use std::sync::{Arc};
use config::collect::HttpCollectConfig;

pub struct AppStatus {
    pub session_service: Arc<dyn SessionService>,
    pub collect_service: Arc<dyn CollectService>,
}

impl AppStatus {
    pub fn new(config: &'static HttpCollectConfig<'static>) -> Self {
        let click_house_db_context = repository_click_house::context::DbContext::new(&config.clickhouse);
        let session_repository = SessionRepositoryImpl::new(click_house_db_context.clone());
        let event_repository = EventRepositoryImpl::new(click_house_db_context.clone());

        Self {
            session_service: SessionServiceImpl::new(
                session_repository.clone(),
            ),
            collect_service: CollectServiceImpl::new(
                event_repository.clone(),
            ),
        }
    }
}
