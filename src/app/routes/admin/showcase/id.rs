use leptos::*;
use leptos_router::*;
use serde::{Serialize, Deserialize};
use crate::app::components::Divider;
use futures::future::join;
#[cfg(feature = "ssr")]
use crate::app::server_fn::error::NoCustomError;

#[server]
async fn update_showcase(
    showcase_id: String,
    title: String,
    url: String,
    discord_url: String,
    description: String,
    posted_date: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let id: [u8; 16] = showcase_id
    .parse::<rusty_ulid::Ulid>()
    .map_err(|_| ServerFnError::<NoCustomError>::ServerError("expected a valid showcase id".to_string()))?
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
pub fn Showcase() -> impl IntoView {
    let params = use_params_map();

    let update_showcase =
        create_server_action::<UpdateShowcase>();
    let showcase =
        create_resource(move || {
            params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })
        }, 
            fetch_showcase_by_id
        );

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
        <Suspense fallback=move || view! { <p>"Loading Showcase"</p> }>
        {showcase.get().map(|data| match data {
            Err(e) => view!{ <div><div>{e.to_string()}</div></div> },
            Ok(None) => view!{ <div><div>{"Unable to find Showcase".to_string()}</div></div> },
            Ok(Some(showcase)) => {
                let showcase_id = showcase.id.clone();
                view! {
                    <div>
            <ActionForm class="isolate -space-y-px rounded-md shadow-sm" action=update_showcase>
                <div class="relative rounded-md rounded-b-none px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                    <label for="title" class="block text-xs font-medium text-gray-900">
                        Title
                    </label>
                    <input
                      type="hidden"
                      name="showcase_id"
                      id="showcase_id"
                      value=showcase.id
                    />
                    <input
                        required
                        type="text"
                        name="title"
                        id="title"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                        value=showcase.title
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
                        value=showcase.url
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
                        value=showcase.discord_url
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
                    <input type="date" id="posted_date" name="posted_date" min="2024-01-01" value=showcase.posted_date.unwrap().to_string()/>
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
                    >{showcase.description}</textarea>
                </div>
                <button
                    type="submit"
                    class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                    Update Showcase
                </button>
            </ActionForm>
            <Divider title="Showcase Images"/>
            <ul role="list" class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8">
                <For
                    each=move || showcase.images.clone()
                    key=|image| image.id.clone()
                    let:image
                >
                    <ShowcaseImageLi showcase_id=showcase_id.clone() id=image.id url=image.url/>
                </For>
            </ul>
            </div>
                    }}
                })
            }
            </Suspense>
            <Divider title="All Images"/>
            <Images showcase_id=params.with(|p| {
                p.get("id").cloned().unwrap_or_default()
            })/>
        </div>
    }
}


#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlShowcaseData {
    id: Vec<u8>,
    title: String,
    url: String,
    posted_date: Option<time::Date>,
    discord_url: String,
    description: String,
    images: serde_json::Value
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ShowcaseData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub posted_date: Option<time::Date>,
    pub discord_url: String,
    pub description: String,
    pub images: Vec<ImgDataTransformed>
    
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImgData {
    id: String,
    cloudinary_public_id: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImgDataTransformed {
    id: String,
    url: String
}

#[cfg(feature = "ssr")]
impl From<SqlShowcaseData> for ShowcaseData {
    fn from(value: SqlShowcaseData) -> Self {
        use data_encoding::BASE64;
       
        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        let images = serde_json::from_value::<Vec<ImgData>>(value.images)
        .inspect_err(|e| {tracing::warn!(?e);})
        .unwrap_or(vec![])
        .into_iter()
        .map(|img_data| {
            use cloudinary::transformation::{
                Transformations::Resize, resize_mode::ResizeMode::ScaleByWidth, Image as CImage, aspect_ratio::AspectRatio
            };
            
            let base_id = BASE64.decode(&img_data.id.as_bytes()).expect("a valid id in base64 format");
            let img_ulid = rusty_ulid::Ulid::try_from(base_id.as_slice())
            .expect(
                "expect valid ids from the database",
            );
            let image = CImage::new("dilgcuzda".into(), img_data.cloudinary_public_id.into())
                .add_transformation(Resize(ScaleByWidth{ width:300, ar: None, liquid:None }));
            ImgDataTransformed {
                id: img_ulid.to_string(),
                url: image.to_string()
            }
        }).collect();

        ShowcaseData {
            id: id_str.to_string(),
            title: value.title,
url: value.url,
posted_date: value.posted_date,
discord_url: value.discord_url,
description: value.description,
images
        }
    }
}


#[server]
pub async fn fetch_showcase_by_id(
    showcase_id: String
) -> Result<Option<ShowcaseData>, ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let showcase_id: [u8; 16] = showcase_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("expected a valid issue id".to_string()))?
        .into();

    let showcase: Option<SqlShowcaseData> = sqlx::query_as!(
        SqlShowcaseData,
        r#"SELECT
    id,
    title,
    url,
    posted_date,
    discord_url,
    description,
    images
from
    showcase
    LEFT JOIN (
        SELECT
            showcase_id,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    "id",
                    TO_BASE64(image.id),
                    "cloudinary_public_id",
                    cloudinary_public_id
                )
            ) AS images
        FROM
            showcase__image
            LEFT JOIN image ON showcase__image.image_id = image.id
        GROUP BY
            showcase_id
    ) as i on i.showcase_id = showcase.id
WHERE showcase.id = ?"#,
showcase_id.as_slice()
    )
    .fetch_optional(&pool)
    .await?;

    Ok(showcase.map(ShowcaseData::from))
}

#[server]
async fn associate_image_with_showcase(
    image_id: String,
    showcase_id: String,
) -> Result<(), ServerFnError> {

    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let image_id: [u8; 16] = image_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("expected a valid image id".to_string()))?
        .into();

    let showcase_id: [u8; 16] = showcase_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("expected a valid showcase id".to_string()))?
        .into();

    sqlx::query!(
        r#"
    INSERT INTO showcase__image ( image_id, showcase_id )
    VALUES ( ?, ? )
        "#,
        image_id.as_slice(),
        showcase_id.as_slice()
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;
    Ok(())
}

#[server]
async fn remove_image_from_showcase(
    image_id: String,
    showcase_id: String,
) -> Result<(), ServerFnError> {

    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;

    let image_id: [u8; 16] = image_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("expected a valid image id".to_string()))?
        .into();

    let showcase_id: [u8; 16] = showcase_id
        .parse::<rusty_ulid::Ulid>()
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("expected a valid showcase id".to_string()))?
        .into();

    sqlx::query!(
        r#"
    DELETE FROM showcase__image
    WHERE image_id = ?
    AND showcase_id = ?
        "#,
        image_id.as_slice(),
        showcase_id.as_slice()
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;
    Ok(())
}

#[component]
fn Images(showcase_id: String) -> impl IntoView {
    let images = create_resource(move || {}, |_| fetch_images());

    view! {
        <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
        {
            let showcase_id = showcase_id.clone();
            
            images.get().map(move |data| match (showcase_id, data) {
            (_, Err(e)) => view!{ <div>{e.to_string()}</div> }.into_view(),
            (showcase_id, Ok(images)) => {
                view! {
                    <ul role="list" class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8">
                        <For
                            each=move || images.clone()
                            key=|image| image.id.clone()
                            let:image
                        >
                            <ImageLi showcase_id=showcase_id.clone() id=image.id url=image.url description=image.description/>
                        </For>
                    </ul>
                    }.into_view()
                }
            })
            }
        </Suspense>
    }
}

#[component]
fn ShowcaseImageLi(showcase_id: String, id: String, url: String) -> impl IntoView {
    let remove_image_from_showcase = create_server_action::<RemoveImageFromShowcase>();

    view! {
        <li class="relative">
            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
            <img src=url alt="" class="pointer-events-none object-cover group-hover:opacity-75"/>
            <button type="submit" class="absolute inset-0 focus:outline-none">
                <span class="sr-only">View details</span>
            </button>
            </div>
            <p class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">{&id}</p>
            // <p class="pointer-events-none block text-sm font-medium text-gray-500">{description}</p>
          <ActionForm action=remove_image_from_showcase>
            <input type="hidden" name="showcase_id" value=showcase_id/>
            <input type="hidden" name="image_id" value=id/>
            <button
              type="submit"
              class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            >Remove from Showcase</button>
          </ActionForm>
        </li>
    }
}

#[component]
fn ImageLi(showcase_id: String, id: String, url: String, description: String) -> impl IntoView {
    let associate_image_with_showcase = create_server_action::<AssociateImageWithShowcase>();

    view! {
        <li class="relative">
            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
            <img src=url alt="" class="pointer-events-none object-cover group-hover:opacity-75"/>
            <button type="submit" class="absolute inset-0 focus:outline-none">
                <span class="sr-only">View details</span>
            </button>
            </div>
            <p class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">{&id}</p>
            <p class="pointer-events-none block text-sm font-medium text-gray-500">{description}</p>
          <ActionForm action=associate_image_with_showcase>
            <input type="hidden" name="showcase_id" value=showcase_id/>
            <input type="hidden" name="image_id" value=id/>
            <button
              type="submit"
              class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            >Add To Showcase</button>
          </ActionForm>
        </li>
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlImage {
    id: Vec<u8>,
    description: String,
    cloudinary_public_id: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: String,
    pub description: String,
    pub url: String
}

#[cfg(feature = "ssr")]
impl From<SqlImage> for Image {
    fn from(value: SqlImage) -> Self {
        use cloudinary::transformation::{
            Transformations::Resize, resize_mode::ResizeMode::ScaleByWidth, Image as CImage, aspect_ratio::AspectRatio
        };
        
        let image = CImage::new("dilgcuzda".into(), value.cloudinary_public_id.into())
            .add_transformation(Resize(ScaleByWidth{ width:300, ar: None, liquid:None }));

        let id_str =
            rusty_ulid::Ulid::try_from(value.id.as_slice())
                .expect(
                    "expect valid ids from the database",
                );
        Image {
            id: id_str.to_string(),
            description: value.description,
            url: image.to_string()
        }
    }
}


#[server]
async fn fetch_images(
) -> Result<Vec<Image>, ServerFnError> {
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
    .map_err(|e| {
        ServerFnError::<NoCustomError>::ServerError("sql failed".to_string())
    })?;

    Ok(images.into_iter().map(Image::from).collect())
}
