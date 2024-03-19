use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
};
use atom_syndication::*;
use crate::state::AppState;

pub async fn atom_feed(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    use std::fs::File;
use std::io::{BufReader, sink};
use atom_syndication::Feed;

let issues: Vec<SqlIssueShort> = sqlx::query_as!(
    SqlIssueShort,
    r#"
SELECT
id,
slug,
issue_date,
display_name,
description,
youtube_id
FROM issue
WHERE status = "publish"
ORDER BY status, issue_date DESC
LIMIT 5"#
)
.fetch_all(&app_state.pool)
.await.unwrap();

let issues: Vec<IssueShort> = issues.into_iter().map(IssueShort::from).collect();

let mut newest_issue_date = None;

let entries: Vec<Entry> = issues.into_iter().filter_map(|issue| {
let date = issue.issue_date.and_then(|v| {
    use atom_syndication::FixedDateTime;
    use std::str::FromStr;
    FixedDateTime::from_str(&format!("{v}T12:30:00Z")).ok()
})?;

if Some(date) > newest_issue_date {
    newest_issue_date = Some(date);
};

Some(    EntryBuilder::default()
    .title(issue.display_name.clone())
    .id(format!("https://thisweekinbevy.com/issue/{}", issue.slug))
    .updated(date.clone())
    .author(
        Person {
        name: "Chris Biscardi".to_string(),
        email: None,
        uri: Some("https://www.christopherbiscardi.com/".to_string())
      })
      .link( Link {
        href: format!("https://thisweekinbevy.com/issue/{}", issue.slug),
        rel: "alternate".to_string(),
        hreflang: Some("en".to_string()),
        mime_type: Some("text/html".to_string()),
        title: Some(issue.display_name),
        length: None,
    })
    .published(date)
    .summary(Some(Text{
        value: issue.description,
    base: None,
    lang: None,
    r#type: TextType::Html,

}))
    .content(Some(Content {
        base: None,
        lang: Some("en".to_string()),
        value: None,
        src: Some(format!("https://thisweekinbevy.com/issue/{}", issue.slug)),
        content_type: Some("text/html".to_string()),
    })).build())
}).collect();
let mut feed = Feed::default();

feed.set_id("https://thisweekinbevy.com/".to_string());
feed.set_updated(newest_issue_date.expect("having an issue should mean there is a most recent date"));
feed.set_title("This Week in Bevy");
feed
  .set_logo("https://res.cloudinary.com/dilgcuzda/image/upload/v1708481576/thisweekinbevy/this-week-in-bevylight_uddwes.avif".to_string());
  feed
  .set_icon("https://cdn.thisweekinbevy.com/favicon-32x32.png".to_string());
  feed
  .set_authors(vec![
    Person {
    name: "Chris Biscardi".to_string(),
    email: None,
    uri: Some("https://www.christopherbiscardi.com/".to_string())
  }]);
  feed
  .set_links(vec![
    Link {
        href: "https://thisweekinbevy.com".to_string(),
        rel: "alternate".to_string(),
        hreflang: Some("en".to_string()),
        mime_type: Some("text/html".to_string()),
        title: Some("This Week in Bevy".to_string()),
        length: None,
    }
  ]);
  feed
  .set_subtitle(Text::from("What happened this week in the Bevy Engine ecosystem"));
  feed
  .set_entries(entries);
feed.to_string()

}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlIssueShort {
    pub id: Vec<u8>,
    pub slug: String,
    pub issue_date: Option<time::Date>,
    pub display_name: String,
    pub description: String,
    pub youtube_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueShort {
    pub id: String,
    pub slug: String,
    pub issue_date: Option<time::Date>,
    pub display_name: String,
    pub description: String,
    pub youtube_id: String,
}

#[cfg(feature = "ssr")]
fn markdown_trim(input: &str) -> nom::IResult<&str, &str> {
    use nom::{
        character::complete::{anychar, line_ending},
        combinator::consumed,
        multi::many_till,
        sequence::{pair, tuple},
    };
    let (input, (intro, _)) = consumed(tuple((
        many_till(anychar, pair(line_ending, line_ending)),
        many_till(anychar, pair(line_ending, line_ending)),
        many_till(anychar, pair(line_ending, line_ending)),
    )))(input)?;
    Ok((input, intro))
}

#[cfg(feature = "ssr")]
impl From<SqlIssueShort> for IssueShort {
    fn from(value: SqlIssueShort) -> Self {
        use crate::markdown::compile;
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        IssueShort {
            id: id_str.to_string(),
            slug: value.slug,
            issue_date: value.issue_date,
            display_name: value.display_name,
            description: compile(&value.description),
            youtube_id: value.youtube_id,
        }
    }
}

