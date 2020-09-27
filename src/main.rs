use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CalendarEvent {
  title: String,
  start: DateTime<Utc>,
  end: DateTime<Utc>,
  notes: String,
  #[serde(rename = "cal")]
  calendar_name: String,
}

#[derive(Default)]
struct AppData {
  events: Mutex<Vec<CalendarEvent>>,
}

#[get("/")]
async fn hello(data: web::Data<AppData>) -> impl Responder {
  HttpResponse::Ok().body(format!(
    "calbox: {} events",
    data.events.lock().unwrap().len()
  ))
}

#[post("/update")]
async fn update(data: web::Data<AppData>, payload: web::Json<CalendarEvent>) -> impl Responder {
  data.events.lock().unwrap().push(payload.into_inner());

  HttpResponse::Ok()
}

#[post("/clean")]
async fn clean(data: web::Data<AppData>) -> impl Responder {
  data.events.lock().unwrap().clear();

  HttpResponse::Ok()
}

#[get("/json")]
async fn get_json(data: web::Data<AppData>) -> impl Responder {
  let mut events = data.events.lock().unwrap();
  events.retain(|e| e.end >= Utc::now());

  web::Json(serde_json::to_value::<&Vec<CalendarEvent>>(events.as_ref()).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("calbox on port 5010");

  let data = web::Data::new(AppData::default());
  HttpServer::new(move || {
    App::new()
      .app_data(data.clone())
      .service(hello)
      .service(update)
      .service(clean)
      .service(get_json)
  })
  .bind(("0.0.0.0", 5010))?
  .run()
  .await
}
