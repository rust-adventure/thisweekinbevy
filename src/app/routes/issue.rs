use std::ops::Not;

use crate::app::components::{Container, Divider};

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::types::Json;

#[derive(Clone, Serialize, Deserialize)]
pub struct Issue {
    /// The title of the issue is technically the
    /// date in yyyy-mm-dd format, but this field
    /// is a human-readable string that goes in
    /// the email subject line and slug url
    title: String,
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
    educational: Vec<Educational>,
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
    images: Option<Vec<ImgData>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Educational {
    title: String,
    description: String,
    url: String,
    images: Vec<String>,
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
struct SqlShowcaseData {
    slug: String,
    issue_date: time::Date,
    cloudinary_public_id: String,
    display_name: String,
    description: String,
    youtube_id: String,
    showcases:
        Option<sqlx::types::Json<Vec<ShowcaseData2>>>,
    crate_releases:
        Option<sqlx::types::Json<Vec<SqlCrateRelease>>>,
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
struct ShowcaseData2 {
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
        SqlShowcaseData,
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

        Issue {
        title: issue.display_name,
        issue_date: issue.issue_date,
        description: compile(&issue.description),
        showcases,
        crate_releases: crate_releases,
        merged_pull_requests,
        contributors: vec![],
        educational: vec![],
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
    let issue = create_resource(
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
                        <article class="py-16 lg:py-36">
                            <Container>
                                <header class="flex flex-col">
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
                                            <h1 class="mt-2 text-4xl font-bold text-slate-900">
                                                {issue.title.clone()}
                                            </h1>
                                            <p class="order-first font-mono text-sm leading-7 text-slate-500">
                                                {issue.issue_date.to_string()}
                                            </p>
                                        </div>
                                    </div>
                                    // <p class="ml-24 mt-3 text-lg font-medium leading-8 text-slate-700">
                                    // {issue.as_ref().unwrap().description.clone()}
                                    // </p>
                                    <div
                                        class=r#"mt-3 leading-8 text-slate-700 prose [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#
                                        inner_html=issue.description.clone()
                                    ></div>
                                </header>
                                <Divider title="Showcase"/>
                                <div class=r#"prose prose-slate mt-14 [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#></div>
                                // dangerouslySetInnerHTML={{ __html: episode.content }}
                                // <h2 class="mt-2 text-2xl font-bold text-slate-900">Showcase</h2>
                                {issue
                                    .showcases
                                    .into_iter()
                                    .map(|showcase| {
                                        view! { <ShowcaseView showcase=showcase/> }
                                    })
                                    .collect::<Vec<_>>()}
                                // <h2 class="mt-2 text-2xl font-bold text-slate-900">Crates</h2>
                                <Divider title="Crates"/>

                                {issue
                                    .crate_releases
                                    .into_iter()
                                    .map(|crate_release| {
                                        view! { <CrateReleaseView crate_release=crate_release/> }
                                    })
                                    .collect_view()}

                                <Divider title="Devlogs"/>
                                // <h2 class="mt-2 text-2xl font-bold text-slate-900">Devlogs</h2>
                                <Divider title="Educational"/>
                                // <h2 class="mt-2 text-2xl font-bold text-slate-900">Education</h2>
                                // <Divider title="Fixes and Features"/>
                                // <h2 class="mt-2 text-2xl font-bold text-slate-900">
                                // Pull Requests Merged this week
                                // </h2>
                                <Divider title="Pull Requests Merged This Week"/>
                                <ul role="list" class="space-y-6 mt-6">

                                    {issue
                                        .merged_pull_requests
                                        .iter()
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
                                </CalloutInfo>
                                <h2 class="mt-6 text-2xl font-bold text-slate-900">
                                    Pull Requests Opened this week
                                </h2>
                                <ul role="list" class="space-y-6 mt-6">
                                    {issue
                                        .new_pull_requests
                                        .iter()
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
                                <h2 class="mt-6 text-2xl font-bold text-slate-900">
                                    Issues Opened this week
                                </h2>
                                <ul role="list" class="space-y-6 mt-6">
                                    {issue
                                        .new_github_issues
                                        .iter()
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
                <div class="w-px bg-gray-200"></div>
            </div>
            <div class="relative flex h-6 w-6 flex-none items-center justify-center bg-white">

                {match icon {
                    ActivityListIcon::Default => {
                        view! {
                            <>
                                <div class="h-1.5 w-1.5 rounded-full bg-gray-100 ring-1 ring-gray-300"></div>
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
            <p class="flex-auto py-0.5 text-xs leading-5 text-gray-500">
                <a href=url class="font-medium text-gray-900">
                    {title}
                </a>
                " authored by "
                <span class="font-medium text-gray-900">{author}</span>
            </p>
            <time
                datetime=date.to_string()
                class="flex-none py-0.5 text-xs leading-5 text-gray-500"
            >
                {date.to_string()}
            </time>
        </li>
    }
}
#[component]
#[allow(dead_code)]
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
    dbg!(url);
    view! {
        <li class="relative flex gap-x-4">
            <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                <div class="w-px bg-gray-200"></div>
            </div>
            <img
                src="/this-week-in-bevydark"
                alt=""
                class="relative mt-3 h-6 w-6 flex-none rounded-full bg-gray-50"
            />
            <div class="flex-auto rounded-md p-3 ring-1 ring-inset ring-gray-200">
                <div class="flex justify-between gap-x-4">
                    <div class="py-0.5 text-xs leading-5 text-gray-500">
                        <span class="font-medium text-gray-900">{author}</span>
                        commented
                    </div>
                    <time datetime=time class="flex-none py-0.5 text-xs leading-5 text-gray-500">
                        3d ago
                    </time>
                </div>
                <p class="text-sm leading-6 text-gray-500">{comment}</p>
            </div>
        </li>
    }
}

#[component]
#[allow(dead_code)]
fn CrateReleaseView(
    crate_release: CrateRelease
) -> impl IntoView {
    view! {
        <h3 class="mt-2 text-xl font-bold text-slate-900">{crate_release.title}</h3>
        <ul
            role="list"
            class="mt-3 grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
        >

            {crate_release
                .images
                .iter()
                .map(|image| {
                    view! {
                        <li class="relative">
                            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                                <img
                                    src=&image.url
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
        <div
            class=r#"mt-3 prose prose-slate [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#
            inner_html=crate_release.description
        ></div>
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
                        <img class="mt-12" src=&image.url alt=&image.description/>
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
        <div class="flex">
            <h3 class="mt-2 text-xl font-bold text-slate-900">{showcase.title}</h3>
            {showcase
                .discord_url
                .is_empty()
                .not()
                .then_some(
                    view! {
                        <a
                            href=&showcase.discord_url
                            class="text-indigo-700 hover:text-indigo-400 flex items-end ml-4"
                        >
                            <span>discord</span>
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
                    },
                )}

        </div>
        <div
            class=r#"mt-3 prose prose-slate [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#
            inner_html=showcase.description
        ></div>
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
            CalloutType::Info => "bg-blue-50",
            CalloutType::Caution => "bg-yellow-50",
            CalloutType::Warning => "bg-red-50",
            CalloutType::Success => "bg-green-50",
        }
    }
    fn icon(&self) -> &str {
        match self {
            CalloutType::Info => "text-blue-400",
            CalloutType::Caution => "text-yellow-400",
            CalloutType::Warning => "text-red-400",
            CalloutType::Success => "text-green-400",
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
            CalloutType::Info => "text-blue-800",
            CalloutType::Caution => "text-yellow-800",
            CalloutType::Warning => "text-red-800",
            CalloutType::Success => "text-green-800",
        }
    }
    fn content(&self) -> &str {
        match self {
            CalloutType::Info => "text-blue-700",
            CalloutType::Caution => "text-yellow-700",
            CalloutType::Warning => "text-red-700",
            CalloutType::Success => "text-green-700",
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
