use actix_web::web;

use super::user_routers;


pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/web")
            .configure(user_routers::users_routes)
    );
}
