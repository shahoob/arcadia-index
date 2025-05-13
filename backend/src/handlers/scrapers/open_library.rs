use crate::Result;
use actix_web::{HttpResponse, web};
// Datelike and Timelike are needed in the tests, even though they are not directly referenced
#[allow(unused_imports)]
use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};
use serde::Deserialize;

use crate::models::title_group::{ContentType, UserCreatedTitleGroup, create_default_title_group};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Description {
    Typed {
        #[allow(unused)]
        #[serde(rename = "type")]
        type_: String,
        value: String,
    },
    Untyped(String),
}

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        match self {
            Description::Typed { value, .. } => value,
            Description::Untyped(value) => value,
        }
    }
}

impl From<Description> for String {
    fn from(d: Description) -> Self {
        match d {
            Description::Typed { value, .. } => value,
            Description::Untyped(value) => value,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Work {
    title: String,
    description: Option<Description>,
    first_publish_date: Option<String>,
}

fn parse_date(date: &str) -> Option<DateTime<Local>> {
    date.parse::<i32>()
        .ok()
        .and_then(|y| NaiveDate::from_ymd_opt(y, 1, 1))
        .or_else(|| NaiveDate::parse_from_str(date, "%B %d, %Y").ok())
        .map(|nd| nd.and_hms_opt(0, 0, 0))
        .flatten()
        .map(|ndt| DateTime::<Local>::from_naive_utc_and_offset(ndt, *Local::now().offset()))
}

#[derive(Debug, Deserialize)]
pub struct GetOpenLibraryQuery {
    id: String,
}

pub async fn get_open_library_data(query: web::Query<GetOpenLibraryQuery>) -> Result<HttpResponse> {
    let url = format!("https://openlibrary.org/works/{}.json", query.id);

    let work = reqwest::get(&url).await?.json::<Work>().await?;

    // TODO: kill unwrap and make date nullable
    let original_release_date = work
        .first_publish_date
        .as_ref()
        .and_then(|d| parse_date(d))
        .unwrap();

    let description = work
        .description
        .map(String::from)
        .unwrap_or_else(|| "Description not provided.".into());

    let title_group = UserCreatedTitleGroup {
        name: work.title,
        description,
        external_links: vec![format!("https://openlibrary.org/works/{}", query.id)],
        original_release_date: original_release_date.into(),
        content_type: ContentType::Book,
        ..create_default_title_group()
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({"title_group": title_group})))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization_moby_dick() {
        let moby_dick = serde_json::from_str::<Work>(include_str!("testdata/OL102749W.json"));

        assert!(moby_dick.is_ok());

        let moby_dick = moby_dick.unwrap();

        assert_eq!(moby_dick.title, "Moby Dick");
        assert!(moby_dick.description.is_some());
        assert_eq!(
            moby_dick.description.unwrap().as_ref(),
            "\"Command the murderous chalices! Drink ye harpooners! Drink and swear, ye men that man the deathful whaleboat's bow -- Death to Moby Dick!\" So Captain Ahab binds his crew to fulfil his obsession -- the destruction of the great white whale. Under his lordly but maniacal command the Pequod's commercial mission is perverted to one of vengeance. To Ahab, the monster that destroyed his body is not a creature, but the symbol of \"some unknown but still reasoning thing.\" Uncowed by natural disasters, ill omens, even death, Ahab urges his ship towards \"the undeliverable, nameless perils of the whale.\" Key letters from Melville to Nathaniel Hawthorne are printed at the end of this volume. - Back cover."
        );
        assert!(moby_dick.first_publish_date.is_none());
    }

    #[test]
    fn test_deserialization_les_mis() {
        let les_mis = serde_json::from_str::<Work>(include_str!("testdata/OL1063588W.json"));

        assert!(les_mis.is_ok());

        let les_mis = les_mis.unwrap();

        assert_eq!(les_mis.title, "Les Misérables");
        assert!(les_mis.description.is_some());
        assert_eq!(
            les_mis.description.unwrap().as_ref(),
            "In this story of the trials of the peasant Jean Valjean--a man unjustly imprisoned, baffled by destiny, and hounded by his nemesis, the magnificently realized, ambiguously malevolent police detective Javert--Hugo achieves the sort of rare imaginative resonance that allows a work of art to transcend its genre."
        );
        assert_eq!(les_mis.first_publish_date, Some("1863".into()));
    }

    #[test]
    fn test_parse_date() {
        // OpenLibrary published date is not normalized, try a couple varieties.

        let local_offset_hours = Local::now().offset().local_minus_utc() / 3600;

        let date1 = parse_date("1970").unwrap();
        assert_eq!(
            (
                date1.year(),
                date1.month(),
                date1.day(),
                date1.hour(),
                date1.minute(),
                date1.second(),
            ),
            (1970, 1, 1, local_offset_hours as u32, 0, 0)
        );

        let date2 = parse_date("February 19, 1994").unwrap();
        assert_eq!(
            (
                date2.year(),
                date2.month(),
                date2.day(),
                date2.hour(),
                date2.minute(),
                date2.second(),
            ),
            (1994, 2, 19, local_offset_hours as u32, 0, 0)
        );
    }
}
