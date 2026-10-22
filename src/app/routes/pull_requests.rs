use itertools::Itertools;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::types::Json;
#[cfg(feature = "ssr")]
use time::Date;

use crate::app::components::Divider;

#[component]
pub fn PullRequests() -> impl IntoView {
    let merged_prs =
        Resource::new(move || {}, |_| fetch_prs());

    view! {
        <div class="bg-slate-900">
        <div class="bg-sky-700 pt-8">
        <div class="md:flex md:items-center md:justify-between pt-4 px-8 max-w-3xl mx-auto">
            <div class="min-w-0 flex-1">
                <h2 class="text-2xl/7 font-bold text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight dark:text-white">Recent Merged Pull Requests</h2>
                <p class="mt-2 max-w-2xl text-slate-200">"All Bevy features make their way into Pull Requests, which can be reviewed by Subject Matter Experts and the community, before being merged into the project. The project uses tags with different categories, such a `A-` for areas of the engine, like `Rendering` or `Scenes`"</p>
            </div>
        </div>
                <Divider title=""/>
        </div>
        //   <nav class="flex overflow-x-auto border-b border-gray-200 py-4 dark:border-white/10">
        //     <ul role="list" class="flex min-w-full flex-none gap-x-6 px-4 text-sm/6 font-semibold text-gray-500 sm:px-6 lg:px-8 dark:text-gray-400">
        //     <li>
        //         <a href="/pull-requests" class="text-sky-600 dark:text-sky-400">Merged</a>
        //     </li>
        //     <li>
        //         <a href="/pull-requests/" class="">Activity</a>
        //     </li>
        //     <li>
        //         <a href="#" class="">Settings</a>
        //     </li>
        //     <li>
        //         <a href="#" class="">Collaborators</a>
        //     </li>
        //     <li>
        //         <a href="#" class="">Notifications</a>
        //     </li>
        //     </ul>
        // </nav>
        <ul role="list" class="divide-y divide-gray-100 dark:divide-white/5 max-w-3xl px-8 mx-auto">
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
            {move || Suspend::new(async move {
                let Ok(merged) = merged_prs.await else {
                    return None;
                };

            Some(merged
                .into_iter()
                // .sorted_by_key(|pr| &pr.merged_at_date)
                .map(|pull_request| {
                    view! {
                        <MergedPR
                            pr=pull_request
                        />
                    }
                })
                .collect_view())
                })}
            </Suspense>
        </ul>
        </div>
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MergedPR {
    pub github_id: String,
    pub title: String,
    pub url: String,
    pub merged_at_date: String,
    pub labels: Vec<GitHubLabel>,
    /* author: String,
     * author_url: String, */
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlMergedPR {
    github_id: String,
    title: String,
    url: String,
    merged_at_date: Option<Date>,
    labels: Option<Json<Vec<GitHubLabel>>>,
    /*serde_json::Value,
     * author: String,
     * author_url: String, */
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHubLabel {
    name: String,
    color: String,
}

#[cfg(feature = "ssr")]
impl From<SqlMergedPR> for MergedPR {
    fn from(value: SqlMergedPR) -> Self {
        let labels = match value.labels {
            Some(json) => json.0,
            None => vec![],
        };

        MergedPR {
            github_id: value.github_id,
            title: value.title,
            url: value.url,
            merged_at_date: value
                .merged_at_date
                .map(|v| v.to_string())
                .unwrap_or("".to_string()),
            labels,
        }
    }
}

#[server]
pub async fn fetch_prs(
) -> Result<Vec<MergedPR>, ServerFnError> {
    let pool = crate::sql::pool()?;

    let prs: Vec<SqlMergedPR> = sqlx::query_as!(
        SqlMergedPR,
        r#"SELECT 
        github_id,
        title,
        url,
        merged_at_date,
        labels as "labels: Json<Vec<GitHubLabel>>"
FROM merged_pull_request
ORDER BY merged_at_date DESC
LIMIT 100"#
    )
    .fetch_all(&pool)
    .await?;

    Ok(prs.into_iter().map(MergedPR::from).collect())
}

#[component]
fn MergedPR(pr: MergedPR) -> impl IntoView {
    let merged_at_date = pr.merged_at_date.clone();
    let title = pr.title.clone();
    let filtered_labels: Vec<GitHubLabel> = pr
        .labels
        .into_iter()
        .filter(|v| !v.name.starts_with("S-"))
        .sorted_by_key(|v| v.name.clone())
        .collect();
    let github_id = pr.github_id.clone();

    view! {
    // <!-- Include this script tag or install `@tailwindplus/elements` via npm: -->
    // <!-- <script src="https://cdn.jsdelivr.net/npm/@tailwindplus/elements@1" type="module"></script> -->
            <li class="flex items-center justify-between gap-x-6 py-5">

                <div class="min-w-0">
                    <div class="mt-1 flex items-center gap-x-2 text-xs/5 text-gray-500 dark:text-gray-400">
                        <p class="whitespace-nowrap">Merged on <time datetime={merged_at_date}>{pr.merged_at_date}</time></p>
                    </div>
                <div class="flex items-start gap-x-3">
                    <p class="text-sm/6 font-semibold text-gray-900 dark:text-white">
                        #{pr.github_id.clone()}
                        <svg viewBox="0 0 2 2" class="size-0.5 fill-current inline mx-2">
                            <circle r="1" cx="1" cy="1" />
                        </svg>
                        {pr.title}
                    </p>
                </div>
                <div class="flex items-start gap-3 mt-2 flex-wrap">
                    <For
                        each=move || filtered_labels.clone()
                        key=|label| label.name.clone()
                        children={move |label: GitHubLabel| {
                            view! {
                                <span class="pill-lighten-text inline-flex items-center gap-x-1.5 rounded-md px-2 py-1 text-xs font-medium inset-ring"
                                    style=format!("--pill-text-color: #{}; --tw-inset-ring-color: color-mix(in oklab, #{} 40%, transparent);", label.color, label.color)
                                >
                                    <svg viewBox="0 0 6 6" aria-hidden="true" class="size-1.5"
                                        style=format!("fill: #{};", label.color)
                                    >
                                        <circle r="3" cx="3" cy="3" />
                                    </svg>
                                    {label.name}
                                </span>
                                // <button
                                //     class="pill-lighten-text mt-0.5 rounded-md bg-green-50 px-1.5 py-0.5 text-xs font-medium inset-ring inset-ring-green-600/20 dark:bg-green-400/10 dark:inset-ring-green-500/20"
                                //     style=format!("--pill-text-color: #{}; background: #{};", label.color, label.color)>
                                //     {label.name}
                                // </button>
                            }
                        }}
                    />
                </div>
                    // <p class="mt-0.5 rounded-md bg-green-50 px-1.5 py-0.5 text-xs font-medium text-green-700 inset-ring inset-ring-green-600/20 dark:bg-green-400/10 dark:text-green-400 dark:inset-ring-green-500/20">Complete</p>
                </div>
                <div class="flex flex-none items-center gap-x-4">
                <a
                    href=format!("https://github.com/bevyengine/bevy/pull/{}", github_id)
                    class="hidden rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-xs inset-ring inset-ring-gray-300 hover:bg-gray-50 sm:block dark:bg-white/10 dark:text-white dark:shadow-none dark:inset-ring-white/5 dark:hover:bg-white/20">
                    View #{pr.github_id}
                </a>
                // <el-dropdown class="relative flex-none">
                //     <button class="relative block text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">
                //     <span class="absolute -inset-2.5"></span>
                //     <span class="sr-only">Open options</span>
                //     <svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5">
                //         <path d="M10 3a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM10 8.5a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM11.5 15.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z" />
                //     </svg>
                //     </button>
                //     <el-menu anchor="bottom end" popover class="w-32 origin-top-right rounded-md bg-white py-2 shadow-lg outline-1 outline-gray-900/5 transition transition-discrete [--anchor-gap:--spacing(2)] data-closed:scale-95 data-closed:transform data-closed:opacity-0 data-enter:duration-100 data-enter:ease-out data-leave:duration-75 data-leave:ease-in dark:bg-gray-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">
                //     <a href="#" class="block px-3 py-1 text-sm/6 text-gray-900 focus:bg-gray-50 focus:outline-hidden dark:text-white dark:focus:bg-white/5">Edit<span class="sr-only">, GraphQL API</span></a>
                //     <a href="#" class="block px-3 py-1 text-sm/6 text-gray-900 focus:bg-gray-50 focus:outline-hidden dark:text-white dark:focus:bg-white/5">Move<span class="sr-only">, GraphQL API</span></a>
                //     <a href="#" class="block px-3 py-1 text-sm/6 text-gray-900 focus:bg-gray-50 focus:outline-hidden dark:text-white dark:focus:bg-white/5">Delete<span class="sr-only">, GraphQL API</span></a>
                //     </el-menu>
                // </el-dropdown>
                </div>
            </li>
        }
}
