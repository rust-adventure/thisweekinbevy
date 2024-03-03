use crate::app::components::Container;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use std::ops::Not;

#[component]
fn PauseIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg aria-hidden="true" viewBox="0 0 10 10" class=class>
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M1.496 0a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5H2.68a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5H1.496Zm5.82 0a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5H8.5a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5H7.316Z"
            ></path>
        </svg>
    }
}

#[component]
fn PlayIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg aria-hidden="true" viewBox="0 0 10 10" class=class>
            <path d="M8.25 4.567a.5.5 0 0 1 0 .866l-7.5 4.33A.5.5 0 0 1 0 9.33V.67A.5.5 0 0 1 .75.237l7.5 4.33Z"></path>
        </svg>
    }
}

#[component]
fn IssueEntry(issue: IssueShort) -> impl IntoView {
    view! {
        <article aria-labelledby=format!("issue-{}-title", issue.id) class="py-10 sm:py-12">
            <Container>
                <div class="flex flex-col items-start">
                    <h2
                        id=format!("issue-{}-title", issue.id)
                        class="mt-2 text-lg font-bold text-ctp-text"
                    >
                        <a href=format!("/issue/{}", issue.slug)>{&issue.display_name}</a>
                    </h2>
                    <p class="order-first font-mono text-sm leading-7 text-ctp-text">
                        {&issue.issue_date.map(|date| date.to_string()).unwrap_or("".to_string())}
                    </p>
                    <div
                        class=r#"mt-1 text-base leading-7 text-ctp-text prose [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#
                        inner_html=issue.description.clone()
                    ></div>
                    <div class="mt-4 flex items-center gap-4">

                        {issue
                            .youtube_id
                            .trim()
                            .is_empty()
                            .not()
                            .then_some(
                                view! {
                                    <a
                                        href=format!("https://youtube.com/watch?v=")
                                        class="flex items-center gap-x-3 text-sm font-bold leading-6 text-ctp-pink hover:text-pink-700 active:text-pink-900"
                                    >
                                        <PlayIcon class="h-2.5 w-2.5 fill-current"/>
                                        <span aria-hidden="true">Watch</span>
                                    </a>
                                    <span
                                        aria-hidden="true"
                                        class="text-sm font-bold text-ctp-text"
                                    >
                                        /
                                    </span>
                                },
                            )}
                        <a
                            href=format!("/issue/{}", issue.slug)
                            class="flex items-center text-sm font-bold leading-6 text-ctp-pink hover:text-pink-700 active:text-pink-900"
                            aria-label=format!("full issue for {}", issue.display_name)
                        >
                            Read issue...
                        </a>
                    </div>
                </div>
            </Container>
        </article>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let issues =
        create_resource(move || {}, |_| fetch_issues());

    view! {
        <div class="pb-12 sm:pb-4">
            <Title text="This Week in the Bevy Game Engine"/>
            <Meta
                name="description"
                content="What happened this week in the Bevy Game Engine ecosystem"
            />

            <Meta property="og:type" content="website"/>
            <Meta property="og:url" content="https://thisweekinbevy.com/"/>
            <Meta
                property="og:image"
                content="https://res.cloudinary.com/dilgcuzda/image/upload/v1708310121/thisweekinbevy/this-week-in-bevyopengraph-light_zwqzqz.avif"
            />

            <div class="pt-16 lg:pt-12 pb-4 lg:pb-8  bg-gradient-to-r from-ctp-mantle to-ctp-base">
                <Container>
                    <h1 class="text-2xl font-bold leading-7 text-ctp-text">Issues</h1>
                </Container>
            </div>
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    issues
                        .get()
                        .map(|data| match data {
                            Err(_e) => view! { <div></div> },
                            Ok(issues) => {
                                view! {
                                    <div class="divide-y-4 divide-ctp-mantle lg:border-y-4 lg:border-ctp-mantle">

                                        {issues
                                            .into_iter()
                                            .map(|issue| view! { <IssueEntry issue=issue/> })
                                            .collect::<Vec<_>>()}

                                    </div>
                                }
                            }
                        })
                }}

            </Suspense>
        </div>
    }
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
        let summary = markdown_trim(&value.description)
            .map(|(_, output)| output)
            .unwrap_or(&value.description);
        IssueShort {
            id: id_str.to_string(),
            slug: value.slug,
            issue_date: value.issue_date,
            display_name: value.display_name,
            description: compile(summary),
            youtube_id: value.youtube_id,
        }
    }
}

#[server]
pub async fn fetch_issues(
) -> Result<Vec<IssueShort>, ServerFnError> {
    let pool = crate::sql::pool()?;

    let issues: Vec<SqlIssueShort> = sqlx::query_as!(
        SqlIssueShort,
        "SELECT
          id,
          slug,
          issue_date,
          display_name,
          description,
          youtube_id
FROM issue
ORDER BY status, issue_date DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(issues.into_iter().map(IssueShort::from).collect())
}
