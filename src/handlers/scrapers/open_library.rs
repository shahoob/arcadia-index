use actix_web::{HttpResponse, web};
use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashMap;

use crate::models::title_group::{ContentType, create_default_title_group};

pub async fn get_open_library_data(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let open_library_id = query.get("id").expect("id not found in query");
    //TODO: check if there is an entry in the db with this open_library_id
    let mut title_group = create_default_title_group();
    title_group.external_links = vec![format!("https://openlibrary.org/works/{}", open_library_id)];
    title_group.content_type = ContentType::Book;

    match reqwest::get(format!(
        "https://openlibrary.org/works/{}.json",
        open_library_id
    ))
    .await
    {
        Ok(response) => match response.json::<Value>().await {
            Ok(json) => {
                title_group.name = json["title"]
                    .as_str()
                    .unwrap_or("Title not found")
                    .to_string();
                title_group.description = json["description"]["value"]
                    .as_str()
                    .unwrap_or("Description not found")
                    .to_string();
                title_group.original_release_date = NaiveDate::from_ymd_opt(
                    json["first_publish_date"]
                        .as_str()
                        .unwrap_or("0000")
                        .parse()
                        .unwrap_or(0),
                    1,
                    1,
                )
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap();
            }
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to parse JSON");
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to fetch data");
        }
    }
    HttpResponse::Ok().json(serde_json::json!({"title_group": title_group}))
}
