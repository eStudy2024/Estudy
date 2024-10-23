use crate::MiniJinjaRenderer;
use actix_files as fs;
use actix_web::{web, Responder};

#[actix_web::get("/")]
async fn index(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("intro.html", minijinja::Value::UNDEFINED)
}

#[actix_web::get("/home")]
async fn base(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("home.html", minijinja::Value::UNDEFINED)
}

#[actix_web::get("/login")]
async fn login(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("login.html", minijinja::Value::UNDEFINED)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
       .service(login)
       .service(base)
       .service(fs::Files::new("/static", "./static/"));
}
