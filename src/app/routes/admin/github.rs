use crate::app::components::Divider;
use crate::app::server_fn::error::NoCustomError;
use leptos::{either::EitherOf3, prelude::*};
use serde::{Deserialize, Serialize};

#[component]
pub fn GitHub() -> impl IntoView {
    let select_new_github_issues: ServerAction<
        SelectNewGithubIssues,
    > = ServerAction::new();
    let select_new_pull_requests: ServerAction<
        SelectNewPullRequests,
    > = ServerAction::new();
    let select_merged_pull_requests: ServerAction<
        SelectMergedPullRequests,
    > = ServerAction::new();
    let issues =
        Resource::new(move || {}, |_| fetch_issues());
    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading (Suspense Fallback)..."</p> }
        }>
            {move || match issues.get() {
                None => EitherOf3::A(view! { <div>error</div> }),
                Some(Err(e)) => EitherOf3::B(view! { <div>{e.to_string()}</div> }),
                Some(Ok(issues)) => {
                    let issues_a = issues.clone();
                    let issues_b = issues.clone();
                    EitherOf3::C(view! {
                        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
                            <Divider title="new github issues"/>

                            <ActionForm
                                action=select_new_github_issues
                                attr:class="isolate -space-y-px rounded-md shadow-sm"
                            >
                                <label
                                    for="issue_id"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    issue_id
                                </label>
                                <select
                                    name="issue_id"
                                    class="mt-2 block w-full text-white bg-slate-800 p-5 rounded-md focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                >
                                    {issues_a
                                        .iter()
                                        .map(|issue| {
                                            view! {
                                                <option value=issue.id.clone()>
                                                    {issue.issue_date.to_string()} - {issue.display_name.clone()}
                                                </option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                                <div class="flex gap-x-4 mt-4">
                                <div>
                                <label
                                    for="start_date"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    Start Date
                                </label>
                                <div class="mt-2">
                                    <input
                                        class="text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        type="date"
                                        id="start_date"
                                        name="start_date"
                                        min="2024-01-01"
                                    />
                                </div>
                                </div>
                                <div>
                                <label
                                    for="end_date"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    End Date
                                </label>
                                <div class="mt-2">
                                    <input
                                        class="text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        type="date"
                                        id="end_date"
                                        name="end_date"
                                        min="2024-01-01"
                                    />
                                </div>
                                </div>
                                </div>
                                <button
                                    class="mt-3 rounded-md bg-sky-600 px-5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                                    type="submit"
                                >Add to Issue</button>
                            </ActionForm>

                            <Divider title="new pull requests"/>

                            <ActionForm
                                action=select_new_pull_requests
                                attr:class="isolate -space-y-px rounded-md shadow-sm"
                            >
                                <label
                                    for="issue_id"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    issue_id
                                </label>
                                <select
                                    name="issue_id"
                                    class="mt-2 block w-full text-white bg-slate-800 p-5 rounded-md focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                >
                                    {issues_b
                                        .clone()
                                        .iter()
                                        .map(|issue| {
                                            view! {
                                                <option value=issue.id.clone()>
                                                    {issue.issue_date.to_string()} - {issue.display_name.clone()}
                                                </option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                                <div class="flex gap-x-4 mt-4">
                                <div>
                                <label
                                    for="start_date"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    Start Date
                                </label>
                                <div class="mt-2">
                                    <input
                                        class="text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        type="date"
                                        id="start_date"
                                        name="start_date"
                                        min="2024-01-01"
                                    />
                                </div>
                                </div>
                                <div>
                                <label
                                    for="end_date"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    End Date
                                </label>
                                <div class="mt-2">
                                    <input
                                        class="text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        type="date"
                                        id="end_date"
                                        name="end_date"
                                        min="2024-01-01"
                                    />
                                </div>
                                </div>
                                </div>
                                <button
                                    class="mt-3 rounded-md bg-sky-600 px-5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                                >Add to Issue</button>
                            </ActionForm>

                            <Divider title="merged pull requests"/>

                            <ActionForm
                                action=select_merged_pull_requests
                                attr:class="isolate -space-y-px rounded-md shadow-sm"
                            >
                                <label
                                    for="issue_id"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    issue_id
                                </label>
                                <select
                                    name="issue_id"
                                    class="mt-2 block w-full text-white bg-slate-800 p-5 rounded-md focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                >

                                    {issues
                                        .iter()
                                        .map(|issue| {
                                            view! {
                                                <option value=issue.id.clone()>
                                                    {issue.issue_date.to_string()} - {issue.display_name.clone()}
                                                </option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                                <div class="flex gap-x-4 mt-4">
                                <div>
                                <label
                                    for="start_date"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    Start Date
                                </label>
                                <div class="mt-2">
                                    <input
                                        class="text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        type="date"
                                        id="start_date"
                                        name="start_date"
                                        min="2024-01-01"
                                    />
                                </div>
                                </div>
                                <div>
                                <label
                                    for="end_date"
                                    class="block text-sm font-medium leading-6 text-slate-50"
                                >
                                    End Date
                                </label>
                                <div class="mt-2">
                                    <input
                                        class="text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        type="date"
                                        id="end_date"
                                        name="end_date"
                                        min="2024-01-01"
                                    />
                                </div>
                                </div>
                                </div>
                                <button
                                    class="mt-3 rounded-md bg-sky-600 px-5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                                type="submit"
                                >Add to Issue</button>
                            </ActionForm>
                        </div>
                    })
                }
            }}

        </Suspense>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlIssueShort {
    id: Vec<u8>,
    display_name: String,
    issue_date: time::Date,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueShort {
    pub id: String,
    pub display_name: String,
    issue_date: time::Date,
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
            issue_date: value.issue_date,
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
        display_name,
        issue_date
FROM issue
WHERE status = "draft"
ORDER BY issue_date DESC"#
    )
    .fetch_all(&pool)
    .await?;

    Ok(issues.into_iter().map(IssueShort::from).collect())
}

#[server]
pub async fn select_new_github_issues(
    issue_id: String,
    start_date: time::Date,
    end_date: time::Date,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    tracing::info!(
        issue_id,
        ?start_date,
        ?end_date,
        "select_new_github_issue"
    );
    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        "INSERT INTO issue__new_github_issue (issue_id, github_issue_id )
SELECT ?, ngi.id
FROM new_github_issue ngi 
WHERE gh_created_at BETWEEN ? AND ?",
        issue_id.as_slice(),
        start_date,
        end_date,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!(?e);
        ServerFnError::<NoCustomError>::ServerError(
            e.to_string(),
        )
    })?;

    Ok(())
}

#[server]
pub async fn select_new_pull_requests(
    issue_id: String,
    start_date: time::Date,
    end_date: time::Date,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        "INSERT INTO issue__new_pull_request (issue_id, pull_request_id )
SELECT ?, ngi.id
FROM new_pull_request ngi 
WHERE gh_created_at BETWEEN ? AND ?",
        issue_id.as_slice(),
        start_date,
        end_date,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!(?e);
        ServerFnError::<NoCustomError>::ServerError(
            e.to_string(),
        )
    })?;

    Ok(())
}
#[server]
pub async fn select_merged_pull_requests(
    issue_id: String,
    start_date: time::Date,
    end_date: time::Date,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let issue_id: [u8; 16] = issue_id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        "INSERT INTO issue__merged_pull_request (issue_id, merged_pull_request_id )
SELECT ?, ngi.id
FROM merged_pull_request ngi 
WHERE merged_at_date BETWEEN ? AND ?",
        issue_id.as_slice(),
        start_date,
        end_date,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!(?e);
        ServerFnError::<NoCustomError>::ServerError(
            e.to_string(),
        )
    })?;

    Ok(())
}
