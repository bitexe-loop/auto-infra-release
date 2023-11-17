use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
struct FormData{
    name: String,
    message: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../static/index.html"))
}


async fn form() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../static/form.html"))
}

async fn handle_form(form: web::Form<FormData>, data: web::Data<Mutex<Vec<FormData>>>) -> impl Responder {
    let mut data = data.lock().unwrap();
    data.push(form.into_inner());
    HttpResponse::SeeOther().append_header(("Location", "/submissions")).finish()
}

async fn submissions(data: web::Data<Mutex<Vec<FormData>>>) -> impl Responder {
    let data = data.lock().unwrap();
    let submissions = data.iter().enumerate().map(|(index, f)| {
        format!("{}: {} <form action='/delete/{}' method='post'><button type='submit'>Delete</button></form>", f.name, f.message, index)
    }).collect::<Vec<String>>().join("<br>");

    let navbar = "
        <header>
            <nav>
                <a href='/'>Home</a> | 
                <a href='/form'>Submit Form</a> | 
                <a href='/submissions'>View Submissions</a>
            </nav>
        </header>
    ";

    let body = format!("{}<div class='submission-container'>{}</div>", navbar, submissions);
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn delete_submission(data: web::Data<Mutex<Vec<FormData>>>, info: web::Path<usize>) -> impl Responder {
    let mut data = data.lock().unwrap();
    let index = info.into_inner(); // Change this line

    if index < data.len() {
        data.remove(index);
    }

    HttpResponse::SeeOther().append_header(("Location", "/submissions")).finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Mutex::new(Vec::<FormData>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/form", web::get().to(form))
            .route("/handle_form", web::post().to(handle_form))
            .route("/submissions", web::get().to(submissions))
            .route("/delete/{id}", web::post().to(delete_submission))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}