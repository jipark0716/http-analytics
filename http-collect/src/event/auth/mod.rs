mod login;
mod logout;
mod password_find_end;
mod password_find_start;
mod pre_login;
mod sign_in_end;
mod sign_in_start;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("auth")
            .service(login::action)
            .service(pre_login::action)
            .service(logout::action)
            .service(sign_in_start::action)
            .service(sign_in_end::action)
            .service(password_find_start::action)
            .service(password_find_end::action),
    );
}
