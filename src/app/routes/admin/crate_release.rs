use crate::app::components::Divider;
use futures::future::join;
use leptos::{either::EitherOf3, prelude::*};
use serde::{Deserialize, Serialize};
pub mod id;

#[server]
async fn add_crate_release(
    title: String,
    url: String,
    discord_url: String,
    description: String,
    posted_date: String,
) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::MySqlPool>()
        .expect("to be able to access app_state");
    let username = crate::sql::with_admin_access()?;

    let id: [u8; 16] = rusty_ulid::generate_ulid_bytes();

    sqlx::query!(
        r#"
    INSERT INTO crate_release ( id, title, url, discord_url, posted_date, description, submitted_by )
    VALUES ( ?, ?, ?, ?, ?, ?, ? )
        "#,
        id.as_slice(),
        title,
        url,
        discord_url,
        posted_date,
        description,
        username.0
    )
    .execute(&pool)
    .await
    .expect("successful insert");
    Ok(())
}

#[component]
pub fn CrateRelease() -> impl IntoView {
    let add_crate_release: ServerAction<AddCrateRelease> =
        ServerAction::new();
    let crate_releases = Resource::new(
        move || {},
        |_| join(fetch_crate_releases(), fetch_issues()),
    );

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <ActionForm attr:class="isolate -space-y-px rounded-md shadow-sm" action=add_crate_release>
                <div class="relative rounded-md rounded-b-none px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                    <label for="title" class="block text-xs font-medium text-gray-900">
                        Title
                    </label>
                    <input
                        required
                        type="text"
                        name="title"
                        id="title"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                        placeholder="Hexagon procedural generation"
                    />
                </div>
                <div class="relative px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                    <label for="url" class="block text-xs font-medium text-gray-900">
                        URL
                    </label>
                    <input
                        required
                        type="text"
                        name="url"
                        id="url"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                        placeholder="https"
                    />
                </div>
                <div class="relative rounded-md rounded-t-none px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                    <label for="discord_url" class="block text-xs font-medium text-gray-900">
                        Discord URL
                    </label>
                    <input
                        type="text"
                        name="discord_url"
                        id="discord_url"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                        placeholder="https"
                    />
                </div>
                <label
                    for="posted_date"
                    class="block text-sm font-medium leading-6 text-gray-900"
                >
                    Posted At
                </label>
                <div class="mt-2">
                    <input type="date" required id="posted_date" name="posted_date" min="2024-01-01"/>
                </div>
                <label
                    for="description"
                    class="block text-sm font-medium leading-6 text-gray-900"
                >
                    Add your description (markdown compatible)
                </label>
                <div class="mt-2">
                    <textarea
                        required
                        rows="4"
                        name="description"
                        id="description"
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    ></textarea>
                </div>
                <button
                    type="submit"
                    class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                    Add CrateRelease
                </button>
            </ActionForm>
            <Divider title="CrateReleases without an issue"/>
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || crate_releases
                    .get()
                    .map(|data| match data {
                        (Err(e), Err(e2)) => {
                            EitherOf3::A(view! {
                                <div>
                                    <div>{e.to_string()}</div>
                                    <div>{e2.to_string()}</div>
                                </div>
                            })
                        }
                        (_, Err(e)) | (Err(e), _) => {
                            EitherOf3::B(view! {
                                <div>
                                    <div>{e.to_string()}</div>
                                </div>
                            })
                        }
                        (Ok(crate_releases), Ok(issues)) => {
                            EitherOf3::C(view! {
                                <div>
                                    <ul role="list" class="divide-y divide-gray-100">
                                        <For
                                            each=move || crate_releases.clone()
                                            key=|crate_release| crate_release.id.clone()
                                            let:crate_release
                                        >
                                            <AddCrateReleaseToIssueForm
                                                crate_release=crate_release
                                                issue_id=issues.first().map(|issue| issue.id.clone())
                                            />
                                        </For>
                                    </ul>
                                </div>
                            })
                        }
                    })}

            </Suspense>
        </div>
    }
}

#[component]
fn AddCrateReleaseToIssueForm(
    crate_release: CrateReleaseData,
    issue_id: Option<String>,
) -> impl IntoView {
    let associate_crate_release_with_issue: ServerAction<
        AssociateCrateReleaseWithIssue,
    > = ServerAction::new();

    view! {
        <li class="flex items-center justify-between gap-x-6 py-5">
            <div class="min-w-0">
                <div class="flex items-start gap-x-3">
                    <p class="text-sm font-semibold leading-6 text-gray-900">
                        {crate_release.title}
                    </p>
                </div>
                <div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
                    <p class="whitespace-nowrap">
                        posted on
                        <time datetime=crate_release
                            .posted_date
                            .as_ref()
                            .unwrap()
                            .to_string()>
                            {crate_release.posted_date.as_ref().unwrap().to_string()}
                        </time>
                    </p>
                    <svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
                        <circle cx="1" cy="1" r="1"></circle>
                    </svg>
                // <p class="truncate">Submitted by {crate_release.submitted_by}</p>
                </div>
            </div>
            {issue_id
                .map(|issue_id| {
                    view! {
                        <div class="flex flex-none items-center gap-x-4">
                            <ActionForm action=associate_crate_release_with_issue>
                                <input
                                    type="hidden"
                                    value=crate_release.id
                                    name="crate_release_id"
                                />
                                <input type="hidden" value=issue_id name="issue_id"/>
                                <button
                                    type="submit"
                                    class="hidden rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:block"
                                >
                                    Add to current draft
                                </button>
                            </ActionForm>
                        </div>
                    }
                })}

        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlCrateReleaseData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CrateReleaseData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
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
        }
    }
}

#[server]
pub async fn fetch_crate_releases(
) -> Result<Vec<CrateReleaseData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let crate_releases: Vec<SqlCrateReleaseData> = sqlx::query_as!(
        SqlCrateReleaseData,
        "SELECT
        id,
        title,
        posted_date
FROM crate_release
LEFT JOIN issue__crate_release
  ON crate_release.id = issue__crate_release.crate_release_id
WHERE issue__crate_release.issue_id IS NULL
ORDER BY crate_release.id"
    )
    .fetch_all(&pool)
    .await?;

    Ok(crate_releases
        .into_iter()
        .map(CrateReleaseData::from)
        .collect())
}

#[server]
async fn associate_crate_release_with_issue(
    crate_release_id: String,
    issue_id: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let crate_release_id: [u8; 16] = crate_release_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        r#"
    INSERT INTO issue__crate_release ( issue_id, crate_release_id )
    VALUES ( ?, ? )
        "#,
        issue_id.as_slice(),
        crate_release_id.as_slice()
    )
    .execute(&pool)
    .await
    .expect("successful insert");
    Ok(())
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlIssueShort {
    id: Vec<u8>,
    display_name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueShort {
    pub id: String,
    pub display_name: String,
}

#[cfg(feature = "ssr")]
impl From<SqlIssueShort> for IssueShort {
    fn from(value: SqlIssueShort) -> Self {
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        IssueShort {
            id: id_str.to_string(),
            display_name: value.display_name,
        }
    }
}

#[server]
async fn fetch_issues(
) -> Result<Vec<IssueShort>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issues: Vec<SqlIssueShort> = sqlx::query_as!(
        SqlIssueShort,
        r#"SELECT
        id,
        display_name
FROM issue
WHERE status = "draft"
ORDER BY issue_date DESC"#
    )
    .fetch_all(&pool)
    .await?;

    Ok(issues.into_iter().map(IssueShort::from).collect())
}
