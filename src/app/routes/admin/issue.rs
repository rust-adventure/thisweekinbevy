use leptos::*;
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
            <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
            {move || {
          issue.read().map(|data| match data {
            Err(e) => view! {  <pre>{e.to_string()}</pre> }.into_view(),
            Ok(issue) => issue
                .map(|issue| {
                    view! {
                        <IssueForm issue=issue />
                    }
                })
                .collect_view(),
          })
        }

      }
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
        // let id: [u8; 16] =
        // rusty_ulid::generate_ulid_bytes();
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
    let username = crate::sql::with_admin_access()?;

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

    let username = crate::sql::with_admin_access()?;

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

// #[server]
// pub async fn create_draft_issue(
//     issue_date: String,
// ) -> Result<(), ServerFnError> {
//     let pool =
// use_context::<sqlx::MySqlPool>().expect("to be
// able to access app_state");     let username =
// use_context::<Option<crate::Username>>().
// expect("a user to be logged in").expect("a
// username to be accessible");

//     // https://res.cloudinary.com/dilgcuzda/image/upload/v1708310121/

//     let id: [u8; 16] =
//     rusty_ulid::generate_ulid_bytes();

//     let slug = format!("{issue_date}-todo");
//     // default id for opengraph image
//     let cloudinary_public_id =
// "thisweekinbevy/
// this-week-in-bevyopengraph-light_zwqzqz.avif";
//     let display_name = format!("Draft for
// {issue_date}");

//     sqlx::query!(
//         r#"
//     INSERT INTO issue ( id, issue_date, slug,
// cloudinary_public_id, display_name )     VALUES
// ( ?, ?, ?, ?, ? )         "#,
//         id.as_slice(),
//         issue_date,
//         slug,
//         cloudinary_public_id,
//         display_name
//     )
//     .execute(&pool)
//     .await
//     .expect("successful insert");
//     Ok(())
// }

#[component]
fn IssueForm(issue: IssueData) -> impl IntoView {
    let update_issue_metadata =
        create_server_action::<UpdateIssueMetadata>();
    view! {
            <div class="isolate bg-white px-6 py-24 sm:py-32 lg:px-8">
      <div class="absolute inset-x-0 top-[-10rem] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[-20rem]" aria-hidden="true">
        <div class="relative left-1/2 -z-10 aspect-[1155/678] w-[36.125rem] max-w-none -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-40rem)] sm:w-[72.1875rem]" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
      </div>
      <div class="mx-auto max-w-2xl text-center">
        <h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">{&issue.issue_date.to_string()}</h2>
        <p class="mt-2 text-lg leading-8 text-gray-600">{&issue.status}</p>
      </div>
      <ActionForm
        class="mx-auto mt-16 max-w-xl sm:mt-20"
        action=update_issue_metadata
      >
        <input
          type="hidden"
          name="issue_id"
          id="issue_id"
          value=&issue.id
        />
        <div class="grid grid-cols-1 gap-x-8 gap-y-6 sm:grid-cols-2">
          <div>
            <label for="display_name" class="block text-sm font-semibold leading-6 text-gray-900">display_name</label>
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
            <label for="slug" class="block text-sm font-semibold leading-6 text-gray-900">slug</label>
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
            <label for="cloudinary_public_id" class="block text-sm font-semibold leading-6 text-gray-900">cloudinary_public_id</label>
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
            <label for="youtube_id" class="block text-sm font-semibold leading-6 text-gray-900">youtube_id</label>
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
            <label for="description" class="block text-sm font-semibold leading-6 text-gray-900">description</label>
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
          <button type="submit" class="block w-full rounded-md bg-indigo-600 px-3.5 py-2.5 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Save</button>
        </div>
      </ActionForm>
    </div>
        }
}
