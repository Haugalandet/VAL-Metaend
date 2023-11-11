use actix_web::{get, HttpResponse, Responder, web};
use serde::Deserialize;

use crate::db::{establish_connection, tables::poll::find_polls_by_title};

#[derive(Deserialize)]
struct Data {
    title: String,
}

#[get("/polls/")]
pub async fn get_polls_by_title(data: web::Query<Data>) -> impl Responder {
    if let Ok(client) = establish_connection().await {
        let polls = find_polls_by_title(&client, data.title.clone()).await.unwrap_or_default();

        let json_data = web::Json(polls);

        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .json(json_data);
    }

    HttpResponse::NotFound().insert_header(("Access-Control-Allow-Origin", "*")).body("Title not found")
}