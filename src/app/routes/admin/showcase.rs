use crate::app::components::Divider;
use futures::future::join;
use leptos::{either::EitherOf3, prelude::*};
use serde::{Deserialize, Serialize};
pub mod id;

#[server]
async fn add_showcase(
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
    INSERT INTO showcase ( id, title, url, discord_url, posted_date, description, submitted_by )
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
pub fn Showcase() -> impl IntoView {
    let add_showcase: ServerAction<AddShowcase> =
        ServerAction::new();
    let showcases = Resource::new(
        move || {},
        |_| join(fetch_showcases(), fetch_issues()),
    );

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <ActionForm attr:class="isolate -space-y-px rounded-md shadow-sm" action=add_showcase>
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
                    Add Showcase
                </button>
            </ActionForm>
            <Divider title="Showcases without an issue"/>
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {move || showcases
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
                        (Ok(showcases), Ok(issues)) => {
                            EitherOf3::C(view! {
                                <div>
                                    <ul role="list" class="divide-y divide-gray-100">
                                        <For
                                            each=move || showcases.clone()
                                            key=|showcase| showcase.id.clone()
                                            let:showcase
                                        >
                                            <AddShowcaseToIssueForm
                                                showcase=showcase
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
fn AddShowcaseToIssueForm(
    showcase: ShowcaseData,
    issue_id: Option<String>,
) -> impl IntoView {
    let associate_showcase_with_issue: ServerAction<
        AssociateShowcaseWithIssue,
    > = ServerAction::new();

    view! {
        <li class="flex items-center justify-between gap-x-6 py-5">
            <div class="min-w-0">
                <div class="flex items-start gap-x-3">
                    <p class="text-sm font-semibold leading-6 text-gray-900">{showcase.title}</p>
                </div>
                <div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
                    <p class="whitespace-nowrap">
                        posted on
                        <time datetime=showcase
                            .posted_date
                            .as_ref()
                            .unwrap()
                            .to_string()>{showcase.posted_date.as_ref().unwrap().to_string()}</time>
                    </p>
                    <svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
                        <circle cx="1" cy="1" r="1"></circle>
                    </svg>
                // <p class="truncate">Submitted by {showcase.submitted_by}</p>
                </div>
            </div>
            {issue_id
                .map(|issue_id| {
                    view! {
                        <div class="flex flex-none items-center gap-x-4">
                            <ActionForm action=associate_showcase_with_issue>
                                <input type="hidden" value=showcase.id name="showcase_id"/>
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
struct SqlShowcaseData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ShowcaseData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
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
        }
    }
}

#[server]
pub async fn fetch_showcases(
) -> Result<Vec<ShowcaseData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let showcases: Vec<SqlShowcaseData> = sqlx::query_as!(
        SqlShowcaseData,
        "SELECT
        id,
        title,
        posted_date
FROM showcase
LEFT JOIN issue__showcase
  ON showcase.id = issue__showcase.showcase_id
WHERE issue__showcase.issue_id IS NULL
ORDER BY showcase.id"
    )
    .fetch_all(&pool)
    .await?;

    Ok(showcases
        .into_iter()
        .map(ShowcaseData::from)
        .collect())
}

#[server]
async fn associate_showcase_with_issue(
    showcase_id: String,
    issue_id: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let showcase_id: [u8; 16] = showcase_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        r#"
    INSERT INTO issue__showcase ( issue_id, showcase_id )
    VALUES ( ?, ? )
        "#,
        issue_id.as_slice(),
        showcase_id.as_slice()
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
