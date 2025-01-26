use crate::app::components::Divider;
use leptos::prelude::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Issue() -> impl IntoView {
    let params = use_params_map();
    let issue = create_resource(
        move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        },
        fetch_issue,
    );
    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    issue
                        .get()
                        .map(|data| match data {
                            Err(e) => view! { <pre>{e.to_string()}</pre> }.into_view(),
                            Ok(issue) => {
                                issue
                                    .map(|issue| {
                                        view! { <IssueForm issue=issue/> }
                                    })
                                    .collect_view()
                            }
                        })
                }}

            </Suspense>
            <Divider title="Showcases"/>
            <Showcases/>
            <Divider title="Crate Releases"/>
            <CrateReleases/>
            <Divider title="Devlogs"/>
            <Devlogs/>
            <Divider title="Educationals"/>
            <Educationals/>
        </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlIssueData {
    id: Vec<u8>,
    slug: String,
    issue_date: time::Date,
    cloudinary_public_id: String,
    status: String,
    display_name: String,
    description: String,
    youtube_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueData {
    pub id: String,
    pub slug: String,
    pub issue_date: time::Date,
    pub cloudinary_public_id: String,
    pub status: String,
    pub display_name: String,
    pub description: String,
    pub youtube_id: String,
}

#[cfg(feature = "ssr")]
impl From<SqlIssueData> for IssueData {
    fn from(value: SqlIssueData) -> Self {
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        IssueData {
            id: id_str.to_string(),
            slug: value.slug,
            issue_date: value.issue_date,
            cloudinary_public_id: value
                .cloudinary_public_id,
            status: value.status,
            display_name: value.display_name,
            description: value.description,
            youtube_id: value.youtube_id,
        }
    }
}

#[server]
pub async fn fetch_issue(
    id: String,
) -> Result<Option<IssueData>, ServerFnError> {
    let id: [u8; 16] = id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue = sqlx::query_as!(
        SqlIssueData,
        "SELECT
            id,
            slug,
            issue_date,
            cloudinary_public_id,
            status,
            display_name,
            description,
            youtube_id
FROM issue
WHERE id = ?",
        id.as_slice()
    )
    .fetch_optional(&pool)
    .await?;

    Ok(issue.map(IssueData::from))
}

#[server]
pub async fn update_issue_metadata(
    issue_id: String,
    display_name: String,
    slug: String,
    cloudinary_public_id: String,
    youtube_id: String,
    description: String,
) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::MySqlPool>()
        .expect("to be able to access app_state");

    let _username = crate::sql::with_admin_access()?;

    let id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        r#"
    UPDATE issue 
    SET
        display_name = ?,
        slug = ?,
        cloudinary_public_id = ?,
        youtube_id = ?,
        description = ?
    WHERE id = ?
        "#,
        display_name,
        slug,
        cloudinary_public_id,
        youtube_id,
        description,
        id.as_slice()
    )
    .execute(&pool)
    .await
    .expect("successful insert");

    Ok(())
}

#[component]
fn IssueForm(issue: IssueData) -> impl IntoView {
    let update_issue_metadata =
        create_server_action::<UpdateIssueMetadata>();
    view! {
        <div class="isolate bg-white px-6 py-24 sm:py-32 lg:px-8">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                    {&issue.issue_date.to_string()}
                </h2>
                <p class="mt-2 text-lg leading-8 text-gray-600">{&issue.status}</p>
            </div>
            <ActionForm class="mx-auto mt-16 max-w-xl sm:mt-20" action=update_issue_metadata>
                <input type="hidden" name="issue_id" id="issue_id" value=&issue.id/>
                <div class="grid grid-cols-1 gap-x-8 gap-y-6 sm:grid-cols-2">
                    <div>
                        <label
                            for="display_name"
                            class="block text-sm font-semibold leading-6 text-gray-900"
                        >
                            display_name
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="text"
                                name="display_name"
                                id="display_name"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                value=&issue.display_name
                            />
                        </div>
                    </div>
                    <div>
                        <label
                            for="slug"
                            class="block text-sm font-semibold leading-6 text-gray-900"
                        >
                            slug
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="text"
                                name="slug"
                                id="slug"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                value=&issue.slug
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <label
                            for="cloudinary_public_id"
                            class="block text-sm font-semibold leading-6 text-gray-900"
                        >
                            cloudinary_public_id
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="text"
                                name="cloudinary_public_id"
                                id="cloudinary_public_id"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                value=&issue.cloudinary_public_id
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <label
                            for="youtube_id"
                            class="block text-sm font-semibold leading-6 text-gray-900"
                        >
                            youtube_id
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="youtube_id"
                                name="youtube_id"
                                id="youtube_id"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                value=&issue.youtube_id
                            />
                        </div>
                    </div>

                    <div class="sm:col-span-2">
                        <label
                            for="description"
                            class="block text-sm font-semibold leading-6 text-gray-900"
                        >
                            description
                        </label>
                        <div class="mt-2.5">
                            <textarea
                                name="description"
                                id="description"
                                rows="4"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                            >
                                {&issue.description}
                            </textarea>
                        </div>
                    </div>
                </div>
                <div class="mt-10">
                    <button
                        type="submit"
                        class="block w-full rounded-md bg-indigo-600 px-3.5 py-2.5 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                        Save
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
fn Showcases() -> impl IntoView {
    let params = use_params_map();

    let showcases = create_resource(
        move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        },
        fetch_showcases_for_issue_id,
    );

    view! {
        <ul
            role="list"
            class="divide-y divide-gray-100 overflow-hidden bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl"
        >
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    showcases
                        .get()
                        .map(|data| match data {
                            Err(e) => view! { <pre>{e.to_string()}</pre> }.into_view(),
                            Ok(showcases) => {
                                showcases
                                    .iter()
                                    .map(|showcase| {
                                        view! { <ShowcaseLi showcase=showcase.clone()/> }
                                    })
                                    .collect_view()
                            }
                        })
                }}

            </Suspense>
        </ul>
    }
}

#[component]
fn ShowcaseLi(showcase: ShowcaseData) -> impl IntoView {
    view! {
        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-gray-50 sm:px-6">
            <div class="flex min-w-0 gap-x-4">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-gray-900">
                        <a href=format!("/admin/showcase/{}", showcase.id)>
                            <span class="absolute inset-x-0 -top-px bottom-0"></span>
                            {showcase.title}
                        </a>
                    </p>
                    {showcase
                        .posted_date
                        .map(|posted_date| {
                            view! {
                                <p class="mt-1 flex text-xs leading-5 text-gray-500">
                                    <span>"posted at"</span>
                                    <time datetime=posted_date.to_string() class="ml-1">
                                        {posted_date.to_string()}
                                    </time>
                                </p>
                            }
                        })}

                </div>
            </div>
            <div class="flex shrink-0 items-center gap-x-4">
                <div class="hidden sm:flex sm:flex-col sm:items-end">
                    <p class="text-sm leading-6 text-gray-900">{showcase.image_count} images</p>
                // <p class="mt-1 text-xs leading-5 text-gray-500">Last seen <time datetime="2023-01-23T13:23Z">3h ago</time></p>
                </div>
                <svg
                    class="h-5 w-5 flex-none text-gray-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </div>
        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlShowcaseData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
    image_count: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ShowcaseData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
    pub image_count: u32,
}

#[cfg(feature = "ssr")]
impl From<SqlShowcaseData> for ShowcaseData {
    fn from(value: SqlShowcaseData) -> Self {
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        ShowcaseData {
            id: id_str.to_string(),
            title: value.title,
            posted_date: value.posted_date,
            image_count: value
                .image_count
                .unwrap_or_default()
                as u32,
        }
    }
}

#[server]
pub async fn fetch_showcases_for_issue_id(
    issue_id: String,
) -> Result<Vec<ShowcaseData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let showcases: Vec<SqlShowcaseData> = sqlx::query_as!(
        SqlShowcaseData,
        "SELECT
        showcase.id,
        showcase.title,
        showcase.posted_date,
        si.image_count
FROM issue__showcase
INNER JOIN showcase
  ON showcase.id = issue__showcase.showcase_id
LEFT JOIN (
    SELECT showcase__image.showcase_id, COUNT(*) as image_count
    FROM showcase__image
    GROUP BY showcase__image.showcase_id
) AS si ON si.showcase_id = showcase.id
WHERE issue__showcase.issue_id = ?
ORDER BY showcase.posted_date",
issue_id.as_slice()
    )
    .fetch_all(&pool)
    .await?;

    Ok(showcases
        .into_iter()
        .map(ShowcaseData::from)
        .collect())
}

// crate_releases

#[component]
fn CrateReleases() -> impl IntoView {
    let params = use_params_map();

    let crate_releases = create_resource(
        move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        },
        fetch_crate_releases_for_issue_id,
    );

    view! {
        <ul
            role="list"
            class="divide-y divide-gray-100 overflow-hidden bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl"
        >
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    crate_releases
                        .get()
                        .map(|data| match data {
                            Err(e) => view! { <pre>{e.to_string()}</pre> }.into_view(),
                            Ok(crate_releases) => {
                                crate_releases
                                    .iter()
                                    .map(|crate_release| {
                                        view! {
                                            <CrateReleaseLi crate_release=crate_release.clone()/>
                                        }
                                    })
                                    .collect_view()
                            }
                        })
                }}

            </Suspense>
        </ul>
    }
}

#[component]
fn CrateReleaseLi(
    crate_release: CrateReleaseData,
) -> impl IntoView {
    view! {
        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-gray-50 sm:px-6">
            <div class="flex min-w-0 gap-x-4">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-gray-900">
                        <a href=format!("/admin/crate_release/{}", crate_release.id)>
                            <span class="absolute inset-x-0 -top-px bottom-0"></span>
                            {crate_release.title}
                        </a>
                    </p>
                    {crate_release
                        .posted_date
                        .map(|posted_date| {
                            view! {
                                <p class="mt-1 flex text-xs leading-5 text-gray-500">
                                    <span>"posted at"</span>
                                    <time datetime=posted_date.to_string() class="ml-1">
                                        {posted_date.to_string()}
                                    </time>
                                </p>
                            }
                        })}

                </div>
            </div>
            <div class="flex shrink-0 items-center gap-x-4">
                <div class="hidden sm:flex sm:flex-col sm:items-end">
                    <p class="text-sm leading-6 text-gray-900">
                        {crate_release.image_count} images
                    </p>
                // <p class="mt-1 text-xs leading-5 text-gray-500">Last seen <time datetime="2023-01-23T13:23Z">3h ago</time></p>
                </div>
                <svg
                    class="h-5 w-5 flex-none text-gray-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </div>
        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlCrateReleaseData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
    image_count: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CrateReleaseData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
    pub image_count: u32,
}

#[cfg(feature = "ssr")]
impl From<SqlCrateReleaseData> for CrateReleaseData {
    fn from(value: SqlCrateReleaseData) -> Self {
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        CrateReleaseData {
            id: id_str.to_string(),
            title: value.title,
            posted_date: value.posted_date,
            image_count: value
                .image_count
                .unwrap_or_default()
                as u32,
        }
    }
}

#[server]
pub async fn fetch_crate_releases_for_issue_id(
    issue_id: String,
) -> Result<Vec<CrateReleaseData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let crate_releases: Vec<SqlCrateReleaseData> = sqlx::query_as!(
        SqlCrateReleaseData,
        "SELECT
        crate_release.id,
        crate_release.title,
        crate_release.posted_date,
        si.image_count
FROM issue__crate_release
INNER JOIN crate_release
  ON crate_release.id = issue__crate_release.crate_release_id
LEFT JOIN (
    SELECT crate_release__image.crate_release_id, COUNT(*) as image_count
    FROM crate_release__image
    GROUP BY crate_release__image.crate_release_id
) AS si ON si.crate_release_id = crate_release.id
WHERE issue__crate_release.issue_id = ?
ORDER BY crate_release.posted_date",
issue_id.as_slice()
    )
    .fetch_all(&pool)
    .await?;

    Ok(crate_releases
        .into_iter()
        .map(CrateReleaseData::from)
        .collect())
}

// devlogs

#[component]
fn Devlogs() -> impl IntoView {
    let params = use_params_map();

    let devlogs = create_resource(
        move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        },
        fetch_devlogs_for_issue_id,
    );

    view! {
        <ul
            role="list"
            class="divide-y divide-gray-100 overflow-hidden bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl"
        >
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    devlogs
                        .get()
                        .map(|data| match data {
                            Err(e) => view! { <pre>{e.to_string()}</pre> }.into_view(),
                            Ok(devlogs) => {
                                devlogs
                                    .iter()
                                    .map(|devlog| {
                                        view! { <DevlogLi devlog=devlog.clone()/> }
                                    })
                                    .collect_view()
                            }
                        })
                }}

            </Suspense>
        </ul>
    }
}

#[component]
fn DevlogLi(devlog: DevlogData) -> impl IntoView {
    view! {
        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-gray-50 sm:px-6">
            <div class="flex min-w-0 gap-x-4">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-gray-900">
                        <a href=format!("/admin/devlog/{}", devlog.id)>
                            <span class="absolute inset-x-0 -top-px bottom-0"></span>
                            {devlog.title}
                        </a>
                    </p>
                    {devlog
                        .posted_date
                        .map(|posted_date| {
                            view! {
                                <p class="mt-1 flex text-xs leading-5 text-gray-500">
                                    <span>"posted at"</span>
                                    <time datetime=posted_date.to_string() class="ml-1">
                                        {posted_date.to_string()}
                                    </time>
                                </p>
                            }
                        })}

                </div>
            </div>
            <div class="flex shrink-0 items-center gap-x-4">
                <div class="hidden sm:flex sm:flex-col sm:items-end">
                    <p class="text-sm leading-6 text-gray-900">{devlog.image_count} images</p>
                // <p class="mt-1 text-xs leading-5 text-gray-500">Last seen <time datetime="2023-01-23T13:23Z">3h ago</time></p>
                </div>
                <svg
                    class="h-5 w-5 flex-none text-gray-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </div>
        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlDevlogData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
    image_count: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DevlogData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
    pub image_count: u32,
}

#[cfg(feature = "ssr")]
impl From<SqlDevlogData> for DevlogData {
    fn from(value: SqlDevlogData) -> Self {
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        DevlogData {
            id: id_str.to_string(),
            title: value.title,
            posted_date: value.posted_date,
            image_count: value
                .image_count
                .unwrap_or_default()
                as u32,
        }
    }
}

#[server]
pub async fn fetch_devlogs_for_issue_id(
    issue_id: String,
) -> Result<Vec<DevlogData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let devlogs: Vec<SqlDevlogData> = sqlx::query_as!(
        SqlDevlogData,
        "SELECT
        devlog.id,
        devlog.title,
        devlog.posted_date,
        si.image_count
FROM issue__devlog
INNER JOIN devlog
  ON devlog.id = issue__devlog.devlog_id
LEFT JOIN (
    SELECT devlog__image.devlog_id, COUNT(*) as image_count
    FROM devlog__image
    GROUP BY devlog__image.devlog_id
) AS si ON si.devlog_id = devlog.id
WHERE issue__devlog.issue_id = ?
ORDER BY devlog.posted_date",
        issue_id.as_slice()
    )
    .fetch_all(&pool)
    .await?;

    Ok(devlogs.into_iter().map(DevlogData::from).collect())
}

// educationals

#[component]
fn Educationals() -> impl IntoView {
    let params = use_params_map();

    let educationals = create_resource(
        move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        },
        fetch_educationals_for_issue_id,
    );

    view! {
        <ul
            role="list"
            class="divide-y divide-gray-100 overflow-hidden bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl"
        >
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    educationals
                        .get()
                        .map(|data| match data {
                            Err(e) => view! { <pre>{e.to_string()}</pre> }.into_view(),
                            Ok(educationals) => {
                                educationals
                                    .iter()
                                    .map(|educational| {
                                        view! { <EducationalLi educational=educational.clone()/> }
                                    })
                                    .collect_view()
                            }
                        })
                }}

            </Suspense>
        </ul>
    }
}

#[component]
fn EducationalLi(
    educational: EducationalData,
) -> impl IntoView {
    view! {
        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-gray-50 sm:px-6">
            <div class="flex min-w-0 gap-x-4">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-gray-900">
                        <a href=format!("/admin/educational/{}", educational.id)>
                            <span class="absolute inset-x-0 -top-px bottom-0"></span>
                            {educational.title}
                        </a>
                    </p>
                    {educational
                        .posted_date
                        .map(|posted_date| {
                            view! {
                                <p class="mt-1 flex text-xs leading-5 text-gray-500">
                                    <span>"posted at"</span>
                                    <time datetime=posted_date.to_string() class="ml-1">
                                        {posted_date.to_string()}
                                    </time>
                                </p>
                            }
                        })}

                </div>
            </div>
            <div class="flex shrink-0 items-center gap-x-4">
                <div class="hidden sm:flex sm:flex-col sm:items-end">
                    <p class="text-sm leading-6 text-gray-900">{educational.image_count} images</p>
                // <p class="mt-1 text-xs leading-5 text-gray-500">Last seen <time datetime="2023-01-23T13:23Z">3h ago</time></p>
                </div>
                <svg
                    class="h-5 w-5 flex-none text-gray-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </div>
        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlEducationalData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
    image_count: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EducationalData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
    pub image_count: u32,
}

#[cfg(feature = "ssr")]
impl From<SqlEducationalData> for EducationalData {
    fn from(value: SqlEducationalData) -> Self {
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        EducationalData {
            id: id_str.to_string(),
            title: value.title,
            posted_date: value.posted_date,
            image_count: value
                .image_count
                .unwrap_or_default()
                as u32,
        }
    }
}

#[server]
pub async fn fetch_educationals_for_issue_id(
    issue_id: String,
) -> Result<Vec<EducationalData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let educationals: Vec<SqlEducationalData> = sqlx::query_as!(
        SqlEducationalData,
        "SELECT
        educational.id,
        educational.title,
        educational.posted_date,
        si.image_count
FROM issue__educational
INNER JOIN educational
  ON educational.id = issue__educational.educational_id
LEFT JOIN (
    SELECT educational__image.educational_id, COUNT(*) as image_count
    FROM educational__image
    GROUP BY educational__image.educational_id
) AS si ON si.educational_id = educational.id
WHERE issue__educational.issue_id = ?
ORDER BY educational.posted_date",
        issue_id.as_slice()
    )
    .fetch_all(&pool)
    .await?;

    Ok(educationals
        .into_iter()
        .map(EducationalData::from)
        .collect())
}
