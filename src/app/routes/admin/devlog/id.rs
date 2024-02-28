use crate::app::components::Divider;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::app::server_fn::error::NoCustomError;

#[server]
#[allow(unused_variables)]
async fn update_devlog(
    devlog_id: String,
    title: String,
    url: String,
    discord_url: String,
    description: String,
    posted_date: String,
) -> Result<(), ServerFnError> {
    let _pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let _id: [u8; 16] = devlog_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "expected a valid devlog id"
                    .to_string(),
            )
        })?
        .into();

    dbg!(title);
    // sqlx::query!(
    //     r#"
    // UPDATE showcase
    //   SET
    //   id = ?
    //   title = ?
    //   url = ?
    //   discord_url = ?
    //   posted_date = ?
    //   description
    // VALUES ( ?, ?, ?, ?, ?, ? )
    //     "#,
    //     id.as_slice(),
    //     title,
    //     url,
    //     discord_url,
    //     posted_date,
    //     description
    // )
    // .execute(&pool)
    // .await
    // .expect("successful insert");
    Ok(())
}

#[component]
pub fn Devlog() -> impl IntoView {
    let params = use_params_map();

    let update_devlog =
        create_server_action::<UpdateDevlog>();
    let devlog = create_resource(
        move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        },
        fetch_devlog_by_id,
    );

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <Suspense fallback=move || {
                view! { <p>"Loading Crate Release"</p> }
            }>
                {devlog
                    .get()
                    .map(|data| match data {
                        Err(e) => {
                            view! {
                                <div>
                                    <div>{e.to_string()}</div>
                                </div>
                            }
                        }
                        Ok(None) => {
                            view! {
                                <div>
                                    <div>{"Unable to find Crate Release".to_string()}</div>
                                </div>
                            }
                        }
                        Ok(Some(devlog)) => {
                            let devlog_id = devlog.id.clone();
                            view! {
                                <div>
                                    <ActionForm
                                        class="isolate -space-y-px rounded-md shadow-sm"
                                        action=update_devlog
                                    >
                                        <div class="relative rounded-md rounded-b-none px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                                            <label
                                                for="title"
                                                class="block text-xs font-medium text-gray-900"
                                            >
                                                Title
                                            </label>
                                            <input
                                                type="hidden"
                                                name="devlog_id"
                                                id="devlog_id"
                                                value=devlog.id
                                            />
                                            <input
                                                required
                                                type="text"
                                                name="title"
                                                id="title"
                                                class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                                                value=devlog.title
                                            />
                                        </div>
                                        <div class="relative px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                                            <label
                                                for="video_url"
                                                class="block text-xs font-medium text-gray-900"
                                            >
                                                Video URL
                                            </label>
                                            <input
                                                required
                                                type="text"
                                                name="video_url"
                                                id="video_url"
                                                class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                                                value=devlog.video_url
                                            />
                                        </div>
                                        <div class="relative px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                                            <label
                                                for="post_url"
                                                class="block text-xs font-medium text-gray-900"
                                            >
                                                Post URL
                                            </label>
                                            <input
                                                required
                                                type="text"
                                                name="post_url"
                                                id="post_url"
                                                class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                                                value=devlog.post_url
                                            />
                                        </div>
                                        <div class="relative rounded-md rounded-t-none px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                                            <label
                                                for="discord_url"
                                                class="block text-xs font-medium text-gray-900"
                                            >
                                                Discord URL
                                            </label>
                                            <input
                                                type="text"
                                                name="discord_url"
                                                id="discord_url"
                                                class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                                                value=devlog.discord_url
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
                                            <input
                                                type="date"
                                                id="posted_date"
                                                name="posted_date"
                                                min="2024-01-01"
                                                value=devlog.posted_date.unwrap().to_string()
                                            />
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
                                            >
                                                {devlog.description}
                                            </textarea>
                                        </div>
                                        <button
                                            type="submit"
                                            class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                                        >
                                            Update Release
                                        </button>
                                    </ActionForm>
                                    <Divider title="Crate Release Images"/>
                                    <ul
                                        role="list"
                                        class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
                                    >
                                        <For
                                            each=move || devlog.images.clone()
                                            key=|image| image.id.clone()
                                            let:image
                                        >
                                            <DevlogImageLi
                                                devlog_id=devlog_id.clone()
                                                id=image.id
                                                url=image.url
                                            />
                                        </For>
                                    </ul>
                                </div>
                            }
                        }
                    })}

            </Suspense>
            <Divider title="All Images"/>
            <Images devlog_id=params.with(|p| { p.get("id").cloned().unwrap_or_default() })/>
        </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlDevlogData {
    id: Vec<u8>,
    title: String,
    video_url: String,
    post_url: String,
    posted_date: Option<time::Date>,
    discord_url: String,
    description: String,
    images: serde_json::Value,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DevlogData {
    id: String,
    title: String,
    video_url: String,
    post_url: String,
    posted_date: Option<time::Date>,
    discord_url: String,
    description: String,
    images: Vec<ImgDataTransformed>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImgData {
    id: String,
    cloudinary_public_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImgDataTransformed {
    id: String,
    url: String,
}

#[cfg(feature = "ssr")]
impl From<SqlDevlogData> for DevlogData {
    fn from(value: SqlDevlogData) -> Self {
        use data_encoding::BASE64;

        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        let images =
            serde_json::from_value::<Vec<ImgData>>(
                value.images,
            )
            .inspect_err(|e| {
                tracing::warn!(?e);
            })
            .unwrap_or_default()
            .into_iter()
            .map(|img_data| {
                use cloudinary::transformation::{
                    resize_mode::ResizeMode::ScaleByWidth,
                    Image as CImage,
                    Transformations::Resize,
                };

                let base_id = BASE64
                    .decode(img_data.id.as_bytes())
                    .expect("a valid id in base64 format");
                let img_ulid = rusty_ulid::Ulid::try_from(
                    base_id.as_slice(),
                )
                .expect(
                    "expect valid ids from the database",
                );
                let image = CImage::new(
                    "dilgcuzda".into(),
                    img_data.cloudinary_public_id.into(),
                )
                .add_transformation(Resize(ScaleByWidth {
                    width: 300,
                    ar: None,
                    liquid: None,
                }));
                ImgDataTransformed {
                    id: img_ulid.to_string(),
                    url: image.to_string(),
                }
            })
            .collect();

        DevlogData {
            id: id_str.to_string(),
            title: value.title,
            video_url: value.video_url,
            post_url: value.post_url,
            posted_date: value.posted_date,
            discord_url: value.discord_url,
            description: value.description,
            images,
        }
    }
}

#[server]
pub async fn fetch_devlog_by_id(
    devlog_id: String,
) -> Result<Option<DevlogData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let devlog_id: [u8; 16] = devlog_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "expected a valid issue id".to_string(),
            )
        })?
        .into();

    let devlog: Option<SqlDevlogData> = sqlx::query_as!(
        SqlDevlogData,
        r#"SELECT
    id,
    title,
    video_url,
    post_url,
    posted_date,
    discord_url,
    description,
    images
from
    devlog
    LEFT JOIN (
        SELECT
            devlog_id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "id",
                    TO_BASE64(image.id),
                    "cloudinary_public_id",
                    cloudinary_public_id
                )
            ) AS images
        FROM
            devlog__image
            LEFT JOIN image ON devlog__image.image_id = image.id
        GROUP BY
            devlog_id
    ) as i on i.devlog_id = devlog.id
WHERE devlog.id = ?"#,
devlog_id.as_slice()
    )
    .fetch_optional(&pool)
    .await?;

    Ok(devlog.map(DevlogData::from))
}

#[server]
async fn associate_image_with_devlog(
    image_id: String,
    devlog_id: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let image_id: [u8; 16] = image_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "expected a valid image id".to_string(),
            )
        })?
        .into();

    let devlog_id: [u8; 16] = devlog_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "expected a valid devlog id"
                    .to_string(),
            )
        })?
        .into();

    sqlx::query!(
        r#"
    INSERT INTO devlog__image ( image_id, devlog_id )
    VALUES ( ?, ? )
        "#,
        image_id.as_slice(),
        devlog_id.as_slice()
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        ServerFnError::<NoCustomError>::ServerError(
            e.to_string(),
        )
    })?;
    Ok(())
}

#[server]
async fn remove_image_from_devlog(
    image_id: String,
    devlog_id: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let image_id: [u8; 16] = image_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "expected a valid image id".to_string(),
            )
        })?
        .into();

    let devlog_id: [u8; 16] = devlog_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "expected a valid devlog id"
                    .to_string(),
            )
        })?
        .into();

    sqlx::query!(
        r#"
    DELETE FROM devlog__image
    WHERE image_id = ?
    AND devlog_id = ?
        "#,
        image_id.as_slice(),
        devlog_id.as_slice()
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        ServerFnError::<NoCustomError>::ServerError(
            e.to_string(),
        )
    })?;
    Ok(())
}

#[component]
fn Images(devlog_id: String) -> impl IntoView {
    let images =
        create_resource(move || {}, |_| fetch_images());

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading (Suspense Fallback)..."</p> }
        }>

            {
                let devlog_id = devlog_id.clone();
                images
                    .get()
                    .map(move |data| match (devlog_id, data) {
                        (_, Err(e)) => view! { <div>{e.to_string()}</div> }.into_view(),
                        (devlog_id, Ok(images)) => {
                            view! {
                                <ul
                                    role="list"
                                    class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
                                >
                                    <For
                                        each=move || images.clone()
                                        key=|image| image.id.clone()
                                        let:image
                                    >
                                        <ImageLi
                                            devlog_id=devlog_id.clone()
                                            id=image.id
                                            url=image.url
                                            description=image.description
                                        />
                                    </For>
                                </ul>
                            }
                                .into_view()
                        }
                    })
            }

        </Suspense>
    }
}

#[component]
fn DevlogImageLi(
    devlog_id: String,
    id: String,
    url: String,
) -> impl IntoView {
    let remove_image_from_devlog =
        create_server_action::<RemoveImageFromDevlog>(
        );

    view! {
        <li class="relative">
            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                <img
                    src=url
                    alt=""
                    class="pointer-events-none object-cover group-hover:opacity-75"
                />
                <button type="submit" class="absolute inset-0 focus:outline-none">
                    <span class="sr-only">View details</span>
                </button>
            </div>
            <p class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">
                {&id}
            </p>
            // <p class="pointer-events-none block text-sm font-medium text-gray-500">{description}</p>
            <ActionForm action=remove_image_from_devlog>
                <input type="hidden" name="devlog_id" value=devlog_id/>
                <input type="hidden" name="image_id" value=id/>
                <button
                    type="submit"
                    class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                    Remove from Devlog
                </button>
            </ActionForm>
        </li>
    }
}

#[component]
fn ImageLi(
    devlog_id: String,
    id: String,
    url: String,
    description: String,
) -> impl IntoView {
    let associate_image_with_devlog =
        create_server_action::<
            AssociateImageWithDevlog,
        >();

    view! {
        <li class="relative">
            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                <img
                    src=url
                    alt=""
                    class="pointer-events-none object-cover group-hover:opacity-75"
                />
                <button type="submit" class="absolute inset-0 focus:outline-none">
                    <span class="sr-only">View details</span>
                </button>
            </div>
            <p class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">
                {&id}
            </p>
            <p class="pointer-events-none block text-sm font-medium text-gray-500">{description}</p>
            <ActionForm action=associate_image_with_devlog>
                <input type="hidden" name="devlog_id" value=devlog_id/>
                <input type="hidden" name="image_id" value=id/>
                <button
                    type="submit"
                    class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                    Add To Devlog
                </button>
            </ActionForm>
        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlImage {
    id: Vec<u8>,
    description: String,
    cloudinary_public_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: String,
    pub description: String,
    pub url: String,
}

#[cfg(feature = "ssr")]
impl From<SqlImage> for Image {
    fn from(value: SqlImage) -> Self {
        use cloudinary::transformation::{
            resize_mode::ResizeMode::ScaleByWidth,
            Image as CImage, Transformations::Resize,
        };

        let image = CImage::new(
            "dilgcuzda".into(),
            value.cloudinary_public_id.into(),
        )
        .add_transformation(Resize(ScaleByWidth {
            width: 300,
            ar: None,
            liquid: None,
        }));

        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        Image {
            id: id_str.to_string(),
            description: value.description,
            url: image.to_string(),
        }
    }
}

#[server]
async fn fetch_images() -> Result<Vec<Image>, ServerFnError>
{
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let images: Vec<SqlImage> = sqlx::query_as!(
        SqlImage,
        r#"SELECT
    id,
    cloudinary_public_id,
    description
FROM image
ORDER BY created_at DESC
limit 5"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError(
            "sql failed".to_string(),
        )
    })?;

    Ok(images.into_iter().map(Image::from).collect())
}
