use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Params, PartialEq, Default, Clone)]
struct Query {
    crate_filter: Option<String>,
    release_filter: Option<String>,
}

#[component]
pub fn Crates() -> impl IntoView {
    let crates_io = Resource::new(
        move || {},
        |_| fetch_crates_io_data(),
    );
    let crates_rows =
        Resource::new(move || {}, |_| fetch_crate_rows());

    let query = leptos_router::hooks::use_query::<Query>();

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8 mt-4">
            <div class="md:flex md:items-center md:justify-between">
                <div class="min-w-0 flex-1">
                    <h2 class="text-2xl/7 font-bold text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight dark:text-white">Crates</h2>
                </div>
                <div class="mt-4 flex md:mt-0 md:ml-4">
                    <button type="button" class="inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-xs inset-ring inset-ring-gray-300 hover:bg-gray-50 dark:bg-white/10 dark:text-white dark:shadow-none dark:inset-ring-white/5 dark:hover:bg-white/20">Edit</button>
                    <button type="button" class="ml-3 inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:bg-indigo-500 dark:shadow-none dark:hover:bg-indigo-400 dark:focus-visible:outline-indigo-400">Publish</button>
                </div>
            </div>

            <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
                <div class="grid grid-cols-2 gap-4 mt-8">
                    <div class="col-span-1">
                     {move || Suspend::new(async move {
                            let data = crates_rows.await;
                            let filter = query.get().unwrap_or_default().crate_filter;
                            data.map(|data| {
                                view! { <TestCratesRows rows=data filter={filter} /> }
                            })
                        })}
                    </div>
                    <div class="col-span-1">
                        {move || Suspend::new(async move {
                            let data = crates_io.await;
                            let filter = query.get().unwrap_or_default().release_filter;
                            data.map(|data| {
                                view! { <TestCratesIo crates_io=data filter={filter} /> }
                            })
                        })}
                    </div>
                </div>
            </Suspense>
        </div>
    }
}

#[component]
pub fn TestCratesRows(
    rows: Vec<CrateInfoRow>,
    filter: Option<String>,
) -> impl IntoView {
    let update_crate_details: ServerAction<
        UpdateCrateDetails,
    > = ServerAction::new();

    dbg!(&filter);

    let rows = rows
        .iter()
        .filter(|v| match &filter {
            Some(filter) => {
                v.display_name.contains(filter)
                    || v.associated_crates.iter().any(
                        |ass_crate| {
                            ass_crate.contains(filter)
                        },
                    )
            }
            None => true,
        })
        .cloned()
        .collect::<Vec<CrateInfoRow>>();
    view! {
        <div class="flex flex-col gap-y-2">
            <For
                each=move || rows.clone()
                key=|crate_info| crate_info.crate_id.clone()
                children=move |info: CrateInfoRow| {
                    view! {
                        <div class="lg:col-start-3 lg:row-end-1">
                            <details><summary class="rounded-lg text-slate-50 px-6 py-3 bg-gray-50 shadow-xs outline-1 outline-gray-900/5 dark:bg-gray-800/50 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">{info.display_name.clone()}</summary>
                            <div class="rounded-lg bg-gray-50 shadow-xs outline-1 outline-gray-900/5 dark:bg-gray-800/50 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">
                                <dl class="flex flex-wrap">
                                <div class="flex-auto p-6">
                                    <ActionForm action=update_crate_details attr:class="flex gap-x-4">
                                        <input
                                            required
                                            class="hidden"
                                            name="id"
                                            value={info.crate_id.clone()}
                                        />
                                        <div class="w-full">
                                        <label for="display_name" class="block text-xs font-medium text-slate-50">
                                            Display Name
                                        </label>
                                        <input
                                            required
                                            type="text"
                                            name="display_name"
                                            value={info.display_name.clone()}
                                            class="w-full text-white bg-slate-800 p-5 rounded focus:ring-2 focus:ring-inset focus:ring-sky-600 focus:outline-none"
                                        />
                                        <label
                                            for="description"
                                            class="block text-sm font-medium leading-6 text-slate-50 mt-5"
                                        >
                                            Description
                                        </label>
                                        <textarea
                                            required
                                            rows="4"
                                            name="description"
                                            class="p-5 block w-full rounded-md border-0 text-slate-50 shadow-sm ring-0 ring-inset bg-slate-800 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-sky-600 sm:text-sm sm:leading-6 focus:outline-none"
                                            >
                                            {info.description.unwrap_or("".to_string())}
                                        </textarea>
                                        </div>
                                        <button
                                            type="submit"
                                            class="block rounded-md bg-sky-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-xs hover:bg-sky-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600 dark:bg-sky-500 dark:hover:bg-sky-400 dark:focus-visible:outline-sky-500"
                                        >Update</button>
                                    </ActionForm>
                                </div>
                                <For
                                    each=move || info.associated_crates.clone()
                                    key=|crate_name| crate_name.clone()
                                    children=move |crate_name: String| {
                                        view! {
                                            <div class="flex w-full flex-none gap-x-4 px-6 pt-6">
                                                <dt class="flex-none">

                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" data-slot="icon" aria-hidden="true" class="h-6 w-5 text-gray-400">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9" />
                                                </svg>

                                                </dt>
                                                <dd class="text-sm/6 font-medium text-gray-900 dark:text-white">{crate_name}</dd>
                                            </div>
                                        }
                                    }
                                />
                                </dl>
                                <div class="mt-6 border-t border-gray-900/5 px-6 py-6 dark:border-white/5">
                                <a href="#" class="text-sm/6 font-semibold text-gray-900 hover:text-gray-700 dark:text-white dark:hover:text-gray-300">Do something?</a>
                                </div>
                            </div>
                            </details>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[island]
pub fn TestCratesIo(
    crates_io: Vec<CrateInfo>,
    filter: Option<String>,
) -> impl IntoView {
    let exclude_crate: ServerAction<ExcludeCrate> =
        ServerAction::new();
    let handle_crates: ServerAction<HandleCrates> =
        ServerAction::new();

    let crates_io_filtered = crates_io
        .iter()
        .filter(|v| match &filter {
            Some(filter) => v.crate_name.contains(filter),
            None => true,
        })
        .cloned()
        .collect::<Vec<CrateInfo>>();

    view! {
    <div class="px-4 sm:px-6 lg:px-8">
      <div class="sm:flex sm:items-center">
        <div class="sm:flex-auto">
          <h1 class="text-base font-semibold text-gray-900 dark:text-white">crates_io</h1>
          <p class="mt-2 text-sm text-gray-700 dark:text-gray-300">{crates_io.len()} from the crates_io index</p>
        </div>
        <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
          <button type="button" class="block rounded-md bg-sky-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-xs hover:bg-sky-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600 dark:bg-sky-500 dark:hover:bg-sky-400 dark:focus-visible:outline-sky-500">Batch Together</button>
        </div>
      </div>
      <div class="mt-8 flow-root">
        <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
          <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
            <div class="group/table relative">

              <ActionForm action=handle_crates>

              <div class="absolute top-0 left-14 z-10 hidden h-12 items-center space-x-3 bg-white group-has-checked/table:flex sm:left-12 dark:bg-gray-900">
                <button type="submit" class="inline-flex items-center rounded-sm bg-white px-2 py-1 text-sm font-semibold text-gray-900 shadow-xs inset-ring inset-ring-gray-300 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-30 disabled:hover:bg-white dark:bg-white/10 dark:text-white dark:inset-ring-white/10 dark:hover:bg-white/15 dark:disabled:hover:bg-white/10">New Crate Info</button>
                // <button type="button" class="inline-flex items-center rounded-sm bg-white px-2 py-1 text-sm font-semibold text-gray-900 shadow-xs inset-ring inset-ring-gray-300 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-30 disabled:hover:bg-white dark:bg-white/10 dark:text-white dark:inset-ring-white/10 dark:hover:bg-white/15 dark:disabled:hover:bg-white/10">Delete all</button>
              </div>


              <table id="users-table" class="relative min-w-full table-fixed divide-y divide-gray-300 dark:divide-white/15">
                <thead>
                  <tr>
                    <th scope="col" class="relative px-7 sm:w-12 sm:px-6">
                      <div class="group absolute top-1/2 left-4 -mt-2 grid size-4 grid-cols-1">
                        <input type="checkbox" class="col-start-1 row-start-1 appearance-none rounded-sm border border-gray-300 bg-white checked:border-indigo-600 checked:bg-indigo-600 indeterminate:border-indigo-600 indeterminate:bg-indigo-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:checked:bg-gray-100 dark:border-white/20 dark:bg-gray-800/50 dark:checked:border-indigo-500 dark:checked:bg-indigo-500 dark:indeterminate:border-indigo-500 dark:indeterminate:bg-indigo-500 dark:focus-visible:outline-indigo-500 dark:disabled:border-white/10 dark:disabled:bg-gray-800 dark:disabled:checked:bg-gray-800 forced-colors:appearance-auto" />
                        <svg viewBox="0 0 14 14" fill="none" class="pointer-events-none col-start-1 row-start-1 size-3.5 self-center justify-self-center stroke-white group-has-disabled:stroke-gray-950/25 dark:group-has-disabled:stroke-white/25">
                          <path d="M3 8L6 11L11 3.5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-0 group-has-checked:opacity-100" />
                          <path d="M3 7H11" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-0 group-has-indeterminate:opacity-100" />
                        </svg>
                      </div>
                    </th>
                    <th scope="col" class="min-w-48 py-3.5 pr-3 text-left text-sm font-semibold text-gray-900 dark:text-white">crate_name</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900 dark:text-white">last_pubtime</th>
                    <th scope="col" class="py-3.5 pr-4 pl-3 sm:pr-3">
                      <span class="sr-only">Edit</span>
                    </th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-gray-200 bg-white dark:divide-white/10 dark:bg-gray-900">
                    <For
                        each=move || crates_io_filtered.clone()
                        key=|crate_info| crate_info.crate_name.clone()
                        children=move |info: CrateInfo| {
                        view! {
                            <tr class="group has-checked:bg-gray-50 dark:has-checked:bg-gray-800/50">
                                <td class="relative px-7 sm:w-12 sm:px-6">
                                <div class="absolute inset-y-0 left-0 hidden w-0.5 bg-indigo-600 group-has-checked:block dark:bg-indigo-500"></div>

                                <div class="group absolute top-1/2 left-4 -mt-2 grid size-4 grid-cols-1">
                                    <input
                                        type="checkbox"
                                        class="col-start-1 row-start-1 appearance-none rounded-sm border border-gray-300 bg-white checked:border-indigo-600 checked:bg-indigo-600 indeterminate:border-indigo-600 indeterminate:bg-indigo-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:checked:bg-gray-100 dark:border-white/20 dark:bg-gray-800/50 dark:checked:border-indigo-500 dark:checked:bg-indigo-500 dark:indeterminate:border-indigo-500 dark:indeterminate:bg-indigo-500 dark:focus-visible:outline-indigo-500 dark:disabled:border-white/10 dark:disabled:bg-gray-800 dark:disabled:checked:bg-gray-800 forced-colors:appearance-auto"
                                        name="crate_names[]"
                                        value={info.crate_name.clone()}
                                        />
                                    <svg viewBox="0 0 14 14" fill="none" class="pointer-events-none col-start-1 row-start-1 size-3.5 self-center justify-self-center stroke-white group-has-disabled:stroke-gray-950/25 dark:group-has-disabled:stroke-white/25">
                                    <path d="M3 8L6 11L11 3.5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-0 group-has-checked:opacity-100" />
                                    <path d="M3 7H11" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-0 group-has-indeterminate:opacity-100" />
                                    </svg>
                                </div>
                                </td>
                                <td class="py-4 pr-3 text-sm font-medium whitespace-nowrap text-gray-900 group-has-checked:text-indigo-600 dark:text-white dark:group-has-checked:text-indigo-400">
                                    <a
                                        href=format!("https://docs.rs/{crate_name}",
                                                crate_name=info.crate_name,
                                        )
                                        class="text-sky-400 hover:text-sky-500"
                                        >{info.crate_name.clone()}</a>
                                </td>
                                <td class="px-3 py-4 text-sm whitespace-nowrap text-gray-500 dark:text-gray-400">{info.max_pubtime.unwrap().to_string()}</td>
                                <td class="py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-3">
                                <ActionForm action=exclude_crate>
                                    <input class="hidden" type="text" name="crate_name" value={info.crate_name.clone()}/>
                                    <button
                                        class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300"
                                        type="submit"
                                        >
                                        Exclude
                                    </button>
                                 </ActionForm>
                                </td>
                            </tr>
                        }
                        }
                    />
                </tbody>
              </table>
                </ActionForm>
            </div>
          </div>
        </div>
      </div>
    </div>
        }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlCrateInfoRow {
    crate_id: Vec<u8>,
    display_name: String,
    description: Option<String>,
    associated_crates: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CrateInfoRow {
    pub crate_id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub associated_crates: Vec<String>,
}

#[cfg(feature = "ssr")]
impl From<SqlCrateInfoRow> for CrateInfoRow {
    fn from(value: SqlCrateInfoRow) -> Self {
        let id_str = rusty_ulid::Ulid::try_from(
            value.crate_id.as_slice(),
        )
        .expect(
            "expect valid ids from the
database",
        );

        let associated =
            if let Some(serde_json::Value::Array(crates)) =
                value.associated_crates
            {
                crates
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
            } else {
                vec![]
            };
        CrateInfoRow {
            crate_id: id_str.to_string(),
            display_name: value.display_name,
            description: value.description,
            associated_crates: associated,
        }
    }
}

#[server]
pub async fn fetch_crate_rows(
) -> Result<Vec<CrateInfoRow>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let crate_rows: Vec<SqlCrateInfoRow> = sqlx::query_as!(
        SqlCrateInfoRow,
        "SELECT
            crate.crate_id,
            display_name,
            description,
            JSON_ARRAYAGG(crate__associated_crates.crate_name) AS associated_crates
FROM crate
LEFT JOIN crate__associated_crates on crate.crate_id = crate__associated_crates.crate_id
GROUP BY crate.crate_id" 
    )
    .fetch_all(&pool)
    .await?;

    Ok(crate_rows
        .into_iter()
        .map(CrateInfoRow::from)
        .collect())
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateInfo {
    pub crate_name: String,
    pub max_pubtime: Option<OffsetDateTime>,
}

#[server]
pub async fn fetch_crates_io_data(
) -> Result<Vec<CrateInfo>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let crates_io: Vec<CrateInfo> = sqlx::query_as!(
        CrateInfo,
        "SELECT 
        crate_name,
        MAX(pubtime) as max_pubtime
FROM crates_io
WHERE crate_name NOT IN (
    SELECT crate_name from crate__associated_crates
    UNION
    SELECT crate_name FROM thisweekinbevy.crates_io_filters where exclude = 1
)
GROUP BY crate_name"
    )
    .fetch_all(&pool)
    .await
    .inspect_err(|err| {
        tracing::error!(?err);
    })?;

    Ok(crates_io)
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone,
)]
struct HeftyData {
    first_name: String,
    last_name: String,
}

#[server]
async fn handle_crates(
    crate_names: Vec<String>,
) -> Result<(), ServerFnError> {
    tracing::info!(?crate_names);
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    if crate_names.len() == 0 {
        return Ok(());
    }

    let first = crate_names[0].clone();

    let mut tx = pool.begin().await?;

    // 1. create new Crate collection
    //
    // 2. associate the selected crates with the new
    //    Crate collection
    let id: [u8; 16] = rusty_ulid::generate_ulid_bytes();

    let result = sqlx::query!(
        r#"INSERT INTO crate (crate_id, display_name)
     VALUES (?, ?)"#,
        id.as_slice(),
        first
    )
    .execute(&mut *tx)
    .await?;

    for crate_name in &crate_names {
        let result = sqlx::query!(
            r#"INSERT INTO crate__associated_crates (crate_id, crate_name)
     VALUES (?, ?)"#,
            id.as_slice(),
            crate_name
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

#[server]
async fn exclude_crate(
    crate_name: String,
) -> Result<(), ServerFnError> {
    tracing::info!(?crate_name);
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    sqlx::query!(
            r#"INSERT INTO crates_io_filters (crate_name, exclude)
     VALUES (?, ?)"#,
            crate_name,
            true
        )
        .execute(&pool)
        .await
        .inspect_err(|err| {
        tracing::error!(?err);
    })?;

    Ok(())
}

#[server]
async fn update_crate_details(
    id: String,
    display_name: String,
    description: String,
) -> Result<(), ServerFnError> {
    tracing::info!(?display_name);
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    tracing::info!(?id, ?display_name, ?description);

    let crate_release_id: [u8; 16] = id
        .parse::<rusty_ulid::Ulid>()
        .expect("a valid ulid to be returned from the form")
        .into();

    sqlx::query!(
        r#"
    UPDATE crate 
    SET 
        display_name = ?,
        description = ?
    WHERE crate_id = ?
        "#,
        display_name,
        description,
        crate_release_id.as_slice()
    )
    .execute(&pool)
    .await
    .inspect_err(|err| {
        tracing::error!(?err);
    })?;

    Ok(())
}
