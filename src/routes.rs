use crate::MiniJinjaRenderer;
use actix_files as fs;
use actix_web::{web, Responder};

#[actix_web::get("/")]
async fn base(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("intro.html", minijinja::Value::UNDEFINED)
}

#[actix_web::get("/home")]
async fn home(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("home.html", minijinja::Value::UNDEFINED)
}

#[actix_web::get("/createcontest")]
async fn create_contest(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("createcontest.html", minijinja::Value::UNDEFINED)
}

#[actix_web::get("/login")]
async fn login(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    tmpl_env.render("login.html", minijinja::Value::UNDEFINED)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(base)
        .service(home)
        .service(create_contest)
        .service(login)
        .service(fs::Files::new("/static", "./static/"));
}
