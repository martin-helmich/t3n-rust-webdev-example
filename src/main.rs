use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    age: i32,
}

#[derive(Template)]
#[template(path = "users.html")]
struct UserListTemplate<'a> {
    users: &'a [User],
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::NotImplemented()
        .content_type("text/plain")
        .body("Hallo Welt")
}

#[get("/users")]
async fn list_users() -> impl Responder {
    let users = Box::new([User {
        name: "Mustermann".to_string(),
        email: "mustermann@example.com".to_string(),
        age: 99,
    }]);

    HttpResponse::Ok().json(users)
}

#[get("/users.html")]
async fn list_users_html() -> impl Responder {
    let users = Box::new([User {
        name: "Mustermann".to_string(),
        email: "mustermann@example.com".to_string(),
        age: 99,
    }]);

    let tmpl = UserListTemplate { users: &users[..] };
    let tmpl_result = tmpl.render();

    return match tmpl_result {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    };
}

#[post("/users")]
async fn create_user(body: web::Json<User>) -> impl Responder {
    let user = body.into_inner();
    println!("request: {:?}", user);
    HttpResponse::Created().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_factory = || {
        App::new()
            .service(index)
            .service(list_users)
            .service(list_users_html)
            .service(create_user)
    };

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
