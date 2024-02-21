use crate::app::components::Container;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
fn PauseIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
      <svg aria-hidden="true" viewBox="0 0 10 10" class=class>
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M1.496 0a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5H2.68a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5H1.496Zm5.82 0a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5H8.5a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5H7.316Z"
        />
      </svg>
    }
}

#[component]
fn PlayIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
      <svg aria-hidden="true" viewBox="0 0 10 10" class=class>
        <path d="M8.25 4.567a.5.5 0 0 1 0 .866l-7.5 4.33A.5.5 0 0 1 0 9.33V.67A.5.5 0 0 1 .75.237l7.5 4.33Z" />
      </svg>
    }
}

#[component]
fn IssueEntry(issue: IssueShort) -> impl IntoView {
    view! {
      <article
        aria-labelledby=format!("issue-{}-title", issue.id)
        class="py-10 sm:py-12"
      >
        <Container>
          <div class="flex flex-col items-start">
            <h2
              id=format!("issue-{}-title", issue.id)
              class="mt-2 text-lg font-bold text-slate-900"
            >
              <a href=format!("/issue/{}", issue.slug)>{&issue.display_name}</a>
            </h2>
            <p class="order-first font-mono text-sm leading-7 text-slate-500">
              {&issue.issue_date.map(|date| date.to_string()).unwrap_or("".to_string())}
            </p>
            <p class="mt-1 text-base leading-7 text-slate-700">
              {issue.description}
            </p>
            <div class="mt-4 flex items-center gap-4">
            //   <EpisodePlayButton
            //     episode={episode}
            //     class="flex items-center gap-x-3 text-sm font-bold leading-6 text-pink-500 hover:text-pink-700 active:text-pink-900"
            //     playing={
            //       <>
            //         <PauseIcon class="h-2.5 w-2.5 fill-current" />
            //         <span aria-hidden="true">Listen</span>
            //       </>
            //     }
            //     paused={
            //       <>
            //         <PlayIcon class="h-2.5 w-2.5 fill-current" />
            //         <span aria-hidden="true">Listen</span>
            //       </>
            //     }
            //   />
              <span
                aria-hidden="true"
                class="text-sm font-bold text-slate-400"
              >
                /
              </span>
              <a
                href=format!("/issue/{}", issue.slug)
                class="flex items-center text-sm font-bold leading-6 text-pink-500 hover:text-pink-700 active:text-pink-900"
                aria-label=format!("full issue for {}", issue.display_name)
              >
                Read issue...
              </a>
            </div>
          </div>
        </Container>
      </article>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let issues =
        create_resource(move || {}, |_| fetch_issues());

    view! {
      <div class="pb-12 pt-16 sm:pb-4 lg:pt-12">
        <Container>
          <h1 class="text-2xl font-bold leading-7 text-slate-900">
            Issues
          </h1>
      </Container>
        <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
        {move || {
          issues.get().map(|data| match data {
            Err(_e) => view! {  <div></div> },
            Ok(issues) => view! {
                <div class="divide-y divide-slate-100 sm:mt-4 lg:mt-8 lg:border-t lg:border-slate-100">
                {
                  issues
                  .into_iter()
                  .map(|issue| view!{
                          <IssueEntry issue={issue} />
                  })
                  .collect::<Vec<_>>()
                }
                </div>
              }
          })}}
          </Suspense>
      </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlIssueShort {
    pub id: Vec<u8>,
    pub slug: String,
    pub issue_date: Option<time::Date>,
    pub display_name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IssueShort {
    pub id: String,
    pub slug: String,
    pub issue_date: Option<time::Date>,
    pub display_name: String,
    pub description: String,
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
            slug: value.slug,
            issue_date: value.issue_date,
            display_name: value.display_name,
            description: value.description,
        }
    }
}

#[server]
pub async fn fetch_issues(
) -> Result<Vec<IssueShort>, ServerFnError> {
    let pool = crate::sql::pool()?;

    let issues: Vec<SqlIssueShort> = sqlx::query_as!(
        SqlIssueShort,
        "SELECT
          id,
          slug,
          issue_date,
          display_name,
          description
FROM issue
ORDER BY status, issue_date DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(issues.into_iter().map(IssueShort::from).collect())
}
