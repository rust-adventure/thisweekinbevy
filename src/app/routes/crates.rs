use itertools::Itertools;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::types::Json;
#[cfg(feature = "ssr")]
use time::Date;

#[component]
pub fn Crates() -> impl IntoView {
    let crate_infos =
        Resource::new(move || {}, |_| fetch_crate_info());

    view! {
        <div class="md:flex md:items-center md:justify-between pt-4 px-8">
            <div class="min-w-0 flex-1">
                <h2 class="text-2xl/7 font-bold text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight dark:text-white">Crates</h2>
                <p class="mt-2 max-w-2xl text-slate-200">"A"</p>
            </div>
        </div>
        <div class="flex">
        <CratesNav/>
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

        // <ul role="list" class="divide-y divide-gray-100 dark:divide-white/5 max-w-3xl mx-auto">
        <div class="px-4">
        <h2 class="text-2xl/7 font-bold text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight dark:text-white">Physics</h2>
        <div class="grid grid-cols-1 gap-y-4 sm:grid-cols-2 sm:gap-x-6 sm:gap-y-10 lg:grid-cols-3 lg:gap-x-8 mt-2">
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
            {move || Suspend::new(async move {
                let Ok(data) = crate_infos.await else {
                    return None;
                };

            Some(data
                .into_iter()
                // .sorted_by_key(|pr| &pr.merged_at_date)
                .map(|crate_info| {
                    view! {
                        <CrateInfo
                            crate_info=crate_info
                        />
                    }
                })
                .collect_view())
                })}
            </Suspense>
            </div>
        </div>
        </div>
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CrateInfo {
    pub crate_id: String,
    pub display_name: String,
    pub description: String,
}

#[cfg(feature = "ssr")]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    sqlx::FromRow,
)]
struct SqlCrateInfo {
    crate_id: Vec<u8>,
    display_name: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHubLabel {
    name: String,
    color: String,
}

#[cfg(feature = "ssr")]
impl From<SqlCrateInfo> for CrateInfo {
    fn from(value: SqlCrateInfo) -> Self {
        let id = rusty_ulid::Ulid::try_from(
            value.crate_id.as_slice(),
        )
        .expect("expect valid ids from the database");
        CrateInfo {
            crate_id: id.to_string(),
            display_name: value.display_name,
            description: value
                .description
                .unwrap_or("".to_string()),
        }
    }
}

#[server]
pub async fn fetch_crate_info(
) -> Result<Vec<CrateInfo>, ServerFnError> {
    let pool = crate::sql::pool()?;

    let infos: Vec<SqlCrateInfo> = sqlx::query_as!(
        SqlCrateInfo,
        r#"SELECT 
        crate_id,
        display_name,
        description
FROM crate
"#
    )
    .fetch_all(&pool)
    .await?;

    Ok(infos.into_iter().map(CrateInfo::from).collect())
}

#[component]
fn CrateInfo(crate_info: CrateInfo) -> impl IntoView {
    let display_name = crate_info.display_name.clone();
    let description = crate_info.description.clone();
    view! {
        <div class="group relative flex flex-col overflow-hidden rounded-lg border border-slate-800 bg-slate-900">
            <img src="https://placekittens.com/g/480/270" alt="" class="aspect-video bg-slate-900 group-hover:opacity-75" />
            <div class="flex flex-1 flex-col space-y-2 p-4">
                <h3 class="text-sm font-medium text-slate-50">
                    <a href="#">
                    <span aria-hidden="true" class="absolute inset-0"></span>
                        {display_name}
                    </a>
                </h3>
                <p class="text-sm text-slate-400">{description}</p>
                <div class="flex flex-1 flex-col justify-end">
                    <p class="text-sm text-slate-400 italic">8 crates</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct Category {
    display_name: String,
    crate_count: u32,
}

#[component]
fn CratesNav() -> impl IntoView {
    let options = vec![
        Category {
            display_name: "Physics".to_string(),
            crate_count: 2,
        },
        Category {
            display_name: "User Interface".to_string(),
            crate_count: 8,
        },
        Category {
            display_name: "Pathfinding and Navigation"
                .to_string(),
            crate_count: 4,
        },
        Category {
            display_name: "Assets".to_string(),
            crate_count: 20,
        },
        Category {
            display_name: "Rendering".to_string(),
            crate_count: 5,
        },
        Category {
            display_name: "2d Lighting".to_string(),
            crate_count: 4,
        },
        Category {
            display_name: "Integrations".to_string(),
            crate_count: 8,
        },
        Category {
            display_name: "Terrain".to_string(),
            crate_count: 1,
        },
        Category {
            display_name: "Audio".to_string(),
            crate_count: 2,
        },
        Category {
            display_name: "Networking".to_string(),
            crate_count: 4,
        },
        Category {
            display_name: "Templates".to_string(),
            crate_count: 7,
        },
        Category {
            display_name: "VFX".to_string(),
            crate_count: 3,
        },
        Category {
            display_name: "Animation".to_string(),
            crate_count: 0,
        },
        Category {
            display_name: "Utility".to_string(),
            crate_count: 0,
        },
    ];
    view! {
        <nav aria-label="Sidebar" class="flex grow-0 shrink-0 basis-[225px] flex-col p-6 h-full">
            <ul role="list" class="-mx-2 space-y-1">
                <For
                    each=move || options.clone()
                    key=|category| category.display_name.clone()
                    children=move |category| {
                        view! {
                            <a
                                href="#"
                            // class="group flex gap-x-3 rounded-md bg-gray-50 p-2 pl-3 text-sm/6 font-semibold text-indigo-600 dark:bg-white/5 dark:text-white"
                                class="group flex gap-x-3 rounded-md p-2 pl-3 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white"
                            >
                                {category.display_name}
                                {
                                    if category.crate_count > 1 {
                                        view! {
                                            <span aria-hidden="true" class="ml-auto w-9 min-w-max rounded-full bg-white px-2.5 py-0.5 text-center text-xs/5 font-medium whitespace-nowrap text-gray-600 outline-1 -outline-offset-1 outline-gray-200 dark:bg-gray-900 dark:text-gray-400 dark:outline-white/10 h-fit">
                                                {category.crate_count}
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <span/>
                                        }.into_any()
                                    }
                                }
                            </a>
                        }
                    }
                />
                // <!-- Current: "bg-gray-50 dark:bg-white/5 text-indigo-600 dark:text-white", Default: "text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-white/5" -->
                // <li>
                // </li>
                // <li>
                // <a href="#" class="group flex gap-x-3 rounded-md p-2 pl-3 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white">
                // User Interface
                // </a>
                // </li>
                // <li>
                // <a href="#" class="group flex gap-x-3 rounded-md p-2 pl-3 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white">
                //     Pathfinding and Navigation
                //     <span aria-hidden="true" class="ml-auto w-9 min-w-max rounded-full bg-white px-2.5 py-0.5 text-center text-xs/5 font-medium whitespace-nowrap text-gray-600 outline-1 -outline-offset-1 outline-gray-200 dark:bg-gray-900 dark:text-gray-400 dark:outline-white/10">12</span>
                // </a>
                // </li>
                // <li>
                // <a href="#" class="group flex gap-x-3 rounded-md p-2 pl-3 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white">
                //     Assets
                //     <span aria-hidden="true" class="ml-auto w-9 min-w-max rounded-full bg-white px-2.5 py-0.5 text-center text-xs/5 font-medium whitespace-nowrap text-gray-600 outline-1 -outline-offset-1 outline-gray-200 dark:bg-gray-900 dark:text-gray-400 dark:outline-white/10">20+</span>
                // </a>
                // </li>
                // <li>
                // <a href="#" class="group flex gap-x-3 rounded-md p-2 pl-3 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white">
                //     Rendering
                // </a>
                // </li>
                // <li>
                // <a href="#" class="group flex gap-x-3 rounded-md p-2 pl-3 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white">
                //     2d Lighting
                // </a>
                // </li>
            </ul>
        </nav>
    }
}
