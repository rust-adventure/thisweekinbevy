use crate::app::components::{Container, Divider};
use itertools::Itertools;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::types::Json;
use std::ops::Not;

pub const PROSE: &str = r#"prose text-ctp-text prose-code:text-ctp-text prose-a:text-ctp-sky hover:prose-a:text-ctp-blue prose-blockquote:text-ctp-text [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-ctp-text [&>ul]:mt-6 [&>ul]:list-['⮡\20'] [&>ul]:pl-5"#;

#[derive(Clone, Serialize, Deserialize)]
pub struct Issue {
    /// The title of the issue is technically the
    /// date in yyyy-mm-dd format, but this field
    /// is a human-readable string that goes in
    /// the email subject line and slug url
    title: String,
    slug: String,
    opengraph_image: String,
    header_image: String,
    youtube_id: String,
    issue_date: time::Date,
    /// What is this issue about? Is there
    /// anything notable worth mentioning or
    /// making sure people are aware of?
    description: String,
    /// Showcase what people are doing with Bevy
    /// this week. Often these are in-progress
    /// games, experimental rendering approaches,
    /// or other in-progress bevy work
    showcases: Vec<Showcase>,
    /// Crates that have been released in the last
    /// week. This includes updates and new
    /// releases
    crate_releases: Vec<CrateRelease>,
    /// merged pull requests
    /// Meant to convey what is being added to
    /// Bevy
    merged_pull_requests: Vec<MergedPullRequest>,
    /// educational resources published this week.
    /// videos and blog posts
    educationals: Vec<Educational>,
    /// devlogs published this week.
    /// videos and blog posts
    devlogs: Vec<Devlog>,
    /// newsletter contributors
    /// (not a list of people that contribute to
    /// the bevy repo itself, that exists in the
    /// Bevy release announcements already)
    contributors: Vec<Contributor>,
    /// Want to contribute? check out these
    /// pull requests that need review
    new_pull_requests: Vec<NewPullRequest>,
    /// Want to contribute? check out these
    ///  issues that just got opened
    new_github_issues: Vec<NewIssue>,
}

/// `NewPullRequest` is calculated just before
/// publishing each issue. It is a stateful
/// representation of new pull requests that were
/// opened this week (and still open when the
/// issue is published), with the intent of
/// providing This Week in Bevy readers the
/// opportunity to discover pull requests that
/// could benefit from review this week.
#[derive(Clone, Serialize, Deserialize)]
struct NewPullRequest {
    title: String,
    url: String,
    gh_created_at: String,
    author: String,
    author_url: String,
}

/// `NewIssue` is calculated just before
/// publishing each issue. It is a stateful
/// representation of new issues that were
/// opened this week (and still open when the
/// issue is published), with the intent of
/// providing This Week in Bevy readers the
/// opportunity to discover issues that
/// could benefit from reproduction cases and
/// other fixes.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewIssue {
    title: String,
    url: String,
    github_created_at: String,
    author: String,
    author_url: String,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlNewGhIssue {
    title: String,
    url: String,
    gh_created_at: String,
    author: String,
    author_url: String,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlNewPr {
    title: String,
    url: String,
    gh_created_at: String,
    author: String,
    author_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct CrateRelease {
    title: String,
    url: String,
    discord_url: String,
    description: String,
    posted_date: Option<String>,
    images: Vec<ImgDataTransformed>,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlCrateRelease {
    title: String,
    url: String,
    discord_url: String,
    description: String,
    posted_date: Option<String>,
    images: Option<Vec<ImgData>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Devlog {
    title: String,
    post_url: String,
    video_url: String,
    discord_url: String,
    description: String,
    images: Vec<ImgDataTransformed>,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlDevlog {
    title: String,
    post_url: String,
    video_url: String,
    discord_url: String,
    description: String,
    images: Option<Vec<ImgData>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Educational {
    title: String,
    post_url: String,
    video_url: String,
    discord_url: String,
    description: String,
    images: Vec<ImgDataTransformed>,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlEducational {
    title: String,
    post_url: String,
    video_url: String,
    discord_url: String,
    description: String,
    images: Option<Vec<ImgData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Showcase {
    title: String,
    url: String,
    discord_url: String,
    description: String,
    images: Vec<ImgDataTransformed>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MergedPullRequest {
    title: String,
    url: String,
    merged_at_date: String,
    author: String,
    author_url: String,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlMergedPullRequest {
    title: String,
    url: String,
    merged_at_date: String,
    author: String,
    author_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Contributor;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct SqlIssue {
    issue_date: time::Date,
    slug: String,
    cloudinary_public_id: String,
    display_name: String,
    description: String,
    youtube_id: String,
    showcases: Option<sqlx::types::Json<Vec<ShowcaseData>>>,
    crate_releases:
        Option<sqlx::types::Json<Vec<SqlCrateRelease>>>,
    devlogs: Option<sqlx::types::Json<Vec<SqlDevlog>>>,
    educationals:
        Option<sqlx::types::Json<Vec<SqlEducational>>>,
    new_github_issues:
        Option<sqlx::types::Json<Vec<SqlNewGhIssue>>>,
    new_pull_requests:
        Option<sqlx::types::Json<Vec<SqlNewPr>>>,
    merged_pull_requests: Option<
        sqlx::types::Json<Vec<SqlMergedPullRequest>>,
    >,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct ShowcaseData {
    title: String,
    url: String,
    discord_url: String,
    description: String,
    images: Option<Vec<ImgData>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ImgData {
    id: String,
    description: String,
    cloudinary_public_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImgDataTransformed {
    id: String,
    description: String,
    url: String,
}

#[server]
async fn fetch_issue(
    date: time::Date,
) -> Result<Option<Issue>, leptos::ServerFnError> {
    use crate::markdown::compile;
    use cloudinary::transformation::{
        resize_mode::ResizeMode::ScaleByWidth,
        Image as CImage, Transformations::Resize,
    };
    use data_encoding::BASE64;

    let pool = crate::sql::pool()?;

    let showcase_issue = sqlx::query_file_as!(
        SqlIssue,
        "src/app/routes/issue__showcase.sql",
        date
    )
    .fetch_optional(&pool)
    .await
    // .inspect(|v| {tracing::info!(?v);})
    .inspect_err(|e| {
        tracing::error!(?e);
    })?;

    Ok(showcase_issue.map(|issue| {
        
        let showcases = issue.showcases
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|showcase_data_2| {
    Showcase {
        title: showcase_data_2.title,
        url: showcase_data_2.url,
        discord_url: showcase_data_2.discord_url,
        description: compile(&showcase_data_2.description),
        images: showcase_data_2.images.unwrap_or_default().into_iter()
        .map(|img_data| {
 
            
            let base_id = BASE64.decode(img_data.id.as_bytes()).expect("a valid id in base64 format");
            let img_ulid = rusty_ulid::Ulid::try_from(base_id.as_slice())
            .expect(
                "expect valid ids from the database",
            );
            let image = CImage::new("dilgcuzda".into(), img_data.cloudinary_public_id.into())
                .add_transformation(Resize(ScaleByWidth{ width:600, ar: None, liquid:None }));
            ImgDataTransformed {
                id: img_ulid.to_string(),
                description: img_data.description,
                url: image.to_string()
            }
        })
        .collect()
    }
}).collect::<Vec<Showcase>>();

let crate_releases = issue.crate_releases
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|value| {
    CrateRelease {
        title: value.title,
        url: value.url,
        discord_url: value.discord_url,
        description: compile(&value.description),
        posted_date: value.posted_date,
        images: value.images.unwrap_or_default().into_iter()
        .map(|img_data| {
            let base_id = BASE64.decode(img_data.id.as_bytes()).expect("a valid id in base64 format");
            let img_ulid = rusty_ulid::Ulid::try_from(base_id.as_slice())
            .expect(
                "expect valid ids from the database",
            );
            let image = CImage::new("dilgcuzda".into(), img_data.cloudinary_public_id.into())
                .add_transformation(Resize(ScaleByWidth{ width:600, ar: None, liquid:None }));
            ImgDataTransformed {
                id: img_ulid.to_string(),
                description: img_data.description,
                url: image.to_string()
            }
        })
        .collect()
    }
}).collect::<Vec<CrateRelease>>();

let devlogs = issue.devlogs
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|value| {
    Devlog {
        title: value.title,
        post_url: value.post_url,
        video_url: value.video_url,
        discord_url: value.discord_url,
        description: compile(&value.description),
        images: value.images.unwrap_or_default().into_iter()
        .map(|img_data| {
            let base_id = BASE64.decode(img_data.id.as_bytes()).expect("a valid id in base64 format");
            let img_ulid = rusty_ulid::Ulid::try_from(base_id.as_slice())
            .expect(
                "expect valid ids from the database",
            );
            let image = CImage::new("dilgcuzda".into(), img_data.cloudinary_public_id.into())
                .add_transformation(Resize(ScaleByWidth{ width:600, ar: None, liquid:None }));
            ImgDataTransformed {
                id: img_ulid.to_string(),
                description: img_data.description,
                url: image.to_string()
            }
        })
        .collect()
    }
}).collect::<Vec<Devlog>>();

let educationals = issue.educationals
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|value| {
    Educational {
        title: value.title,
        post_url: value.post_url,
        video_url: value.video_url,
        discord_url: value.discord_url,
        description: compile(&value.description),
        images: value.images.unwrap_or_default().into_iter()
        .map(|img_data| {
            let base_id = BASE64.decode(img_data.id.as_bytes()).expect("a valid id in base64 format");
            let img_ulid = rusty_ulid::Ulid::try_from(base_id.as_slice())
            .expect(
                "expect valid ids from the database",
            );
            let image = CImage::new("dilgcuzda".into(), img_data.cloudinary_public_id.into())
                .add_transformation(Resize(ScaleByWidth{ width:600, ar: None, liquid:None }));
            ImgDataTransformed {
                id: img_ulid.to_string(),
                description: img_data.description,
                url: image.to_string()
            }
        })
        .collect()
    }
}).collect::<Vec<Educational>>();

let new_github_issues = issue.new_github_issues
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|SqlNewGhIssue{
    title,
    url,
    gh_created_at,
    author,
    author_url}| NewIssue{
title,
url,
github_created_at: gh_created_at,
author,
author_url

}).collect();

let new_pull_requests = issue.new_pull_requests
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|SqlNewPr{
    title,
    url,
    gh_created_at,
    author,
    author_url}| NewPullRequest{
title,
url,
gh_created_at,
author,
author_url

}).collect();

let merged_pull_requests = issue.merged_pull_requests
.map(|json| json.0)
.unwrap_or_default()
.into_iter().map(|SqlMergedPullRequest{
    title,
    url,
    merged_at_date,
    author,
    author_url}| MergedPullRequest{
title,
url,
merged_at_date,
author,
author_url

}).collect();

let opengraph_image = CImage::new("dilgcuzda".into(), (*issue.cloudinary_public_id).into());
let header_image = CImage::new("dilgcuzda".into(), issue.cloudinary_public_id.into());

        Issue {
        title: issue.display_name,
        issue_date: issue.issue_date,
        slug: issue.slug,
        youtube_id: issue.youtube_id,
        opengraph_image: opengraph_image.to_string().replace(".avif", ".png"),
        header_image: header_image.to_string(),
        description: compile(&issue.description),
        showcases,
        crate_releases,
        devlogs,
        merged_pull_requests,
        contributors: vec![],
        educationals,
        new_pull_requests,
        new_github_issues,
        }
    }))
}

#[component]
pub fn Issue() -> impl IntoView {
    let params = use_params_map();

    // slug id only cares about the date, which is the
    // unique id. the rest of the slug can be
    // changed any time.
    // 2024-02-11-the-one-before-bevy-0-13
    let issue = create_blocking_resource(
        move || {
            params.with(|p| {
                p.get("slug").cloned().and_then(|slug|
                crate::issue_date::parse_issue_date_from_slug(&slug)
            )
            })
        },
        |date| async move {
            let date = date?;

            fetch_issue(date).await.ok().flatten()
        },
    );
    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || match issue.get() {
                None | Some(None) => view! { <article>"404"</article> },
                Some(Some(issue)) => {
                    view! {
                        <article class="py-16 lg:py-16">
                            <Title text=issue.title.clone()/>
                            <Meta
                                name="description"
                                content=format!(
                                    "What happened in the week of {} the Bevy Game Engine ecosystem",
                                    &issue.issue_date,
                                )
                            />

                            <Meta property="og:type" content="article"/>
                            <Meta
                                property="og:url"
                                content=format!("https://thisweekinbevy.com/issue/{}", issue.slug)
                            />
                            <Link
                                rel="canonical"
                                href=format!("https://thisweekinbevy.com/issue/{}", issue.slug)
                            />
                            <Meta property="og:image" content=issue.opengraph_image.clone()/>
                            <Meta name="twitter:card" content="summary_large_image"/>
                            <Meta name="twitter:creator" content="@chrisbiscardi"/>
                            <Meta name="twitter:title" content=issue.title.clone()/>
                            <Meta
                                name="twitter:description"
                                content=format!(
                                    "What happened in the week of {} the Bevy Game Engine ecosystem",
                                    &issue.issue_date,
                                )
                            />

                            <Meta name="twitter:image" content=issue.opengraph_image/>

                            <Container>
                                <img
                                    loading="lazy"
                                    class="w-full"
                                    loading="lazy"
                                    src=issue.header_image
                                    alt=""
                                />
                                <header class="flex flex-col pt-16">
                                    <div class="flex items-center gap-6">
                                        // <EpisodePlayButton
                                        // episode={episode}
                                        // class="group relative flex h-18 w-18 flex-shrink-0 items-center justify-center rounded-full bg-slate-700 hover:bg-slate-900 focus:outline-none focus:ring focus:ring-slate-700 focus:ring-offset-4"
                                        // playing={
                                        // <PauseIcon class="h-9 w-9 fill-white group-active:fill-white/80" />
                                        // }
                                        // paused={
                                        // <PlayIcon class="h-9 w-9 fill-white group-active:fill-white/80" />
                                        // }
                                        // />
                                        <div class="flex flex-col">
                                            <h1 class="mt-2 text-4xl font-bold text-ctp-text">
                                                {issue.title.clone()}
                                            </h1>
                                            <p class="order-first font-mono text-sm leading-7 text-ctp-text">
                                                {issue.issue_date.to_string()}
                                            </p>
                                        </div>
                                    </div>
                                    // <p class="ml-24 mt-3 text-lg font-medium leading-8 text-ctp-text">
                                    // {issue.as_ref().unwrap().description.clone()}
                                    // </p>
                                    <div
                                        class=format!("mt-3 leading-8 {}", PROSE)
                                        inner_html=issue.description.clone()
                                    ></div>
                                </header>
                                <Divider title="Showcase"/>
                                // <h2 class="mt-2 text-2xl font-bold text-ctp-text">Showcase</h2>
                                {issue
                                    .showcases
                                    .into_iter()
                                    .map(|showcase| {
                                        view! { <ShowcaseView showcase=showcase/> }
                                    })
                                    .collect::<Vec<_>>()}
                                // <h2 class="mt-2 text-2xl font-bold text-ctp-text">Crates</h2>
                                <Divider title="Crates"/>

                                {issue
                                    .crate_releases
                                    .into_iter()
                                    .sorted_by_key(|cr| cr.posted_date.clone())
                                    .rev()
                                    .map(|crate_release| {
                                        view! { <CrateReleaseView crate_release=crate_release/> }
                                    })
                                    .collect_view()}

                                <Divider title="Devlogs"/>
                                // <h2 class="mt-2 text-2xl font-bold text-ctp-text">Devlogs</h2>
                                {issue
                                    .devlogs
                                    .into_iter()
                                    .map(|devlog| {
                                        view! { <DevlogView devlog=devlog/> }
                                    })
                                    .collect_view()}

                                <Divider title="Educational"/>
                                {issue
                                    .educationals
                                    .into_iter()
                                    .map(|educational| {
                                        view! { <EducationalView educational=educational/> }
                                    })
                                    .collect_view()}
                                // <h2 class="mt-2 text-2xl font-bold text-ctp-text">Education</h2>
                                // <Divider title="Fixes and Features"/>
                                // <h2 class="mt-2 text-2xl font-bold text-ctp-text">
                                // Pull Requests Merged this week
                                // </h2>
                                <Divider title="Pull Requests Merged This Week"/>
                                <ul role="list" class="space-y-6 mt-6">

                                    {issue
                                        .merged_pull_requests
                                        .iter()
                                        .sorted_by_key(|pr| &pr.merged_at_date)
                                        .map(|pull_request| {
                                            view! {
                                                <ActivityListItem
                                                    date=&pull_request.merged_at_date
                                                    url=&pull_request.url
                                                    title=&pull_request.title
                                                    author=&pull_request.author
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                                <Divider title="Contributing"/>
                                <CalloutInfo
                                    r#type=CalloutType::Info
                                    title="Want to contribute to Bevy?"
                                >
                                    // link=CalloutLink {
                                    // href: "https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md".to_string(),
                                    // label: "Learn More".to_string()
                                    // }
                                    <p>
                                        Here you can find two types of potential contribution: Pull Requests that might need review and Issues that might need to be worked on.
                                    </p>
                                    <p class="flex justify-end mt-2">
                                        <a href="https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md">
                                            How do I contribute?
                                        </a>
                                    </p>
                                </CalloutInfo>
                                <h2 class="mt-6 text-2xl font-bold text-ctp-text">
                                    Pull Requests Opened this week
                                </h2>
                                <ul role="list" class="space-y-6 mt-6">
                                    {issue
                                        .new_pull_requests
                                        .iter()
                                        .sorted_by_key(|pr| &pr.gh_created_at)
                                        .map(|pull_request| {
                                            view! {
                                                <ActivityListItem
                                                    date=&pull_request.gh_created_at
                                                    url=&pull_request.url
                                                    title=&pull_request.title
                                                    author=&pull_request.author
                                                />
                                            }
                                        })
                                        .collect_view()}

                                </ul>
                                <h2 class="mt-6 text-2xl font-bold text-ctp-text">
                                    Issues Opened this week
                                </h2>
                                <ul role="list" class="space-y-6 mt-6">
                                    {issue
                                        .new_github_issues
                                        .iter()
                                        .sorted_by_key(|issue| &issue.github_created_at)
                                        .map(|issue| {
                                            view! {
                                                <ActivityListItem
                                                    date=issue.github_created_at.clone()
                                                    url=&issue.url
                                                    title=&issue.title
                                                    author=&issue.author
                                                />
                                            }
                                        })
                                        .collect::<Vec<_>>()}

                                </ul>
                            </Container>
                        </article>
                    }
                }
            }}

        </Suspense>
    }
}

#[allow(dead_code)]
enum ActivityListIcon {
    Default,
    Check,
}
#[component]
#[allow(dead_code)]
fn ActivityListItem(
    /// datetime for the <time> element
    #[prop(into)]
    date: String,
    #[prop(into)] url: String,
    #[prop(into)] title: String,
    #[prop(into)] author: String,
    #[prop(default=ActivityListIcon::Default)]
    icon: ActivityListIcon,
) -> impl IntoView {
    view! {
        <li class="relative flex gap-x-4">
            <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                <div class="w-px bg-ctp-overlay1"></div>
            </div>
            <div class="relative flex h-6 w-6 flex-none items-center justify-center bg-ctp-base">

                {match icon {
                    ActivityListIcon::Default => {
                        view! {
                            <>
                                <div class="h-1.5 w-1.5 rounded-full bg-ctp-overlay1 ring-1 ring-ctp-overlay-2"></div>
                            </>
                        }
                    }
                    ActivityListIcon::Check => {
                        view! {
                            <>
                                <svg
                                    class="h-6 w-6 text-green-600"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                    aria-hidden="true"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z"
                                        clip-rule="evenodd"
                                    ></path>
                                </svg>
                            </>
                        }
                    }
                }}

            </div>
            <p class=format!(
                "flex-auto py-0.5 text-xs leading-5 {}",
                if author.starts_with("dependabot") { "text-ctp-overlay1" } else { "text-ctp-text" },
            )>
                <a href=url class="font-medium">
                    {title}
                </a>
                " authored by "
                <span class="font-medium">{author}</span>
            </p>
            <time
                datetime=date.to_string()
                class="flex-none py-0.5 text-xs leading-5 text-ctp-text"
            >
                {date.to_string()}
            </time>
        </li>
    }
}
#[component]
fn ActivityListComment(
    /// datetime for the <time> element
    /// "2023-01-23T15:56"
    #[prop(into)]
    time: String,
    #[prop(into)] url: String,
    #[prop(
        into,
        default = "This is going to be super useful for something in the future!".to_string()
    )]
    comment: String,
    #[prop(into, default = "Someone".to_string())]
    author: String,
) -> impl IntoView {
    view! {
        <li class="relative flex gap-x-4">
            <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                <div class="w-px bg-gray-200"></div>
            </div>
            <img
                src="/this-week-in-bevydark"
                loading="lazy"
                alt=""
                class="relative mt-3 h-6 w-6 flex-none rounded-full bg-gray-50"
            />
            <div class="flex-auto rounded-md p-3 ring-1 ring-inset ring-gray-200">
                <div class="flex justify-between gap-x-4">
                    <div class="py-0.5 text-xs leading-5 text-ctp-text">
                        <span class="font-medium text-ctp-text">{author}</span>
                        commented
                    </div>
                    <time datetime=time class="flex-none py-0.5 text-xs leading-5 text-ctp-text">
                        3d ago
                    </time>
                </div>
                <p class="text-sm leading-6 text-ctp-text">{comment}</p>
            </div>
        </li>
    }
}

#[component]
fn CrateReleaseView(
    crate_release: CrateRelease,
) -> impl IntoView {
    let mut it = crate_release.images.iter();
    let first_image = it.next();

    view! {
        {first_image
            .map(|image| {
                view! {
                    <a href=&crate_release.url>
                        <img
                            class="w-full mt-12 w-full rounded-t-md"
                            loading="lazy"
                            src=&image.url
                            alt=&image.description
                        />
                    </a>
                }
            })}

        <ul
            role="list"
            class="mt-3 grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
        >
            {it
                .map(|image| {
                    view! {
                        <li class="relative">
                            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                                <img
                                    src=&image.url
                                    loading="lazy"
                                    alt=&image.description
                                    class="pointer-events-none object-cover group-hover:opacity-75"
                                />
                                <button type="button" class="absolute inset-0 focus:outline-none">
                                    <span class="sr-only">View details</span>
                                </button>
                            </div>
                        </li>
                    }
                })
                .collect_view()}

        </ul>
        <div class="flex justify-between">
            <h3 class="mt-2 text-xl font-bold text-ctp-text">{crate_release.title}</h3>
            <div class="flex space-x-4">

                {crate_release
                    .url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <URLLink url=crate_release.url/> })}
                {crate_release
                    .discord_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <DiscordLink discord_url=crate_release.discord_url/> })}

            </div>
        </div>
        <div class=format!("mt-3 {}", PROSE) inner_html=crate_release.description></div>
    }
}

#[component]
fn DevlogView(devlog: Devlog) -> impl IntoView {
    let mut it = devlog.images.iter();
    let first_image = it.next();
    view! {
        {first_image
            .map(|image| {
                view! {
                    <a href=&devlog.post_url>
                        <img
                            class="w-full mt-12 w-full rounded-t-md"
                            loading="lazy"
                            src=&image.url
                            alt=&image.description
                        />
                    </a>
                }
            })}

        <ul
            role="list"
            class="mt-3 grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
        >

            {it
                .map(|image| {
                    view! {
                        <li class="relative">
                            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                                <img
                                    src=&image.url
                                    loading="lazy"
                                    alt=&image.description
                                    class="pointer-events-none object-cover group-hover:opacity-75"
                                />
                                <button type="button" class="absolute inset-0 focus:outline-none">
                                    <span class="sr-only">View details</span>
                                </button>
                            </div>
                        </li>
                    }
                })
                .collect_view()}

        </ul>
        <div class="flex justify-between">
            <h3 class="mt-2 text-xl font-bold text-ctp-text">{devlog.title}</h3>
            <div class="flex space-x-4">

                {devlog
                    .post_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <PostLink url=devlog.post_url/> })}
                {devlog
                    .video_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <VideoLink url=devlog.video_url/> })}
                {devlog
                    .discord_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <DiscordLink discord_url=devlog.discord_url/> })}

            </div>
        </div>
        <div class=format!("mt-3 {}", PROSE) inner_html=devlog.description></div>
    }
}

#[component]
fn EducationalView(
    educational: Educational,
) -> impl IntoView {
    let mut it = educational.images.iter();
    let first_image = it.next();
    view! {
        {first_image
            .map(|image| {
                view! {
                    <a href=&educational.post_url>
                        <img
                            class="w-full mt-12 w-full rounded-t-md"
                            loading="lazy"
                            src=&image.url
                            alt=&image.description
                        />
                    </a>
                }
            })}

        <ul
            role="list"
            class="mt-3 grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
        >

            {it
                .map(|image| {
                    view! {
                        <li class="relative">
                            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                                <img
                                    src=&image.url
                                    loading="lazy"
                                    alt=&image.description
                                    class="pointer-events-none object-cover group-hover:opacity-75"
                                />
                                <button type="button" class="absolute inset-0 focus:outline-none">
                                    <span class="sr-only">View details</span>
                                </button>
                            </div>
                        </li>
                    }
                })
                .collect_view()}

        </ul>
        <div class="flex justify-between">
            <h3 class="mt-2 text-xl font-bold text-ctp-text">{educational.title}</h3>
            <div class="flex space-x-4">

                {educational
                    .post_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <PostLink url=educational.post_url/> })}
                {educational
                    .video_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <VideoLink url=educational.video_url/> })}
                {educational
                    .discord_url
                    .trim()
                    .is_empty()
                    .not()
                    .then_some(view! { <DiscordLink discord_url=educational.discord_url/> })}

            </div>
        </div>
        <div class=format!("mt-3 {}", PROSE) inner_html=educational.description></div>
    }
}

#[component]
fn ShowcaseView(showcase: Showcase) -> impl IntoView {
    let mut it = showcase.images.iter();
    let first_image = it.next();
    view! {
        {first_image
            .map(|image| {
                view! {
                    <a href=&showcase.url>
                        <img
                            loading="lazy"
                            class="w-full mt-12"
                            src=&image.url
                            alt=&image.description
                        />
                    </a>
                }
            })}

        <ul
            role="list"
            class="mt-3 grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
        >

            {it
                .map(|image| {
                    view! {
                        <li class="relative">
                            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                                <img
                                    src=&image.url
                                    loading="lazy"
                                    alt=&image.description
                                    class="pointer-events-none object-cover group-hover:opacity-75"
                                />
                                <button type="button" class="absolute inset-0 focus:outline-none">
                                    <span class="sr-only">View details for IMG_4985</span>
                                </button>
                            </div>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}

        </ul>
        <div class="flex justify-between">
            <h3 class="mt-2 text-xl font-bold text-ctp-text">{showcase.title}</h3>

            {showcase
                .discord_url
                .trim()
                .is_empty()
                .not()
                .then_some(view! { <DiscordLink discord_url=showcase.discord_url/> })}

        </div>
        <div class=format!("mt-3, {}", PROSE) inner_html=showcase.description></div>
    }
}

#[component]
fn DiscordLink(discord_url: String) -> impl IntoView {
    view! {
        <a
            href=&discord_url
            class="inline-flex items-center gap-x-1.5 rounded-md bg-brand-discord px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-brand-discord focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brand-discord"
        >
            Discord
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 0 1-2.555-.337A5.972 5.972 0 0 1 5.41 20.97a5.969 5.969 0 0 1-.474-.065 4.48 4.48 0 0 0 .978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25Z"
                ></path>
            </svg>
        </a>
    }
}

#[component]
fn VideoLink(url: String) -> impl IntoView {
    view! {
        <a
            href=&url
            class="inline-flex items-center gap-x-1.5 rounded-md bg-ctp-sky px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ctp-sky"
        >
            Video
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="w-6 h-6"
            >
                <path
                    fill-rule="evenodd"
                    d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm14.024-.983a1.125 1.125 0 0 1 0 1.966l-5.603 3.113A1.125 1.125 0 0 1 9 15.113V8.887c0-.857.921-1.4 1.671-.983l5.603 3.113Z"
                    clip-rule="evenodd"
                ></path>
            </svg>
        </a>
    }
}

#[component]
fn PostLink(url: String) -> impl IntoView {
    view! {
        <a
            href=&url
            class="inline-flex items-center gap-x-1.5 rounded-md bg-ctp-sky px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ctp-sky"
        >
            Post
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25"
                ></path>
            </svg>

        </a>
    }
}

#[component]
fn URLLink(url: String) -> impl IntoView {
    view! {
        <a
            href=&url
            class="inline-flex items-center gap-x-1.5 rounded-md bg-ctp-sky px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ctp-sky"
        >
            URL
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25"
                ></path>
            </svg>

        </a>
    }
}

#[allow(dead_code)]
enum CalloutType {
    Info,
    Caution,
    Warning,
    Success,
}
impl CalloutType {
    fn bg(&self) -> &str {
        match self {
            CalloutType::Info => {
                "bg-blue-50 dark:bg-blue-950"
            }
            CalloutType::Caution => {
                "bg-yellow-50 dark:bg-yellow-950"
            }
            CalloutType::Warning => {
                "bg-red-50 dark:bg-red-950"
            }
            CalloutType::Success => {
                "bg-green-50 dark:bg-green-950"
            }
        }
    }
    fn icon(&self) -> &str {
        match self {
            CalloutType::Info => "text-ctp-blue",
            CalloutType::Caution => "text-ctp-yellow",
            CalloutType::Warning => "text-ctp-red",
            CalloutType::Success => "text-ctp-green",
        }
    }
    fn icon_svg(&self) -> impl IntoView {
        match self {
            CalloutType::Info => view! {
                <svg
                    class=format!("h-5 w-5 {}", self.icon())
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            },
            CalloutType::Caution => view! {
                <svg
                    class=format!("h-5 w-5 {}", self.icon())
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            },
            CalloutType::Warning => view! {
                <svg
                    class=format!("h-5 w-5 {}", self.icon())
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            },
            CalloutType::Success => view! {
                <svg
                    class=format!("h-5 w-5 {}", self.icon())
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            },
        }
    }
    fn title(&self) -> &str {
        match self {
            CalloutType::Info => "text-ctp-blue",
            CalloutType::Caution => "text-ctp-yellow",
            CalloutType::Warning => "text-ctp-red",
            CalloutType::Success => "text-ctp-green",
        }
    }
    fn content(&self) -> &str {
        match self {
            CalloutType::Info => "text-ctp-blue",
            CalloutType::Caution => "text-ctp-yellow",
            CalloutType::Warning => "text-ctp-red",
            CalloutType::Success => "text-ctp-green",
        }
    }
    fn child_links(&self) -> &str {
        match self {
            CalloutType::Info => "[&_a]:font-medium [&_a]:text-blue-700 [&_a]:underline [&_a]:hover:text-blue-600",
            CalloutType::Caution => "[&_a]:font-medium [&_a]:text-yellow-700 [&_a]:underline [&_a]:hover:text-yellow-600",
            CalloutType::Warning => "[&_a]:font-medium [&_a]:text-red-700 [&_a]:underline [&_a]:hover:text-red-600",
            CalloutType::Success => "[&_a]:font-medium [&_a]:text-green-700 [&_a]:underline [&_a]:hover:text-green-600",
        }
    }
}

struct CalloutLink {
    href: String,
    label: String,
}

#[component]
fn CalloutInfo(
    r#type: CalloutType,
    #[prop(into, optional, default = "".to_string())] title: String,
    #[prop(optional, default = None)] link: Option<
        CalloutLink,
    >,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("rounded-md p-4 {}", r#type.bg())>
            <div class="flex">
                <div class="flex-shrink-0">

                    {r#type.icon_svg()}

                </div>
                <div class="ml-3">

                    {title
                        .trim()
                        .is_empty()
                        .not()
                        .then_some(
                            view! {
                                <h3 class=format!(
                                    "text-sm font-medium {}",
                                    r#type.title(),
                                )>{title}</h3>
                            },
                        )}
                    <div class=if link.is_none() {
                        format!("mt-2 text-sm {} {}", r#type.content(), r#type.child_links())
                    } else {
                        format!(
                            "ml-3 flex-1 md:flex md:justify-between {} {}",
                            r#type.content(),
                            r#type.child_links(),
                        )
                    }>

                        {children()}
                        {link
                            .map(|CalloutLink { href, label }| {
                                view! {
                                    <p class="mt-3 text-sm md:ml-6 md:mt-0">
                                        <a
                                            href=href
                                            class="whitespace-nowrap font-medium text-blue-700 hover:text-blue-600"
                                        >
                                            {label}
                                            <span aria-hidden="true">" →"</span>
                                        </a>
                                    </p>
                                }
                            })}

                    </div>
                // <div class="mt-4">
                // <div class="-mx-2 -my-1.5 flex">
                // <button type="button" class="rounded-md bg-green-50 px-2 py-1.5 text-sm font-medium text-green-800 hover:bg-green-100 focus:outline-none focus:ring-2 focus:ring-green-600 focus:ring-offset-2 focus:ring-offset-green-50">View status</button>
                // <button type="button" class="ml-3 rounded-md bg-green-50 px-2 py-1.5 text-sm font-medium text-green-800 hover:bg-green-100 focus:outline-none focus:ring-2 focus:ring-green-600 focus:ring-offset-2 focus:ring-offset-green-50">Dismiss</button>
                // </div>
                // </div>
                </div>
            </div>
        </div>
    }
}
