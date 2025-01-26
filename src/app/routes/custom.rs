use std::ops::Not;

use crate::app::components::{Container, Divider};
use futures::future::join4;
use leptos::prelude::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Issue {
    /// The title of the issue is technically the
    /// date in yyyy-mm-dd format, but this field
    /// is a human-readable string that goes in
    /// the email subject line and slug url
    title: String,
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
    crates: Vec<CrateRelease>,
    /// merged pull requests
    /// Meant to convey what is being added to
    /// Bevy
    pull_requests: Vec<PullRequest>,
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
    new_issues: Vec<NewIssue>,
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
#[serde(rename_all = "camelCase")]
struct NewPullRequest {
    author: String,
    github_id: String,
    title: String,
    url: String,
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
    author: String,
    github_id: String,
    title: String,
    url: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct CrateRelease {
    title: String,
    description: String,
    url: String,
    images: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Educational {
    title: String,
    description: String,
    url: String,
    images: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Showcase;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PullRequest {
    author: PullRequestAuthor,
    closed_at: String,
    number: u32,
    title: String,
    url: String,
}
#[derive(Clone, Serialize, Deserialize)]
struct PullRequestAuthor {
    login: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Contributor;

#[server]
async fn fetch_issue(
    _date: time::Date,
) -> Result<Issue, leptos::ServerFnError> {
    use crate::markdown::compile;
    let crates = vec![CrateRelease {
            title: "Hexx 0.13.0".to_string(),
            description: compile("hexx is an hexagonal utility library made with bevy integration in mind

What's new:
* A new sprite sheet example
* A lot of improvements in mesh generation, especially with procedural UV coordinates
* Quality of life improvements
* Fixes !"),
            url: "https://discord.com/channels/691052431525675048/918591326096850974/1202285063358648350".to_string(),
            images: vec!["/test_one.png".to_string(), "/test_two.png".to_string(), "/test_three.png".to_string()],
        },
        CrateRelease{
            title: "bevy_smooth_pixel_camera".to_string(),
            description: "bugfix release".to_string(),
            url: "".to_string(),
            images: vec![]
        }];
    Ok(Issue {
        title: "The one before Bevy 0.13!".to_string(),
        description: compile("It's a new year and there's another month or two until the 0.13 release.

There are three things in this to pay attention to

* Thing A
* Thing B
* Thing C

and a quote from a contributor

> Some Quote
> - [Contributor Person](https://github.com/bevyengine/bevy)"),
        showcases: vec![],
        crates,
        pull_requests: vec![],
        contributors: vec![],
        educational: vec![],
        new_pull_requests: vec![],
        new_issues: vec![],
    })
}

#[component]
pub fn Issue() -> impl IntoView {
    let params = use_params_map();
    // slug id only cares about the date, which is the
    // unique id. the rest of the slug can be
    // changed any time.
    // 2024-02-11-the-one-before-bevy-0-13
    let issue = Resource::new(
        move || {
            params.with(|p| {
                p.get("slug").cloned().and_then(|slug|
                crate::issue_date::parse_issue_date_from_slug(&slug)
            )
            })
        },
        |date| async move {
            let date = date?;

            Some(
                join4(
                    fetch_issue(date),
                    crate::sql::get_merged_pull_requests(
                        date,
                    ),
                    crate::sql::get_opened_pull_requests(
                        date,
                    ),
                    crate::sql::get_opened_issues(date),
                )
                .await,
            )
        },
    );
    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || match issue.get() {
                None | Some(None) => view! { <article>"404"</article> },
                Some(Some((issue, merged_pull_requests, opened_pull_requests, opened_issues))) => {
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
                                                {issue.as_ref().unwrap().title.clone()}
                                            </h1>
                                            // <FormattedDate
                                            // date={date}
                                            <p class="order-first font-mono text-sm leading-7 text-slate-500">
                                                // />
                                                "2024-02-11"
                                            </p>
                                        </div>
                                    </div>
                                    // <p class="ml-24 mt-3 text-lg font-medium leading-8 text-slate-700">
                                    // {issue.as_ref().unwrap().description.clone()}
                                    // </p>
                                    <div
                                        class=r#"mt-3 leading-8 text-slate-700 prose [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#
                                        inner_html=issue.as_ref().unwrap().description.clone()
                                    ></div>
                                </header>
                                <Divider title="Community Showcase"/>
                                <div class=r#"prose prose-slate mt-14 [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#></div>
                                // dangerouslySetInnerHTML={{ __html: episode.content }}
                                <h2 class="mt-2 text-2xl font-bold text-slate-900">Showcase</h2>
                                <h2 class="mt-2 text-2xl font-bold text-slate-900">Crates</h2>

                                {issue
                                    .as_ref()
                                    .unwrap()
                                    .crates
                                    .iter()
                                    .map(|crate_release| {
                                        view! {
                                            <CrateRelease
                                                title=&crate_release.title
                                                description=&crate_release.description
                                                url=&crate_release.url
                                                images=crate_release.images.clone()
                                            />
                                        }
                                    })
                                    .collect::<Vec<_>>()}

                                <h2 class="mt-2 text-2xl font-bold text-slate-900">Devlogs</h2>
                                <h2 class="mt-2 text-2xl font-bold text-slate-900">Education</h2>
                                <Divider title="Fixes and Features"/>
                                <h2 class="mt-2 text-2xl font-bold text-slate-900">
                                    Pull Requests Merged this week
                                </h2>
                                <ul role="list" class="space-y-6 mt-6">

                                    {merged_pull_requests
                                        .expect("")
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
                                        .collect::<Vec<_>>()}

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

                                    {opened_pull_requests
                                        .expect("")
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
                                        .collect::<Vec<_>>()}

                                </ul>
                                <h2 class="mt-6 text-2xl font-bold text-slate-900">
                                    Issues Opened this week
                                </h2>
                                <ul role="list" class="space-y-6 mt-6">

                                    {opened_issues
                                        .expect("")
                                        .iter()
                                        .map(|issue| {
                                            view! {
                                                <ActivityListItem
                                                    date=&issue.gh_created_at
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
            <time datetime=&date class="flex-none py-0.5 text-xs leading-5 text-gray-500">
                {date}
            </time>
        </li>
    }
}
#[component]
#[allow(unused_variables)]
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
#[allow(unused_variables)]
fn CrateRelease(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] url: String,
    #[prop(default=vec![])] images: Vec<String>,
) -> impl IntoView {
    view! {
        <h3 class="mt-2 text-xl font-bold text-slate-900">{title}</h3>
        <ul
            role="list"
            class="mt-3 grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
        >

            {images
                .iter()
                .map(|image| {
                    view! {
                        <li class="relative">
                            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                                <img
                                    src=image
                                    alt=""
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
        <div
            class=r#"mt-3 prose prose-slate [&>h2:nth-of-type(3n)]:before:bg-violet-200 [&>h2:nth-of-type(3n+2)]:before:bg-indigo-200 [&>h2]:mt-12 [&>h2]:flex [&>h2]:items-center [&>h2]:font-mono [&>h2]:text-sm [&>h2]:font-medium [&>h2]:leading-7 [&>h2]:text-slate-900 [&>h2]:before:mr-3 [&>h2]:before:h-3 [&>h2]:before:w-1.5 [&>h2]:before:rounded-r-full [&>h2]:before:bg-cyan-200 [&>ul]:mt-6 [&>ul]:list-['\2013\20'] [&>ul]:pl-5"#
            inner_html=description
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

#[allow(dead_code)]
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
