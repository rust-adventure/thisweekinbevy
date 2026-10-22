use crate::app::components::Divider;
use itertools::Itertools;
use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use time::Time;

#[component]
pub fn Issue() -> impl IntoView {
    let params = use_params_map();
    let issue = Resource::new(
        move || {
            params.with(|p| p.get("id").unwrap_or_default())
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
                            Err(e) => Either::Left(view! { <pre>{e.to_string()}</pre> }),
                            Ok(issue) => {
                                Either::Right(issue
                                    .map(|issue| {
                                        view! { <IssueForm issue=issue/> }
                                    })
                                    .collect_view())
                            }
                        })
                }}

            <Divider title="Showcases"/>
            <Showcases/>
            <Divider title="Crate Releases"/>

            {move || {
                    issue
                        .get()
                        .map(|data| match data {
                            Err(e) => Either::Left(view! { <pre>{e.to_string()}</pre> }),
                            Ok(issue) => {
                                Either::Right(issue
                                    .map(|issue| {
                                        view! {
                                            <CrateReleases issue_date={issue.issue_date}/>
                                        }
                                    })
                                    .collect_view())
                            }
                        })
                }}
            <Divider title="Devlogs"/>
            <Devlogs/>
            <Divider title="Educationals"/>
            <Educationals/>

            </Suspense>
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
    let update_issue_metadata: ServerAction<
        UpdateIssueMetadata,
    > = ServerAction::new();
    view! {
        <div class="isolate px-6 py-24 sm:py-32 lg:px-8">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold tracking-tight text-slate-50 sm:text-4xl">
                    {issue.issue_date.to_string()}
                </h2>
                <p class="mt-2 text-lg leading-8 text-slate-400">{issue.status}</p>
            </div>
            <ActionForm action=update_issue_metadata>
                <input type="hidden" name="issue_id" id="issue_id" value=issue.id/>
                <div class="grid grid-cols-1 gap-x-8 gap-y-6 sm:grid-cols-2">
                    <div>
                        <label
                            for="display_name"
                            class="block text-sm font-semibold leading-6 text-slate-50"
                        >
                            display_name
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="text"
                                name="display_name"
                                id="display_name"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-slate-50 shadow-sm placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-sky-600 sm:text-sm sm:leading-6 focus:outline-none bg-slate-800"
                                value=issue.display_name
                            />
                        </div>
                    </div>
                    <div>
                        <label
                            for="slug"
                            class="block text-sm font-semibold leading-6 text-slate-50"
                        >
                            slug
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="text"
                                name="slug"
                                id="slug"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-slate-50 shadow-sm placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-sky-600 sm:text-sm sm:leading-6 focus:outline-none bg-slate-800"
                                value=issue.slug
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <label
                            for="cloudinary_public_id"
                            class="block text-sm font-semibold leading-6 text-slate-50"
                        >
                            cloudinary_public_id
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="text"
                                name="cloudinary_public_id"
                                id="cloudinary_public_id"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-slate-50 shadow-sm placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-sky-600 sm:text-sm sm:leading-6 focus:outline-none bg-slate-800"
                                value=issue.cloudinary_public_id
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <label
                            for="youtube_id"
                            class="block text-sm font-semibold leading-6 text-slate-50"
                        >
                            youtube_id
                        </label>
                        <div class="mt-2.5">
                            <input
                                type="youtube_id"
                                name="youtube_id"
                                id="youtube_id"
                                class="block w-full rounded-md border-0 px-3.5 py-2 text-slate-50 shadow-sm placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-sky-600 sm:text-sm sm:leading-6 focus:outline-none bg-slate-800"
                                value=issue.youtube_id
                            />
                        </div>
                    </div>

                    <div class="sm:col-span-2">
                        <label
                            for="description"
                            class="block text-sm font-semibold leading-6 text-slate-50"
                        >
                            description
                        </label>
                        <div class="mt-2.5">
                            <textarea
                                name="description"
                                id="description"
                                rows="4"
                                class="field-sizing-content block w-full rounded-md border-0 px-3.5 py-2 text-slate-50 shadow-sm placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-sky-600 sm:text-sm sm:leading-6 focus:outline-none bg-slate-800"
                            >
                                {issue.description}
                            </textarea>
                        </div>
                    </div>
                </div>
                <div class="mt-10">
                    <button
                        type="submit"
                        class="block w-full rounded-md bg-sky-600 px-3.5 py-2.5 text-center text-sm font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
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

    let showcases = Resource::new(
        move || {
            params.with(|p| p.get("id").unwrap_or_default())
        },
        fetch_showcases_for_issue_id,
    );

    view! {
        <ul
            role="list"
            class="divide-y divide-slate-900 overflow-hidden bg-slate-800 shadow-sm ring-1 ring-slate-900/5 sm:rounded-xl"
        >
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || {
                    showcases
                        .get()
                        .map(|data| match data {
                            Err(e) => Either::Left(view! { <pre>{e.to_string()}</pre> }),
                            Ok(showcases) => {
                                Either::Right(showcases
                                    .iter()
                                    .map(|showcase| {
                                        view! { <ShowcaseLi showcase=showcase.clone()/> }
                                    })
                                    .collect_view())
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
        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-slate-900 sm:px-6">
            <div class="flex min-w-0 gap-x-4">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-slate-50">
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
                    <p class="text-sm leading-6 text-slate-50">{showcase.image_count} images</p>
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
fn CrateReleases(issue_date: time::Date) -> impl IntoView {
    let params = use_params_map();

    let crate_releases = Resource::new(
        move || {
            params.with(|p| p.get("id").unwrap_or_default())
        },
        fetch_crate_releases_for_issue_id,
    );
    let crates_io = Resource::new(
        move || issue_date.clone(),
        fetch_crates_io_for_issue_date,
    );

    view! {
        <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
            <ul
                role="list"
                class="divide-y divide-slate-900 overflow-hidden bg-slate-800 shadow-sm ring-1 ring-slate-900/5 sm:rounded-xl"
            >
                {move || {
                    crate_releases
                        .get()
                        .map(|data| match data {
                            Err(e) => Either::Left(view! { <pre>{e.to_string()}</pre> }),
                            Ok(crate_releases) => {
                                Either::Right(crate_releases
                                    .iter()
                                    .map(|crate_release| {
                                        view! {
                                            <CrateReleaseLi crate_release=crate_release.clone()/>
                                        }
                                    })
                                    .collect_view())
                            }
                        })
                }}

                </ul>
                <ul>
                {move || {
                    let Some(Ok(items)) = crates_io.get() else {
                        return None;
                    };
                    Some(items.into_iter().filter_map(|possible_release| {
                        let name = possible_release.display_name?;
                        let crate_id = possible_release.crate_id?;
                        Some((name, crate_id, possible_release.publishes))
                    }).map(|(name, crate_id, publishes)| {
                        view!{
                            <li>
                            <p class="text-slate-50">{name} -- {publishes.len().to_string()} crates</p>
                            </li>
                        }
                    }).collect_view())
                }}
                </ul>
                <ul>
                    {move || {
                        let Some(Ok(items)) = crates_io.get() else {
                            return None;
                        };
                        let Some(data) = items.into_iter().find(|v| v.crate_id.is_none()) else {
                            return None;
                        };

                        Some(data.publishes
                            .iter()
                            .sorted_by_key(|a| &a.crate_name)
                            .map(|crate_info| {
                                view! {
                                    <li class="text-slate-50">
                                    <a
                                        href=format!("https://docs.rs/{crate_name}/{version_string}/{crate_name_underscores}/",
                                            crate_name=crate_info.crate_name,
                                            crate_name_underscores=crate_info.crate_name.replace("-", "_"),
                                            version_string=crate_info.version_string
                                        )
                                        class="text-sky-400 hover:text-sky-500 font-bold"
                                    >{crate_info.crate_name.clone()}</a>
                                        - {crate_info.version_string.clone()} - {crate_info.pubtime.to_string()}
                                    </li>
                                }
                            })
                            .collect_view())

                    }}
                </ul>
        </Suspense>
    }
}

#[component]
fn CrateReleaseLi(
    crate_release: CrateReleaseData,
) -> impl IntoView {
    view! {
        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-slate-900 sm:px-6">
            <div class="flex min-w-0 gap-x-4">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-slate-50">
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
                    <p class="text-sm leading-6 text-slate-50">
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

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone)]
pub struct SqlPossibleCrateReleases {
    pub crate_id: Option<Vec<u8>>,
    pub display_name: Option<String>,
    pub publishes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PossibleCrateReleases {
    pub crate_id: Option<String>,
    pub display_name: Option<String>,
    pub publishes: Vec<CratesIoData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CratesIoData {
    pub crate_name: String,
    pub version_string: String,
    // #[serde(with = "time::serde::iso8601")]
    pub pubtime: String,
}

#[cfg(feature = "ssr")]
impl From<SqlPossibleCrateReleases>
    for PossibleCrateReleases
{
    fn from(value: SqlPossibleCrateReleases) -> Self {
        let id_str = value.crate_id.map(|id| {
            rusty_ulid::Ulid::try_from(id.as_slice())
                .expect(
                    "expect valid ids from the database",
                )
                .to_string()
        });
        let publishes = value
            .publishes
            .and_then(|publish| {
                tracing::info!(?publish);
                serde_json::from_value::<Vec<CratesIoData>>(
                    publish,
                )
                .inspect_err(|err| {
                    tracing::warn!(?err);
                })
                .ok()
            })
            .unwrap_or(vec![]);
        tracing::info!(?publishes);
        PossibleCrateReleases {
            crate_id: id_str,
            display_name: value.display_name,
            publishes,
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

#[server]
pub async fn fetch_crates_io_for_issue_date(
    issue_date: time::Date,
) -> Result<Vec<PossibleCrateReleases>, ServerFnError> {
    use time::OffsetDateTime;

    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;
    let offset_date_time = OffsetDateTime::new_utc(
        issue_date,
        Time::from_hms(0, 0, 0)?,
    );

    let crates_io_data: Vec<SqlPossibleCrateReleases> =
        sqlx::query_as!(
            SqlPossibleCrateReleases,
            "SELECT 
    crate__associated_crates.crate_id,
    crate.display_name,
    JSON_ARRAYAGG(JSON_OBJECT(
					'crate_name',
                    crates_io.crate_name,
                    'version_string',
                    version_string,
                    'pubtime',
                    pubtime)) AS publishes
FROM
    crates_io
        LEFT JOIN
    crate__associated_crates ON crates_io.crate_name = crate__associated_crates.crate_name
    LEFT JOIN crate on crate.crate_id = crate__associated_crates.crate_id
WHERE
    crates_io.crate_name NOT IN (SELECT 
            crate_name
        FROM
            thisweekinbevy.crates_io_filters
        WHERE
            exclude = 1)
        AND pubtime BETWEEN DATE_SUB(DATE(?),
        INTERVAL 8 DAY) AND DATE_SUB(DATE(?),
        INTERVAL 2 DAY)
GROUP BY crate_id",
    //         "SELECT
    //         crate_name,
    //         version_string,
    //         pubtime
    // FROM crates_io
    // WHERE crate_name NOT IN (
    //     SELECT crate_name FROM thisweekinbevy.crates_io_filters where exclude = 1
    //     )
    //     AND
    //     pubtime BETWEEN DATE_SUB(DATE(?), INTERVAL 8 DAY) AND DATE_SUB(DATE(?), INTERVAL 2 DAY)
    // ORDER BY pubtime ASC",
            Some(offset_date_time),
            Some(offset_date_time),
        )
        .fetch_all(&pool)
        .await?;

    Ok(crates_io_data
        .into_iter()
        .map(PossibleCrateReleases::from)
        .collect())
}

// devlogs

#[component]
fn Devlogs() -> impl IntoView {
    let params = use_params_map();

    let devlogs = Resource::new(
        move || {
            params.with(|p| p.get("id").unwrap_or_default())
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
                            Err(e) => Either::Left(view! { <pre>{e.to_string()}</pre> }),
                            Ok(devlogs) => {
                                Either::Right(devlogs
                                    .iter()
                                    .map(|devlog| {
                                        view! { <DevlogLi devlog=devlog.clone()/> }
                                    })
                                    .collect_view())
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
                    <p class="text-sm font-semibold leading-6 text-slate-50">
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
                    <p class="text-sm leading-6 text-slate-50">{devlog.image_count} images</p>
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

    let educationals = Resource::new(
        move || {
            params.with(|p| p.get("id").unwrap_or_default())
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
                            Err(e) => Either::Left(view! { <pre>{e.to_string()}</pre> }),
                            Ok(educationals) => {
                                Either::Right(educationals
                                    .iter()
                                    .map(|educational| {
                                        view! { <EducationalLi educational=educational.clone()/> }
                                    })
                                    .collect_view())
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
                    <p class="text-sm font-semibold leading-6 text-slate-50">
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
                    <p class="text-sm leading-6 text-slate-50">{educational.image_count} images</p>
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
