use actix_web::web;

pub mod auth;

pub fn set_services(conf: &mut web::ServiceConfig) {
    conf.service(auth::register);
    conf.service(auth::login);
}
