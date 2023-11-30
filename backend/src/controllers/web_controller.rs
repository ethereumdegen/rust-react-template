use actix_web::web::ServiceConfig;

pub trait WebController {
    fn config(cfg: &mut ServiceConfig);
}
