use actix_web::web;

mod auth;
mod bug_report;

pub fn set_services(conf: &mut web::ServiceConfig) {
    conf.service(auth::register);
    conf.service(auth::login);
    conf.service(auth::account_information);
    conf.service(bug_report::bug_report_create);
    conf.service(bug_report::bug_report_all);
    conf.service(bug_report::bug_report_update);
    conf.service(bug_report::bug_report_one);
}
