use crate::app::components::Divider;
use futures::future::join;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
pub mod id;

#[server]
async fn add_educational(
    title: String,
    video_url: String,
    post_url: String,
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
    INSERT INTO educational ( id, title, video_url, post_url, discord_url, posted_date, description, submitted_by )
    VALUES ( ?, ?, ?, ?, ?, ?, ?, ? )
        "#,
        id.as_slice(),
        title,
        video_url,
        post_url,
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
pub fn Educational() -> impl IntoView {
    let add_educational = create_server_action::<AddEducational>();
    let educationals = create_resource(
        move || {},
        |_| join(fetch_educationals(), fetch_issues()),
    );

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <ActionForm class="isolate -space-y-px rounded-md shadow-sm" action=add_educational>
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
                    <label for="video_url" class="block text-xs font-medium text-gray-900">
                        Video URL
                    </label>
                    <input
                        required
                        type="text"
                        name="video_url"
                        id="video_url"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                        placeholder="https://www.youtube.com/watch?v=Jcw_v1w7dbI"
                    />
                </div>
                <div class="relative px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                    <label for="post_url" class="block text-xs font-medium text-gray-900">
                        Post URL
                    </label>
                    <input
                        required
                        type="text"
                        name="post_url"
                        id="post_url"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                        placeholder="https://www.nikl.me/blog/2024/bevy_ecs_as_data_layer_in_leptos_ssg/"
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
                    required
                    for="posted_date"
                    class="block text-sm font-medium leading-6 text-gray-900"
                >
                    Posted At
                </label>
                <div class="mt-2">
                    <input type="date" id="posted_date" name="posted_date" min="2024-01-01"/>
                </div>
                <label
                    required
                    for="description"
                    class="block text-sm font-medium leading-6 text-gray-900"
                >
                    Add your description (markdown compatible)
                </label>
                <div class="mt-2">
                    <textarea
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
                    Add Educational
                </button>
            </ActionForm>
            <Divider title="Educationals without an issue"/>
            <Suspense fallback=move || {
                view! { <p>"Loading (Suspense Fallback)..."</p> }
            }>
                {educationals
                    .get()
                    .map(|data| match data {
                        (Err(e), Err(e2)) => {
                            view! {
                                <div>
                                    <div>{e.to_string()}</div>
                                    <div>{e2.to_string()}</div>
                                </div>
                            }
                        }
                        (_, Err(e)) | (Err(e), _) => {
                            view! {
                                <div>
                                    <div>{e.to_string()}</div>
                                </div>
                            }
                        }
                        (Ok(educationals), Ok(issues)) => {
                            view! {
                                <div>
                                    <ul role="list" class="divide-y divide-gray-100">
                                        <For
                                            each=move || educationals.clone()
                                            key=|educational| educational.id.clone()
                                            let:educational
                                        >
                                            <AddEducationalToIssueForm
                                                educational=educational
                                                issue_id=issues.first().map(|issue| issue.id.clone())
                                            />
                                        </For>
                                    </ul>
                                </div>
                            }
                        }
                    })}

            </Suspense>
        </div>
    }
}

#[component]
fn AddEducationalToIssueForm(
    educational: EducationalData,
    issue_id: Option<String>,
) -> impl IntoView {
    let associate_educational_with_issue =
        create_server_action::<AssociateEducationalWithIssue>();

    view! {
        <li class="flex items-center justify-between gap-x-6 py-5">
            <div class="min-w-0">
                <div class="flex items-start gap-x-3">
                    <p class="text-sm font-semibold leading-6 text-gray-900">{educational.title}</p>
                </div>
                <div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
                    <p class="whitespace-nowrap">
                        posted on
                        <time datetime=educational
                            .posted_date
                            .as_ref()
                            .unwrap()
                            .to_string()>
                            {educational.posted_date.as_ref().unwrap().to_string()}
                        </time>
                    </p>
                    <svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
                        <circle cx="1" cy="1" r="1"></circle>
                    </svg>
                // <p class="truncate">Submitted by {educational.submitted_by}</p>
                </div>
            </div>
            {issue_id
                .map(|issue_id| {
                    view! {
                        <div class="flex flex-none items-center gap-x-4">
                            <ActionForm action=associate_educational_with_issue>
                                <input type="hidden" value=educational.id name="educational_id"/>
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
struct SqlEducationalData {
    id: Vec<u8>,
    title: String,
    posted_date: Option<time::Date>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EducationalData {
    pub id: String,
    pub title: String,
    pub posted_date: Option<time::Date>,
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
        }
    }
}

#[server]
pub async fn fetch_educationals(
) -> Result<Vec<EducationalData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let educationals: Vec<SqlEducationalData> = sqlx::query_as!(
        SqlEducationalData,
        "SELECT
        id,
        title,
        posted_date
FROM educational
LEFT JOIN issue__educational
  ON educational.id = issue__educational.educational_id
WHERE issue__educational.issue_id IS NULL
ORDER BY educational.id"
    )
    .fetch_all(&pool)
    .await?;

    Ok(educationals.into_iter().map(EducationalData::from).collect())
}

#[server]
async fn associate_educational_with_issue(
    educational_id: String,
    issue_id: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    let educational_id: [u8; 16] = educational_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        r#"
    INSERT INTO issue__educational ( issue_id, educational_id )
    VALUES ( ?, ? )
        "#,
        issue_id.as_slice(),
        educational_id.as_slice()
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
