use crate::app::components::Container;
use crate::app::issue::PROSE;
use leptos::{either::Either, prelude::*};
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
    let aria_labelledby =
        format!("issue-{}-title", issue.id.clone());
    let aria_labelledby2 = aria_labelledby.clone();

    let time = issue
        .issue_date
        .map(|date| date.to_string())
        .unwrap_or("".to_string());
    view! {
     <article class="relative isolate flex flex-col gap-8 lg:flex-row">
              <div class="relative aspect-video sm:aspect-2/1 lg:w-[300px]">
                <img src={issue.primary_image_url}  alt="" class="absolute inset-0 rounded-xl bg-gray-50  dark:bg-gray-800" />
                // <div class="absolute inset-0 rounded-xl inset-ring inset-ring-gray-900/10 dark:inset-ring-white/10"></div>
              </div>
              <div class="mt-8 lg:mt-0">
                <div class="flex items-center gap-x-4 text-xs">
                  <time datetime={time} class="text-gray-500 dark:text-gray-400">{issue.issue_date.map(|date| date.to_string()).unwrap_or("".to_string())}</time>
                  {issue
                    .youtube_id
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(
                        view! {
                            <a
                                href=format!(
                                    "https://youtube.com/watch?v={}",
                                    issue.youtube_id,
                                )

                                class="z-10 rounded-full px-3 py-1.5 flex items-center gap-x-1 font-medium text-slate-400 hover:text-red-400"
                            >
                                <PlayIcon class="h-2.5 w-2.5 fill-current"/>
                                <span aria-hidden="true">Watch</span>
                            </a>
                            <span
                                aria-hidden="true"
                                class="text-white"
                            >
                                /
                            </span>
                        },
                    )}
                  <a href="#" class="relative z-10 rounded-full px-3 py-1.5 flex items-center gap-x-1 font-medium border border-sky-400 hover:border-sky-500 active:border-sky-600 hover:text-white text-slate-400">Read</a>
                </div>
                <div class="group relative max-w-xl">
                  <h3 class="mt-3 text-lg/6 font-semibold text-gray-900 group-hover:text-gray-600 dark:text-white dark:group-hover:text-gray-300">
                    <a href=format!("/issue/{}", issue.slug)>
                      <span class="absolute inset-0"></span>
                      {issue.display_name}
                    </a>
                  </h3>
                  <div class=format!("mt-5 text-sm/6 text-slate-400 {}", PROSE) inner_html={issue.description}></div>
                </div>
              </div>
            </article>
    }
}

#[component]
fn IssueEntry_old(issue: IssueShort) -> impl IntoView {
    let aria_labelledby =
        format!("issue-{}-title", issue.id.clone());
    let aria_labelledby2 = aria_labelledby.clone();

    view! {
        <div class="bg-slate-950">
            <div aria-hidden="true" class="relative">
                <img src={issue.primary_image_url} alt="" class="h-96 w-full object-cover" />
                <div class="absolute inset-0 bg-linear-to-t from-slate-950"></div>
            </div>

            <div class="relative mx-auto -mt-12 max-w-7xl px-4 pb-16 sm:px-6 sm:pb-24 lg:px-8">
                <div class="mx-auto max-w-2xl text-center lg:max-w-4xl">
                <div>


                    <p class="font-mono text-sm leading-7 text-white">
                        {issue.issue_date.map(|date| date.to_string()).unwrap_or("".to_string())}
                    </p>
                    </div>
                    <a href=format!("/issue/{}", issue.slug)>
                    <h2 class="text-3xl font-bold tracking-tight text-slate-50 sm:text-4xl">{issue.display_name.clone()}</h2>
                    </a>
                </div>
                <div
                    class=format!("mt-4 text-base leading-7 text-white {} max-w-7xl", PROSE)
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
                                        href=format!(
                                            "https://youtube.com/watch?v={}",
                                            issue.youtube_id,
                                        )

                                        class="flex items-center gap-x-3 text-sm font-bold leading-6 text-red-400 hover:text-red-700 active:text-red-900"
                                    >
                                        <PlayIcon class="h-2.5 w-2.5 fill-current"/>
                                        <span aria-hidden="true">Watch</span>
                                    </a>
                                    <span
                                        aria-hidden="true"
                                        class="text-sm font-bold text-white"
                                    >
                                        /
                                    </span>
                                },
                            )}
                  <a
                            href=format!("/issue/{}", issue.slug)
                            class="flex items-center text-sm font-bold leading-6 text-sky-400 hover:text-sky-700 active:text-sky-900"
                            aria-label=format!("full issue for {}", issue.display_name)
                        >
                            Read issue...
                        </a>
                </div>

                <dl class="mx-auto mt-16 grid max-w-2xl grid-cols-1 gap-x-6 gap-y-10 sm:grid-cols-2 sm:gap-y-16 lg:max-w-none lg:grid-cols-3 lg:gap-x-8">
                <div class="border-t border-slate-800 pt-4 flex">
                    <img class="object-cover" src="https://res.cloudinary.com/dilgcuzda/image/upload/v1768285977/thisweekinbevy/01KEV0XDXE14Z7PVAD00T8KQHW.avif"/>
                </div>
                <div class="border-t border-slate-800 pt-4 flex">
                    <img class="object-cover" src="http://res.cloudinary.com/dilgcuzda/image/upload/v1768286298/thisweekinbevy/01KEV177E2SZRFZBNQ12X49G41.avif"/>
                </div>
                <div class="border-t border-slate-800 pt-4 flex">
                    <img class="object-cover" src="http://res.cloudinary.com/dilgcuzda/image/upload/v1768286595/thisweekinbevy/01KEV1G9AR0D7RWEJHKKTFRDPY.avif"/>
                </div>
                </dl>
            </div>
        </div>
    }
}

#[component]
fn IssueEntry2(issue: IssueShort) -> impl IntoView {
    let aria_labelledby =
        format!("issue-{}-title", issue.id.clone());
    let aria_labelledby2 = aria_labelledby.clone();
    view! {
        <article aria-labelledby=aria_labelledby2 class="py-10 sm:py-12 bg-sky-950">
            <Container>
                <div class="flex flex-col items-start">
                    <h2
                        id=aria_labelledby.clone()
                        class="mt-2 text-lg font-bold text-white"
                    >
                        <a href=format!("/issue/{}", issue.slug)>{issue.display_name.clone()}</a>
                    </h2>
                    <p class="order-first font-mono text-sm leading-7 text-white">
                        {issue.issue_date.map(|date| date.to_string()).unwrap_or("".to_string())}
                    </p>
                    <div
                        class=format!("mt-1 text-base leading-7 text-white {}", PROSE)
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
                                        href=format!(
                                            "https://youtube.com/watch?v={}",
                                            issue.youtube_id,
                                        )

                                        class="flex items-center gap-x-3 text-sm font-bold leading-6 text-sky-400 hover:text-sky-700 active:text-sky-900"
                                    >
                                        <PlayIcon class="h-2.5 w-2.5 fill-current"/>
                                        <span aria-hidden="true">Watch</span>
                                    </a>
                                    <span
                                        aria-hidden="true"
                                        class="text-sm font-bold text-white"
                                    >
                                        /
                                    </span>
                                },
                            )}
                        <a
                            href=format!("/issue/{}", issue.slug)
                            class="flex items-center text-sm font-bold leading-6 text-sky-400 hover:text-sky-700 active:text-sky-900"
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
    let issues = Resource::new_blocking(
        move || {},
        |_| fetch_issues(),
    );

    view! {
        <div class="pb-12 sm:pb-4">
            <Title text="This Week in the Bevy Game Engine"/>
            <Meta
                name="description"
                content="What happened this week in the Bevy Game Engine ecosystem"
            />

            <Link rel="canonical" href="https://thisweekinbevy.com"/>
            <Meta property="og:type" content="website"/>
            <Meta property="og:url" content="https://thisweekinbevy.com"/>
            <Meta
                property="og:image"
                content="https://res.cloudinary.com/dilgcuzda/image/upload/v1708310121/thisweekinbevy/this-week-in-bevyopengraph-light_zwqzqz.png"
            />
            <Meta name="twitter:card" content="summary_large_image"/>
            <Meta name="twitter:creator" content="@chrisbiscardi"/>
            <Meta name="twitter:title" content="This Week in the Bevy Game Engine"/>
            <Meta
                name="twitter:description"
                content="What happened this week in the Bevy Game Engine ecosystem"
            />

            <Meta
                name="twitter:image"
                content="https://res.cloudinary.com/dilgcuzda/image/upload/v1708310121/thisweekinbevy/this-week-in-bevyopengraph-light_zwqzqz.png"
            />

            <div class="pt-16 lg:pt-12 pb-4 lg:pb-8 bg-sky-700">
                <Container>
                      // <h1 class="text-4xl font-semibold tracking-tight text-pretty text-slate-900 sm:text-5xl dark:text-white">Issues</h1>
                      // <p class="mt-2 text-lg/8 text-slate-600 dark:text-slate-200">Something about issues here</p>
              <h1 class="text-2xl/7 font-bold text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight dark:text-white">Issues</h1>
              <p class="mt-2 max-w-2xl text-slate-200">"This Week in Bevy issues something something"</p>
                </Container>
            </div>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    issues
                        .get()
                        .map(|data| match data {
                            Err(_e) => Either::Left(view! { <div></div> }),
                            Ok(issues) => {
                                Either::Right(view! {
                                    // <div class="divide-y-4 divide-slate-950 lg:border-y-4 lg:border-slate-950">
                                    <div class="bg-white py-24 sm:py-32 dark:bg-slate-900">
                                    <div class="mx-auto max-w-7xl px-6 lg:px-8">
                                    <div class="mx-auto max-w-2xl lg:max-w-4xl">
                                        // <h2 class="text-4xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-5xl dark:text-white">Issues</h2>
                                        // <p class="mt-2 text-lg/8 text-gray-600 dark:text-gray-400"></p>
                                        <div class="mt-4 space-y-20 lg:mt-20">
                                        {issues
                                            .into_iter()
                                            .map(|issue| view! { <IssueEntry issue=issue/> })
                                            .collect::<Vec<_>>()}
                                            </div>
                                            </div>
                                            </div>
                                    </div>
                                })
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
    pub cloudinary_public_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueShort {
    pub id: String,
    pub slug: String,
    pub issue_date: Option<time::Date>,
    pub display_name: String,
    pub description: String,
    pub youtube_id: String,
    pub primary_image_url: String,
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
        use cloudinary::transformation::{
            resize_mode::ResizeMode::ScaleByWidth,
            Image as CImage, Transformations::Resize,
        };
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        let summary = markdown_trim(&value.description)
            .map(|(_, output)| output)
            .unwrap_or(&value.description);

        let image = CImage::new(
            "dilgcuzda".into(),
            value.cloudinary_public_id.into(),
        )
        .add_transformation(Resize(ScaleByWidth {
            width: 1080,
            ar: None,
            liquid: None,
        }));

        IssueShort {
            id: id_str.to_string(),
            slug: value.slug,
            issue_date: value.issue_date,
            display_name: value.display_name,
            description: compile(summary),
            youtube_id: value.youtube_id,
            primary_image_url: image.to_string(),
        }
    }
}

#[server]
pub async fn fetch_issues(
) -> Result<Vec<IssueShort>, ServerFnError> {
    let pool = crate::sql::pool()?;

    let issues: Vec<SqlIssueShort> =
        match crate::sql::with_admin_access() {
            Ok(_) => {
                // logged in as admin, serve all issues
                sqlx::query_as!(
                    SqlIssueShort,
                    "
SELECT
    id,
    slug,
    issue_date,
    display_name,
    description,
    youtube_id,
    cloudinary_public_id
FROM issue
ORDER BY status, issue_date DESC"
                )
                .fetch_all(&pool)
                .await?
            }
            Err(_) => {
                // not logged in, serve issues that
                sqlx::query_as!(
                    SqlIssueShort,
                    r#"
SELECT
    id,
    slug,
    issue_date,
    display_name,
    description,
    youtube_id,
    cloudinary_public_id
FROM issue
WHERE status = "publish"
ORDER BY status, issue_date DESC"#
                )
                .fetch_all(&pool)
                .await?
            }
        };

    Ok(issues.into_iter().map(IssueShort::from).collect())
}
