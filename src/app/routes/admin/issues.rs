use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Issues() -> impl IntoView {
    let create_draft_issue =
        create_server_action::<CreateDraftIssue>();
    let issues =
        create_resource(move || {}, |_| fetch_issues());
    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <div class="bg-white shadow sm:rounded-lg">
            <div class="px-4 py-5 sm:p-6">
            <h3 class="text-base font-semibold leading-6 text-gray-900">Create New Issue</h3>
            <div class="mt-2 max-w-xl text-sm text-gray-500">
                <p>Creates a new draft issue that content can be attached to. Pick a Monday for the issue date.</p>
            </div>
            <ActionForm
              class="mt-5 sm:flex sm:items-center"
              action=create_draft_issue
            >
                <div class="w-full sm:max-w-xs">
                <label for="issue_date" class="sr-only">issue_date</label>
                <input type="date" name="issue_date" id="issue_date" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
                <button type="submit" class="mt-3 inline-flex w-full items-center justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto">Save</button>
            </ActionForm>
            </div>
        </div>
            <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
            <ul role="list" class="divide-y divide-gray-100">
            {move || {
          issues.read().map(|data| match data {
            Err(e) => view! {  <pre>{e.to_string()}</pre> }.into_view(),
            Ok(issues) => issues
                .iter()
                .map(|issue| {
                    view! {
                        <li class="flex gap-x-4 py-5">
                            <div class="flex-none rounded-full p-1 text-green-400 bg-green-400/10">
                                <div class="h-2 w-2 rounded-full bg-current"></div>
                            </div>
                          <div class="flex-auto">
                            <div class="flex items-baseline justify-between gap-x-4">
                              <a
                                href=format!("/admin/issue/{}", &issue.id)
                                class="text-sm font-semibold leading-6 text-gray-900"
                              >{&issue.display_name}</a>
                              <p class="flex-none text-xs text-gray-600">
                                // <time datetime={&issue.issue_date}>{&issue.issue_date}</time>
                              </p>
                            </div>
                            <p class="mt-1 line-clamp-2 text-sm leading-6 text-gray-600">"trimmed description without markdown render"</p>
                          </div>
                        </li>
                    }
                })
                .collect_view(),
          })
        }

      }  </ul>
    </Suspense>
        </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlIssueShort {
    id: Vec<u8>,
    display_name: String,
    status: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueShort {
    pub id: String,
    pub display_name: String,
    pub status: String,
}

#[cfg(feature = "ssr")]
impl From<SqlIssueShort> for IssueShort {
    fn from(value: SqlIssueShort) -> Self {
        // let id: [u8; 16] =
        // rusty_ulid::generate_ulid_bytes();
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        IssueShort {
            id: id_str.to_string(),
            display_name: value.display_name,
            status: value.status,
        }
    }
}

#[server]
pub async fn fetch_issues(
) -> Result<Vec<IssueShort>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let username = crate::sql::with_admin_access()?;

    let issues: Vec<SqlIssueShort> = sqlx::query_as!(
        SqlIssueShort,
        "SELECT
        id,
        display_name,
        status
FROM issue
ORDER BY status, issue_date DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(issues.into_iter().map(IssueShort::from).collect())
}

#[server]
pub async fn create_draft_issue(
    issue_date: String,
) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::MySqlPool>()
        .expect("to be able to access app_state");
    let username = crate::sql::with_admin_access()?;

    // https://res.cloudinary.com/dilgcuzda/image/upload/v1708310121/

    let id: [u8; 16] = rusty_ulid::generate_ulid_bytes();

    let slug = format!("{issue_date}-todo");
    // default id for opengraph image
    let cloudinary_public_id = "thisweekinbevy/this-week-in-bevyopengraph-light_zwqzqz.avif";
    let display_name = format!("Draft for {issue_date}");

    sqlx::query!(
        r#"
    INSERT INTO issue ( id, issue_date, slug, cloudinary_public_id, display_name )
    VALUES ( ?, ?, ?, ?, ? )
        "#,
        id.as_slice(),
        issue_date,
        slug,
        cloudinary_public_id,
        display_name
    )
    .execute(&pool)
    .await
    .expect("successful insert");
    Ok(())
}
