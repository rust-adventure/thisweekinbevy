use crate::app::components::Divider;
#[cfg(feature = "ssr")]
use crate::app::server_fn::error::NoCustomError;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use tracing::error;

#[component]
pub fn Image() -> impl IntoView {
    let add_image = create_server_action::<AddImage>();

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <ActionForm class="isolate -space-y-px rounded-md shadow-sm" action=add_image>
                <div class="relative rounded-md px-3 pb-1.5 pt-2.5 ring-1 ring-inset ring-gray-300 focus-within:z-10 focus-within:ring-2 focus-within:ring-indigo-600">
                    <label
                        for="cloudinary_public_id"
                        class="block text-xs font-medium text-gray-900"
                    >
                        cloudinary_public_id
                    </label>
                    <input
                        required
                        type="text"
                        name="cloudinary_public_id"
                        id="cloudinary_public_id"
                        class="block w-full border-0 p-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
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
                    ></textarea>
                </div>
                <div class="pt-4">
                    <button
                        type="submit"
                        class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                        Add Image Id
                    </button>
                </div>
            </ActionForm>
            <Divider title="last 5 images"/>
            <Images/>
        </div>
    }
}

#[server]
async fn add_image(
    cloudinary_public_id: String,
    description: String,
) -> Result<(), ServerFnError> {
    let pool = crate::sql::pool()?;
    let _username = crate::sql::with_admin_access()?;
    let id: [u8; 16] = rusty_ulid::generate_ulid_bytes();

    sqlx::query!(
        r#"
    INSERT INTO image ( id, cloudinary_public_id, description )
    VALUES ( ?, ?, ? )
        "#,
        id.as_slice(),
        cloudinary_public_id,
        description
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        error!(?e);
        ServerFnError::<NoCustomError>::ServerError("sql failed".to_string())
    })?;
    Ok(())
}

#[component]
fn Images() -> impl IntoView {
    let images =
        create_resource(move || {}, |_| fetch_images());

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading (Suspense Fallback)..."</p> }
        }>
            {images
                .get()
                .map(|data| match data {
                    Err(e) => view! { <div>{e.to_string()}</div> }.into_view(),
                    Ok(images) => {
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
                                        id=image.id
                                        url=image.url
                                        description=image.description
                                    />
                                </For>
                            </ul>
                        }
                            .into_view()
                    }
                })}

        </Suspense>
    }
}

#[component]
fn ImageLi(
    id: String,
    url: String,
    description: String,
) -> impl IntoView {
    view! {
        <li class="relative">
            <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                <img
                    src=url
                    alt=""
                    class="pointer-events-none object-cover group-hover:opacity-75"
                />
                <button type="button" class="absolute inset-0 focus:outline-none">
                    <span class="sr-only">View details</span>
                </button>
            </div>
            <p class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">
                {id}
            </p>
            <p class="pointer-events-none block text-sm font-medium text-gray-500">{description}</p>
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
    .map_err(|e| {
        error!(?e);
        ServerFnError::<NoCustomError>::ServerError(
            "sql failed".to_string(),
        )
    })?;

    Ok(images.into_iter().map(Image::from).collect())
}
