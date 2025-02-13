use super::PROSE;
use crate::app::issue::ImgDataTransformed;
use leptos::{
    either::{Either, EitherOf8},
    prelude::*,
};
use std::ops::Not;

#[component]
pub fn SideBySide(
    title: String,
    type_name: String,
    images: Vec<ImgDataTransformed>,
    description: String,
    primary_url: String,
    discord_url: String,
    posted_date: Option<String>,
    #[prop(optional)] video_url: String,
) -> impl IntoView {
    view! {
        <div class="bg-ctp-base">
            <div class="mx-auto px-4 py-16 sm:px-6 sm:py-24 lg:max-w-7xl lg:px-8">
                <div class="lg:grid lg:grid-cols-7 lg:grid-rows-1 lg:gap-x-8 lg:gap-y-10 xl:gap-x-16">
                    {images
                        .is_empty()
                        .not()
                        .then_some(
                            view! {
                                <div class="lg:col-span-4 lg:row-end-1">
                                    <div class="aspect-h-3 aspect-w-4 overflow-hidden rounded-lg bg-gray-100 divide-y-8">

                                        {images
                                            .clone()
                                            .into_iter()
                                            .map(|image| {
                                                view! {
                                                    <img
                                                        loading="lazy"
                                                        class="w-full object-cover object-center"
                                                        src=image.url
                                                        alt=image.description
                                                    />
                                                }
                                            })
                                            .collect_view()}

                                    </div>
                                </div>
                            },
                        )}
                    <div class=format!(
                        "mx-auto mt-14 max-w-2xl sm:mt-16 {} lg:mt-0 lg:max-w-none",
                        if images.is_empty() {
                            "col-span-7"
                        } else {
                            "lg:col-span-3 lg:row-span-2 lg:row-end-2"
                        },
                    )>
                        <div class="flex flex-col-reverse">
                            <div class="mt-4">
                                <h2 class="text-2xl font-bold tracking-tight text-ctp-text sm:text-3xl">
                                    {title}
                                </h2>

                                <h3 id="information-heading" class="sr-only">
                                    {type_name}
                                </h3>

                                {posted_date
                                    .map(|date| {
                                        view! {
                                            <p class="mt-2 text-sm text-gray-500">
                                                "(Posted " <time datetime=date.clone()>{date.clone()}</time> ")"
                                            </p>
                                        }
                                    })}

                            </div>

                        </div>

                        <div
                            class=format!("mt-3 text-ctp-text {}", PROSE)
                            inner_html=description
                        ></div>

                        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-4 sm:grid-cols-2">
                            {discord_url
                                .trim()
                                .is_empty()
                                .not()
                                .then_some(
                                    view! {
                                        <a
                                            href=discord_url.clone()
                                            class="flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-50"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                viewBox="0 0 508.67 96.36"
                                                class="h-4"
                                            >
                                                <g fill="#fff">
                                                    <path d="M170.85 20.2h27.3q9.87 0 16.7 3.08a22.5 22.5 0 0 1 10.21 8.58 23.34 23.34 0 0 1 3.4 12.56A23.24 23.24 0 0 1 224.93 57a23.94 23.94 0 0 1-10.79 8.92q-7.24 3.3-17.95 3.29h-25.34Zm25.06 36.54q6.65 0 10.22-3.32a11.8 11.8 0 0 0 3.57-9.07 11.5 11.5 0 0 0-3.18-8.5q-3.2-3.18-9.63-3.19h-8.54v24.08ZM269.34 69.13a37 37 0 0 1-10.22-4.27V53.24a27.77 27.77 0 0 0 9.2 4.38 39.31 39.31 0 0 0 11.17 1.71 8.71 8.71 0 0 0 3.82-.66c.86-.44 1.29-1 1.29-1.58a2.37 2.37 0 0 0-.7-1.75 6.15 6.15 0 0 0-2.73-1.19l-8.4-1.89q-7.22-1.68-10.25-4.65a10.39 10.39 0 0 1-3-7.81 10.37 10.37 0 0 1 2.66-7.07 17.13 17.13 0 0 1 7.56-4.65 36 36 0 0 1 11.48-1.65A43.27 43.27 0 0 1 292 27.69a30.25 30.25 0 0 1 8.12 3.22v11a30 30 0 0 0-7.6-3.11 34 34 0 0 0-8.85-1.16q-6.58 0-6.58 2.24a1.69 1.69 0 0 0 1 1.58 16.14 16.14 0 0 0 3.74 1.08l7 1.26Q295.65 45 299 48t3.36 8.78a11.61 11.61 0 0 1-5.57 10.12q-5.53 3.71-15.79 3.7a46.41 46.41 0 0 1-11.66-1.47ZM318.9 67.66a21 21 0 0 1-9.07-8 21.59 21.59 0 0 1-3-11.34 20.62 20.62 0 0 1 3.15-11.27 21.16 21.16 0 0 1 9.24-7.8 34.25 34.25 0 0 1 14.56-2.84q10.5 0 17.43 4.41v12.83a21.84 21.84 0 0 0-5.7-2.73 22.65 22.65 0 0 0-7-1.05q-6.51 0-10.19 2.38a7.15 7.15 0 0 0-.1 12.43q3.57 2.41 10.36 2.41a23.91 23.91 0 0 0 6.9-1 25.71 25.71 0 0 0 5.84-2.49V66a34 34 0 0 1-17.85 4.62 32.93 32.93 0 0 1-14.57-2.96ZM368.64 67.66a21.77 21.77 0 0 1-9.25-8 21.14 21.14 0 0 1-3.18-11.41A20.27 20.27 0 0 1 359.39 37a21.42 21.42 0 0 1 9.21-7.74 38.17 38.17 0 0 1 28.7 0 21.25 21.25 0 0 1 9.17 7.7 20.41 20.41 0 0 1 3.15 11.27 21.29 21.29 0 0 1-3.15 11.41 21.51 21.51 0 0 1-9.2 8 36.32 36.32 0 0 1-28.63 0Zm21.27-12.42a9.12 9.12 0 0 0 2.56-6.76 8.87 8.87 0 0 0-2.56-6.68 9.53 9.53 0 0 0-7-2.49 9.67 9.67 0 0 0-7 2.49 8.9 8.9 0 0 0-2.55 6.68 9.15 9.15 0 0 0 2.55 6.76 9.53 9.53 0 0 0 7 2.55 9.4 9.4 0 0 0 7-2.55ZM451.69 29v15.14a12.47 12.47 0 0 0-6.93-1.75c-3.73 0-6.61 1.14-8.61 3.4s-3 5.77-3 10.53V69.2H416V28.25h16.8v13q1.4-7.14 4.52-10.53a10.38 10.38 0 0 1 8-3.4 11.71 11.71 0 0 1 6.37 1.68ZM508.67 18.8v50.4h-17.15V60a16.23 16.23 0 0 1-6.62 7.88A20.81 20.81 0 0 1 474 70.6a18.11 18.11 0 0 1-10.15-2.83 18.6 18.6 0 0 1-6.74-7.77 25.75 25.75 0 0 1-2.34-11.17 24.87 24.87 0 0 1 2.48-11.55 19.43 19.43 0 0 1 7.21-8 19.85 19.85 0 0 1 10.61-2.87q12.24 0 16.45 10.64V18.8ZM489 55a8.83 8.83 0 0 0 2.63-6.62A8.42 8.42 0 0 0 489 42a11 11 0 0 0-13.89 0 8.55 8.55 0 0 0-2.59 6.47 8.67 8.67 0 0 0 2.62 6.53 9.42 9.42 0 0 0 6.86 2.51 9.56 9.56 0 0 0 7-2.51ZM107.7 8.07A105.15 105.15 0 0 0 81.47 0a72.06 72.06 0 0 0-3.36 6.83 97.68 97.68 0 0 0-29.11 0A72.37 72.37 0 0 0 45.64 0a105.89 105.89 0 0 0-26.25 8.09C2.79 32.65-1.71 56.6.54 80.21a105.73 105.73 0 0 0 32.17 16.15 77.7 77.7 0 0 0 6.89-11.11 68.42 68.42 0 0 1-10.85-5.18c.91-.66 1.8-1.34 2.66-2a75.57 75.57 0 0 0 64.32 0c.87.71 1.76 1.39 2.66 2a68.68 68.68 0 0 1-10.87 5.19 77 77 0 0 0 6.89 11.1 105.25 105.25 0 0 0 32.19-16.14c2.64-27.38-4.51-51.11-18.9-72.15ZM42.45 65.69C36.18 65.69 31 60 31 53s5-12.74 11.43-12.74S54 46 53.89 53s-5.05 12.69-11.44 12.69Zm42.24 0C78.41 65.69 73.25 60 73.25 53s5-12.74 11.44-12.74S96.23 46 96.12 53s-5.04 12.69-11.43 12.69Z"></path>
                                                    <ellipse
                                                        cx="242.92"
                                                        cy="24.93"
                                                        rx="8.55"
                                                        ry="7.68"
                                                    ></ellipse>
                                                    <path d="M234.36 37.9a22.08 22.08 0 0 0 17.11 0v31.52h-17.11Z"></path>
                                                </g>
                                            </svg>
                                        </a>
                                    },
                                )}
                            {(primary_url != discord_url)
                                .then_some(view! { <PrimaryLink url=primary_url/> })}
                            <PrimaryLink url=video_url/>
                        </div>

                    // content under description/links
                    </div>
                    // technically body content under images
                    <div class="mx-auto mt-16 w-full max-w-2xl lg:col-span-4 lg:mt-0 lg:max-w-none"></div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PrimaryLink(url: String) -> impl IntoView {
    url
        .trim()
        .is_empty()
        .not()
        .then_some(view! {
            <a
                href=url
                class=format!(
                    "flex w-full items-center justify-center rounded-md border border-transparent px-8 py-3 text-base font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-50 {}",
                    match domain_heuristic(&url) {
                        Some(Domain::Discord) => {
                            "bg-brand-discord-faded1 text-brand-discord hover:bg-brand-discord-faded2 focus:ring-brand-discord"
                        }
                        Some(Domain::YouTube) => {
                            "bg-[#ffebeb] text-brand-youtube hover:border-brand-youtube focus:ring-brand-youtube"
                        }
                        Some(Domain::Itchio) => {
                            "bg-brand-itchio-faded1 text-brand-itchio hover:bg-brand-itchio-faded2 focus:ring-brand-itchio"
                        }
                        Some(Domain::Apple) => {
                            "bg-brand-apple-faded1 text-brand-apple hover:bg-brand-apple-faded2 focus:ring-brand-apple"
                        }
                        Some(Domain::GitHub) => {
                            "bg-white text-brand-github hover:border hover:border-brand-github focus:ring-brand-github"
                        }
                        Some(Domain::Mastodon) => {
                            "bg-brand-mastodon-faded1 text-brand-mastodon hover:bg-brand-mastodon-faded2 focus:ring-brand-mastodon"
                        }
                        Some(Domain::Cratesio) => "bg-[#264323] text-white focus:ring-[#264323]",
                        Some(Domain::Docsrs) => "bg-[#353535] text-white focus:ring-[#353535]",
                        None => "bg-sky-50 text-sky-700 hover:bg-sky-100 focus:ring-sky-500",
                    },
                )
            >

                {match domain_heuristic(&url) {
                    Some(d) => d.icon().into_any(),
                    None => {
                        view! {
                            <>
                                <span>Visit</span>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="ml-6 w-6 h-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"
                                    ></path>
                                </svg>
                            </>
                        }.into_any()
                    }
                }}

            </a>
        })
}

#[derive(Debug, Eq, PartialEq)]
enum Domain {
    Discord,
    YouTube,
    Itchio,
    Apple,
    GitHub,
    Mastodon,
    Docsrs,
    Cratesio,
}

impl Domain {
    fn icon(&self) -> impl IntoView {
        match self {
            Domain::Itchio => EitherOf8::A(view! {
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 774.778 198.926">
                    <g fill="#fff">
                        <path d="M253.95 174.12V70.95h34.81v103.17h-34.81zm17.614-111.56q-8.808 0-13.63-4.404-4.614-4.403-4.614-11.743 0-6.92 4.613-11.743 4.823-4.823 13.63-4.823 8.808 0 13.422 4.823 4.823 4.823 4.823 11.743 0 7.34-4.823 11.743-4.613 4.404-13.42 4.404zM340.7 176.22q-15.1 0-22.86-7.97-7.548-8.177-7.548-22.647v-48.86h-13.84V70.948h13.84V45.784h34.81V70.95h22.65v25.79H345.1v43.828q0 4.824 1.888 6.92 2.097 1.888 6.29 1.888 5.663 0 12.373-5.033l7.97 22.858q-6.08 4.2-13.84 6.71-7.76 2.31-19.08 2.31zm85.62 0q-23.907 0-37.747-13.63-13.63-13.632-13.63-39.635 0-18.873 7.758-31.665 7.97-13.21 19.93-17.825 12.58-4.823 23.28-4.823 13.42 0 22.44 5.452 9.02 5.243 13.21 11.534 4.41 6.29 6.29 9.856l-24.11 15.518q-3.35-6.92-7.34-10.905-3.98-3.984-9.64-3.984-7.97 0-12.58 6.29-4.61 6.292-4.61 19.084 0 13.84 5.45 20.34 5.45 6.502 15.52 6.502 7.97 0 13.21-2.94 5.45-2.94 10.277-7.55l11.115 26q-5.034 4.19-14.89 8.39-9.856 3.98-23.906 3.98zm50.65-2.1V34.04h35.02v42.57q4.403-3.146 10.694-5.452 6.29-2.517 15.1-2.517 18.453 0 27.47 10.49 9.227 10.49 9.227 29.57v65.43h-35.02v-61.24q0-8.8-3.35-12.79-3.35-4.19-8.81-4.19-4.61 0-8.6 2.1-3.98 2.1-6.71 4.41v71.72h-35.02zm124.4 2.1q-8.39 0-13.212-4.823-4.823-4.823-4.823-12.372 0-7.55 4.823-12.582 4.823-5.033 13.21-5.033 7.97 0 12.793 5.033 4.83 5.033 4.83 12.582 0 7.55-4.82 12.372-4.61 4.823-12.79 4.823zm25.75-2.1V70.95h34.81v103.17h-34.81zm17.61-111.54q-8.81 0-13.632-4.404-4.613-4.404-4.613-11.743 0-6.92 4.613-11.743 4.823-4.823 13.63-4.823 8.808 0 13.422 4.823 4.823 4.823 4.823 11.743 0 7.34-4.823 11.743-4.613 4.404-13.42 4.404zm78.67 113.64q-12.164 0-21.6-3.984-9.437-4.194-16.147-11.324-6.5-7.34-10.066-17.196-3.355-10.066-3.355-21.81 0-17.404 7.55-30.406 7.758-12.792 19.292-17.825 11.743-5.033 24.325-5.033 18.03 0 29.77 8.388 11.95 8.388 16.78 20.97 4.82 12.582 4.82 23.906 0 11.743-3.57 21.81-3.35 9.855-10.07 17.195-6.5 7.13-16.15 11.33-9.435 3.99-21.6 3.99zm0-26.842q8.807 0 12.79-7.34 3.985-7.55 3.985-20.13 0-11.954-4.194-19.084-4.19-7.13-12.58-7.13-8.18 0-12.37 7.13-4.19 7.13-4.19 19.083 0 12.582 3.99 20.13 4.2 7.34 12.58 7.34z"></path>
                        <path
                            d="M28.832 1.228C19.188 6.954.186 28.785.004 34.51v9.478c0 12.014 11.23 22.572 21.424 22.572 12.24 0 22.44-10.146 22.442-22.187 0 12.04 9.85 22.187 22.093 22.187 12.242 0 21.776-10.146 21.776-22.187 0 12.04 10.47 22.187 22.71 22.187h.22c12.24 0 22.72-10.146 22.72-22.187 0 12.04 9.53 22.187 21.77 22.187s22.09-10.146 22.09-22.187c0 12.04 10.2 22.187 22.44 22.187 10.19 0 21.42-10.557 21.42-22.572V34.51c-.19-5.725-19.19-27.556-28.83-33.282-29.97-1.053-50.76-1.234-81.73-1.23C79.59 0 37.36.483 28.83 1.228zm58.753 59.674a25.261 25.261 0 0 1-4.308 5.546 25.588 25.588 0 0 1-17.94 7.32c-6.985 0-13.356-2.8-17.976-7.322-1.67-1.64-2.94-3.394-4.11-5.436v.004c-1.16 2.046-2.79 3.798-4.46 5.44a25.664 25.664 0 0 1-17.97 7.317c-.84 0-1.71-.23-2.42-.47-.982 10.25-1.4 20.04-1.545 27.19v.04c-.02 3.63-.035 6.61-.054 10.75.19 21.51-2.13 69.7 9.48 81.54 17.99 4.2 51.094 6.11 84.31 6.12h.003c33.214-.01 66.32-1.92 84.31-6.11 11.61-11.843 9.29-60.033 9.48-81.536-.017-4.14-.034-7.122-.053-10.75v-.04c-.15-7.142-.565-16.935-1.55-27.183-.71.24-1.587.473-2.43.473a25.681 25.681 0 0 1-17.975-7.316c-1.675-1.644-3.3-3.396-4.463-5.44l-.005-.006c-1.166 2.04-2.437 3.797-4.112 5.436-4.62 4.522-10.99 7.322-17.973 7.322s-13.32-2.8-17.94-7.32a25.428 25.428 0 0 1-4.31-5.547 25.185 25.185 0 0 1-4.266 5.546 25.673 25.673 0 0 1-17.98 7.32c-.244 0-.49-.01-.73-.02h-.008c-.243.01-.486.02-.73.02-6.986 0-13.357-2.8-17.978-7.32a25.161 25.161 0 0 1-4.27-5.544zM69.123 84.775l-.002.008h.02c7.31.016 13.81 0 21.85 8.783 6.34-.663 12.95-.996 19.58-.985h.01c6.63-.01 13.24.33 19.58.99 8.05-8.78 14.54-8.76 21.85-8.78h.02v-.01c3.458 0 17.27 0 26.9 27.04l10.347 37.1c7.665 27.6-2.453 28.28-15.073 28.3-18.72-.69-29.08-14.29-29.08-27.88-10.36 1.7-22.45 2.55-34.535 2.55h-.005c-12.086 0-24.172-.85-34.53-2.55 0 13.59-10.36 27.18-29.078 27.88-12.62-.02-22.74-.7-15.073-28.29l10.34-37.1c9.63-27.04 23.45-27.04 26.9-27.04zm41.44 21.25v.007c-.017.017-19.702 18.096-23.24 24.526l12.89-.516v11.24c0 .527 5.174.313 10.35.074h.007c5.177.24 10.35.453 10.35-.073v-11.24l12.89.514c-3.538-6.43-23.24-24.525-23.24-24.525v-.006l-.002.002z"
                            color="#000"
                        ></path>
                    </g>
                </svg>
            }),
            Domain::YouTube => EitherOf8::B(view! {
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="409.289 277.787 512 114.301">
                    <g class="style-scope yt-icon">
                        <g class="style-scope yt-icon">
                            <path
                                fill="red"
                                d="M569.154 295.637a20.447 20.447 0 0 0-14.436-14.436c-12.728-3.414-63.79-3.414-63.79-3.414s-51.061 0-63.79 3.414a20.447 20.447 0 0 0-14.435 14.436c-3.414 12.728-3.414 39.3-3.414 39.3s0 26.573 3.414 39.302a20.446 20.446 0 0 0 14.435 14.435c12.729 3.414 63.79 3.414 63.79 3.414s51.062 0 63.79-3.414a20.446 20.446 0 0 0 14.436-14.435c3.414-12.729 3.414-39.301 3.414-39.301s-.014-26.573-3.414-39.301Z"
                                class="style-scope yt-icon"
                            ></path>
                            <path
                                fill="#fff"
                                d="m474.585 359.429 42.42-24.49-42.42-24.488v48.978Z"
                                class="style-scope yt-icon"
                            ></path>
                        </g>
                        <g class="style-scope yt-icon">
                            <g class="style-scope yt-icon">
                                <path
                                    d="M34.602 13.004 31.395 1.418h2.798l1.124 5.252c.287 1.294.497 2.397.633 3.31h.082c.094-.655.306-1.75.633-3.291l1.164-5.27h2.799L37.38 13.003v5.557H34.6v-5.557h.002ZM41.47 18.194c-.565-.381-.967-.974-1.207-1.778-.237-.805-.357-1.872-.357-3.208V11.39c0-1.348.136-2.432.409-3.248.273-.816.699-1.413 1.277-1.787.579-.374 1.338-.563 2.279-.563.927 0 1.667.191 2.227.572.558.381.967.978 1.225 1.787.26.812.389 1.891.389 3.239v1.818c0 1.336-.128 2.408-.38 3.217-.25.811-.66 1.404-1.224 1.778-.565.374-1.332.562-2.298.562-.997.002-1.776-.19-2.34-.571Zm3.165-1.962c.156-.409.236-1.074.236-2.001v-3.902c0-.898-.078-1.557-.236-1.97-.157-.417-.432-.624-.828-.624-.38 0-.651.207-.806.623-.158.417-.235 1.073-.235 1.971v3.902c0 .927.075 1.594.225 2.001.15.41.421.614.816.614.396 0 .67-.204.828-.614ZM56.815 18.563H54.61l-.244-1.533h-.061c-.6 1.157-1.498 1.736-2.698 1.736-.83 0-1.444-.273-1.839-.816-.395-.546-.593-1.397-.593-2.554V6.037h2.82v9.193c0 .56.061.957.184 1.195.122.237.327.357.613.357.245 0 .48-.075.706-.226.226-.15.39-.34.5-.571v-9.95h2.818v12.527Z"
                                    class="style-scope yt-icon"
                                    style="fill:#282828"
                                    transform="matrix(5.71504 0 0 5.71504 409.289 277.787)"
                                ></path>
                                <path
                                    d="M64.475 3.688h-2.798v14.875h-2.759V3.688H56.12V1.42h8.356v2.268Z"
                                    class="style-scope yt-icon"
                                    style="fill:#282828"
                                    transform="matrix(5.71504 0 0 5.71504 409.289 277.787)"
                                ></path>
                                <path
                                    d="M71.277 18.563H69.07l-.245-1.533h-.06c-.6 1.157-1.499 1.736-2.699 1.736-.83 0-1.443-.273-1.839-.816-.395-.546-.592-1.397-.592-2.554V6.037h2.82v9.193c0 .56.06.957.183 1.195.122.237.327.357.614.357.244 0 .48-.075.705-.226.226-.15.39-.34.501-.571v-9.95h2.818v12.527ZM80.609 8.039c-.172-.79-.447-1.362-.828-1.717-.38-.355-.905-.532-1.573-.532-.518 0-1.002.146-1.451.44-.45.294-.798.677-1.042 1.155h-.021v-6.6h-2.717v17.776h2.329l.287-1.186h.06c.22.424.546.755.981 1.002.436.245.92.367 1.451.367.953 0 1.656-.44 2.105-1.317.45-.88.675-2.25.675-4.118v-1.982c0-1.4-.087-2.498-.256-3.288Zm-2.585 5.11c0 .913-.037 1.628-.113 2.145-.075.518-.2.887-.378 1.103a.871.871 0 0 1-.715.327c-.233 0-.447-.054-.645-.165a1.232 1.232 0 0 1-.48-.489V8.96c.095-.34.26-.618.492-.837.23-.218.485-.327.755-.327a.76.76 0 0 1 .663.337c.158.226.266.602.327 1.133.061.532.092 1.287.092 2.268v1.615h.002ZM84.866 13.871c0 .804.023 1.407.07 1.809.047.402.146.694.297.88.15.183.38.274.693.274.421 0 .713-.164.868-.491.158-.327.243-.873.257-1.634l2.431.143c.014.108.022.259.022.45 0 1.156-.318 2.022-.95 2.593-.633.572-1.53.859-2.686.859-1.39 0-2.364-.436-2.921-1.308-.56-.873-.838-2.22-.838-4.045v-2.187c0-1.88.29-3.253.868-4.118.579-.866 1.569-1.299 2.973-1.299.966 0 1.71.177 2.227.532.517.355.882.905 1.094 1.656.211.75.317 1.785.317 3.106v2.145h-4.722v.635Zm.357-5.903c-.143.176-.237.466-.287.868-.047.402-.07 1.011-.07 1.83v.898h2.062v-.898c0-.805-.028-1.414-.082-1.83-.054-.416-.153-.708-.296-.88-.144-.169-.365-.256-.664-.256-.3.002-.522.092-.663.268Z"
                                    class="style-scope yt-icon"
                                    style="fill:#282828"
                                    transform="matrix(5.71504 0 0 5.71504 409.289 277.787)"
                                ></path>
                            </g>
                        </g>
                    </g>
                </svg>
            }),
            Domain::Discord => EitherOf8::C(view! {
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 508.67 96.36">
                    <g fill="#fff">
                        <path d="M170.85 20.2h27.3q9.87 0 16.7 3.08a22.5 22.5 0 0 1 10.21 8.58 23.34 23.34 0 0 1 3.4 12.56A23.24 23.24 0 0 1 224.93 57a23.94 23.94 0 0 1-10.79 8.92q-7.24 3.3-17.95 3.29h-25.34Zm25.06 36.54q6.65 0 10.22-3.32a11.8 11.8 0 0 0 3.57-9.07 11.5 11.5 0 0 0-3.18-8.5q-3.2-3.18-9.63-3.19h-8.54v24.08ZM269.34 69.13a37 37 0 0 1-10.22-4.27V53.24a27.77 27.77 0 0 0 9.2 4.38 39.31 39.31 0 0 0 11.17 1.71 8.71 8.71 0 0 0 3.82-.66c.86-.44 1.29-1 1.29-1.58a2.37 2.37 0 0 0-.7-1.75 6.15 6.15 0 0 0-2.73-1.19l-8.4-1.89q-7.22-1.68-10.25-4.65a10.39 10.39 0 0 1-3-7.81 10.37 10.37 0 0 1 2.66-7.07 17.13 17.13 0 0 1 7.56-4.65 36 36 0 0 1 11.48-1.65A43.27 43.27 0 0 1 292 27.69a30.25 30.25 0 0 1 8.12 3.22v11a30 30 0 0 0-7.6-3.11 34 34 0 0 0-8.85-1.16q-6.58 0-6.58 2.24a1.69 1.69 0 0 0 1 1.58 16.14 16.14 0 0 0 3.74 1.08l7 1.26Q295.65 45 299 48t3.36 8.78a11.61 11.61 0 0 1-5.57 10.12q-5.53 3.71-15.79 3.7a46.41 46.41 0 0 1-11.66-1.47ZM318.9 67.66a21 21 0 0 1-9.07-8 21.59 21.59 0 0 1-3-11.34 20.62 20.62 0 0 1 3.15-11.27 21.16 21.16 0 0 1 9.24-7.8 34.25 34.25 0 0 1 14.56-2.84q10.5 0 17.43 4.41v12.83a21.84 21.84 0 0 0-5.7-2.73 22.65 22.65 0 0 0-7-1.05q-6.51 0-10.19 2.38a7.15 7.15 0 0 0-.1 12.43q3.57 2.41 10.36 2.41a23.91 23.91 0 0 0 6.9-1 25.71 25.71 0 0 0 5.84-2.49V66a34 34 0 0 1-17.85 4.62 32.93 32.93 0 0 1-14.57-2.96ZM368.64 67.66a21.77 21.77 0 0 1-9.25-8 21.14 21.14 0 0 1-3.18-11.41A20.27 20.27 0 0 1 359.39 37a21.42 21.42 0 0 1 9.21-7.74 38.17 38.17 0 0 1 28.7 0 21.25 21.25 0 0 1 9.17 7.7 20.41 20.41 0 0 1 3.15 11.27 21.29 21.29 0 0 1-3.15 11.41 21.51 21.51 0 0 1-9.2 8 36.32 36.32 0 0 1-28.63 0Zm21.27-12.42a9.12 9.12 0 0 0 2.56-6.76 8.87 8.87 0 0 0-2.56-6.68 9.53 9.53 0 0 0-7-2.49 9.67 9.67 0 0 0-7 2.49 8.9 8.9 0 0 0-2.55 6.68 9.15 9.15 0 0 0 2.55 6.76 9.53 9.53 0 0 0 7 2.55 9.4 9.4 0 0 0 7-2.55ZM451.69 29v15.14a12.47 12.47 0 0 0-6.93-1.75c-3.73 0-6.61 1.14-8.61 3.4s-3 5.77-3 10.53V69.2H416V28.25h16.8v13q1.4-7.14 4.52-10.53a10.38 10.38 0 0 1 8-3.4 11.71 11.71 0 0 1 6.37 1.68ZM508.67 18.8v50.4h-17.15V60a16.23 16.23 0 0 1-6.62 7.88A20.81 20.81 0 0 1 474 70.6a18.11 18.11 0 0 1-10.15-2.83 18.6 18.6 0 0 1-6.74-7.77 25.75 25.75 0 0 1-2.34-11.17 24.87 24.87 0 0 1 2.48-11.55 19.43 19.43 0 0 1 7.21-8 19.85 19.85 0 0 1 10.61-2.87q12.24 0 16.45 10.64V18.8ZM489 55a8.83 8.83 0 0 0 2.63-6.62A8.42 8.42 0 0 0 489 42a11 11 0 0 0-13.89 0 8.55 8.55 0 0 0-2.59 6.47 8.67 8.67 0 0 0 2.62 6.53 9.42 9.42 0 0 0 6.86 2.51 9.56 9.56 0 0 0 7-2.51ZM107.7 8.07A105.15 105.15 0 0 0 81.47 0a72.06 72.06 0 0 0-3.36 6.83 97.68 97.68 0 0 0-29.11 0A72.37 72.37 0 0 0 45.64 0a105.89 105.89 0 0 0-26.25 8.09C2.79 32.65-1.71 56.6.54 80.21a105.73 105.73 0 0 0 32.17 16.15 77.7 77.7 0 0 0 6.89-11.11 68.42 68.42 0 0 1-10.85-5.18c.91-.66 1.8-1.34 2.66-2a75.57 75.57 0 0 0 64.32 0c.87.71 1.76 1.39 2.66 2a68.68 68.68 0 0 1-10.87 5.19 77 77 0 0 0 6.89 11.1 105.25 105.25 0 0 0 32.19-16.14c2.64-27.38-4.51-51.11-18.9-72.15ZM42.45 65.69C36.18 65.69 31 60 31 53s5-12.74 11.43-12.74S54 46 53.89 53s-5.05 12.69-11.44 12.69Zm42.24 0C78.41 65.69 73.25 60 73.25 53s5-12.74 11.44-12.74S96.23 46 96.12 53s-5.04 12.69-11.43 12.69Z"></path>
                        <ellipse cx="242.92" cy="24.93" rx="8.55" ry="7.68"></ellipse>
                        <path d="M234.36 37.9a22.08 22.08 0 0 0 17.11 0v31.52h-17.11Z"></path>
                    </g>
                </svg>
            }),
            Domain::Apple => EitherOf8::D(view! {
                <>
                    <span>Apple</span>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 842.32 1000"
                        class="max-w-[50px]"
                    >
                        <path
                            fill="#fff"
                            d="M824.666 779.304c-15.123 34.937-33.023 67.096-53.763 96.663-28.271 40.308-51.419 68.208-69.258 83.702-27.653 25.43-57.282 38.455-89.01 39.196-22.776 0-50.245-6.481-82.219-19.63-32.08-13.085-61.56-19.566-88.516-19.566-28.27 0-58.59 6.48-91.022 19.567-32.48 13.148-58.646 20-78.652 20.678-30.425 1.296-60.75-12.098-91.022-40.245-19.32-16.852-43.486-45.74-72.436-86.665-31.06-43.702-56.597-94.38-76.602-152.155C10.74 658.443 0 598.013 0 539.509c0-67.017 14.481-124.818 43.486-173.255 22.796-38.906 53.122-69.596 91.078-92.126 37.955-22.53 78.967-34.012 123.132-34.746 24.166 0 55.856 7.475 95.238 22.166 39.27 14.74 64.485 22.215 75.54 22.215 8.266 0 36.277-8.74 83.764-26.166 44.906-16.16 82.806-22.85 113.854-20.215 84.133 6.79 147.341 39.955 189.377 99.707-75.245 45.59-112.466 109.447-111.725 191.364.68 63.807 23.827 116.904 69.319 159.063 20.617 19.568 43.64 34.69 69.257 45.431-5.555 16.11-11.42 31.542-17.654 46.357zM631.71 20.006c0 50.011-18.27 96.707-54.69 139.928-43.949 51.38-97.108 81.071-154.754 76.386-.735-6-1.16-12.314-1.16-18.95 0-48.01 20.9-99.392 58.016-141.403 18.53-21.271 42.098-38.957 70.677-53.066C578.316 9.002 605.29 1.316 630.66 0c.74 6.686 1.05 13.372 1.05 20.005z"
                        ></path>
                    </svg>
                </>
            }),
            Domain::GitHub => EitherOf8::E(view! {
                <>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="w-8"
                        preserveAspectRatio="xMinYMin meet"
                        viewBox="0 0 256 249"
                    >
                        <g fill="#161614">
                            <path d="M127.505 0C57.095 0 0 57.085 0 127.505c0 56.336 36.534 104.13 87.196 120.99 6.372 1.18 8.712-2.766 8.712-6.134 0-3.04-.119-13.085-.173-23.739-35.473 7.713-42.958-15.044-42.958-15.044-5.8-14.738-14.157-18.656-14.157-18.656-11.568-7.914.872-7.752.872-7.752 12.804.9 19.546 13.14 19.546 13.14 11.372 19.493 29.828 13.857 37.104 10.6 1.144-8.242 4.449-13.866 8.095-17.05-28.32-3.225-58.092-14.158-58.092-63.014 0-13.92 4.981-25.295 13.138-34.224-1.324-3.212-5.688-16.18 1.235-33.743 0 0 10.707-3.427 35.073 13.07 10.17-2.826 21.078-4.242 31.914-4.29 10.836.048 21.752 1.464 31.942 4.29 24.337-16.497 35.029-13.07 35.029-13.07 6.94 17.563 2.574 30.531 1.25 33.743 8.175 8.929 13.122 20.303 13.122 34.224 0 48.972-29.828 59.756-58.22 62.912 4.573 3.957 8.648 11.717 8.648 23.612 0 17.06-.148 30.791-.148 34.991 0 3.393 2.295 7.369 8.759 6.117 50.634-16.879 87.122-64.656 87.122-120.973C255.009 57.085 197.922 0 127.505 0"></path>
                            <path d="M47.755 181.634c-.28.633-1.278.823-2.185.389-.925-.416-1.445-1.28-1.145-1.916.275-.652 1.273-.834 2.196-.396.927.415 1.455 1.287 1.134 1.923m6.272 5.596c-.608.564-1.797.302-2.604-.589-.834-.889-.99-2.077-.373-2.65.627-.563 1.78-.3 2.616.59.834.899.996 2.08.36 2.65m4.304 7.159c-.782.543-2.06.034-2.849-1.1-.781-1.133-.781-2.493.017-3.038.792-.545 2.05-.055 2.85 1.07.78 1.153.78 2.513-.019 3.069m7.277 8.292c-.699.77-2.187.564-3.277-.488-1.114-1.028-1.425-2.487-.724-3.258.707-.772 2.204-.555 3.302.488 1.107 1.026 1.445 2.496.7 3.258m9.403 2.8c-.307.998-1.741 1.452-3.185 1.028-1.442-.437-2.386-1.607-2.095-2.616.3-1.005 1.74-1.478 3.195-1.024 1.44.435 2.386 1.596 2.086 2.612m10.703 1.187c.036 1.052-1.189 1.924-2.705 1.943-1.525.033-2.758-.818-2.774-1.852 0-1.062 1.197-1.926 2.721-1.951 1.516-.03 2.758.815 2.758 1.86m10.514-.403c.182 1.026-.872 2.08-2.377 2.36-1.48.27-2.85-.363-3.039-1.38-.184-1.052.89-2.105 2.367-2.378 1.508-.262 2.857.355 3.049 1.398"></path>
                        </g>
                    </svg>
                    <span class="text-[#1b1f23] ml-4 font-bold">GitHub</span>
                </>
            }),
            Domain::Mastodon => EitherOf8::F(view! {
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 314 80" fill="none">
                    <path
                        fill="url(#a)"
                        d="M73.445 17.696C72.317 9.198 65.004 2.49 56.346 1.198 54.881.98 49.346.176 36.521.176h-.096c-12.837 0-15.587.803-17.052 1.022-8.43 1.267-16.115 7.281-17.988 15.89C.496 21.325.4 26.023.568 30.334c.24 6.186.289 12.347.84 18.507a88.18 88.18 0 0 0 1.994 12.14c1.777 7.378 8.958 13.514 15.995 16.01a42.384 42.384 0 0 0 23.392 1.254 37.162 37.162 0 0 0 2.534-.706c1.885-.609 4.094-1.29 5.727-2.484.025-.012.037-.036.049-.06a.178.178 0 0 0 .024-.086v-5.966s0-.048-.024-.073c0-.024-.024-.049-.049-.06a16.122 16.122 0 0 1-.072-.037h-.072a63.814 63.814 0 0 1-15.178 1.802c-8.802 0-11.168-4.237-11.84-5.99a19.304 19.304 0 0 1-1.033-4.725c0-.024 0-.048.012-.073 0-.024.024-.048.049-.06l.071-.037h.085a63.02 63.02 0 0 0 14.938 1.802c1.212 0 2.413 0 3.626-.037 5.055-.146 10.387-.401 15.37-1.388.12-.024.253-.048.36-.073 7.854-1.534 15.323-6.331 16.08-18.482.024-.475.096-5.017.096-5.504 0-1.692.54-11.968-.084-18.287l-.013-.025Z"
                    ></path>
                    <path
                        fill="#fff"
                        d="M15.315 22.445c0-2.484 1.957-4.48 4.382-4.48 2.426 0 4.383 2.009 4.383 4.48 0 2.472-1.957 4.48-4.383 4.48-2.425 0-4.382-2.008-4.382-4.48Z"
                    ></path>
                    <path
                        fill="#000"
                        d="M80.519 26.305v20.6h-7.986V26.915c0-4.213-1.729-6.344-5.2-6.344-3.83 0-5.763 2.545-5.763 7.549v10.946h-7.925V28.119c0-5.029-1.91-7.549-5.764-7.549-3.459 0-5.2 2.13-5.2 6.344v19.98h-7.973v-20.59c0-4.2 1.044-7.548 3.146-10.032 2.173-2.484 5.02-3.738 8.538-3.738 4.082 0 7.18 1.607 9.222 4.822l1.993 3.409 1.994-3.41c2.041-3.201 5.127-4.82 9.222-4.82 3.518 0 6.364 1.265 8.538 3.737 2.101 2.484 3.146 5.808 3.146 10.033h.012Zm27.474 10.24c1.657-1.79 2.438-4.006 2.438-6.71 0-2.702-.793-4.943-2.438-6.647-1.585-1.79-3.602-2.642-6.04-2.642-2.437 0-4.443.852-6.04 2.642-1.585 1.704-2.377 3.945-2.377 6.648s.792 4.943 2.377 6.708c1.585 1.705 3.603 2.57 6.04 2.57 2.438 0 4.443-.853 6.04-2.57Zm2.438-23.183h7.865V46.31h-7.865v-3.884c-2.378 3.227-5.668 4.822-9.943 4.822-4.275 0-7.565-1.644-10.483-5.004-2.87-3.36-4.323-7.513-4.323-12.383 0-4.87 1.465-8.961 4.323-12.322 2.93-3.36 6.413-5.065 10.483-5.065 4.071 0 7.565 1.583 9.943 4.797v-3.908Zm34.331 15.853c2.318 1.79 3.471 4.286 3.411 7.439 0 3.36-1.153 6.003-3.531 7.829-2.377 1.79-5.248 2.703-8.73 2.703-6.28 0-10.543-2.642-12.801-7.829l6.822-4.152c.911 2.825 2.918 4.286 5.979 4.286 2.81 0 4.203-.913 4.203-2.825 0-1.388-1.826-2.642-5.548-3.616a45.29 45.29 0 0 1-3.47-1.12c-1.285-.524-2.378-1.12-3.291-1.85-2.257-1.79-3.409-4.153-3.409-7.184 0-3.227 1.092-5.796 3.29-7.647 2.258-1.911 5.007-2.824 8.297-2.824 5.247 0 9.078 2.3 11.588 6.976l-6.7 3.945c-.973-2.24-2.631-3.36-4.888-3.36-2.378 0-3.53.913-3.53 2.703 0 1.388 1.825 2.642 5.548 3.616 2.87.657 5.127 1.643 6.76 2.91h.012-.012Zm25.002-7.695h-6.894v13.71c0 1.643.613 2.642 1.778 3.092.852.329 2.558.39 5.128.268v7.707c-5.296.658-9.139.122-11.396-1.644-2.258-1.704-3.351-4.882-3.351-9.411V21.52h-5.296v-8.17h5.296V6.702l7.865-2.569v9.23h6.894v8.169h-.012l-.012-.012Zm25.072 14.83c1.586-1.705 2.377-3.884 2.377-6.526 0-2.643-.791-4.798-2.377-6.527-1.597-1.704-3.543-2.569-5.92-2.569-2.378 0-4.323.853-5.92 2.57-1.525 1.79-2.318 3.944-2.318 6.526 0 2.58.793 4.736 2.318 6.526 1.585 1.704 3.542 2.569 5.92 2.569 2.377 0 4.323-.853 5.92-2.57Zm-17.375 5.856c-3.111-3.36-4.635-7.44-4.635-12.382 0-4.944 1.524-8.962 4.635-12.322 3.11-3.36 6.952-5.065 11.455-5.065s8.358 1.704 11.456 5.065c3.098 3.36 4.695 7.512 4.695 12.322 0 4.809-1.597 9.022-4.695 12.382-3.111 3.36-6.892 5.004-11.456 5.004-4.563 0-8.357-1.643-11.455-5.004Zm53.903-5.674c1.586-1.79 2.378-4.005 2.378-6.708 0-2.703-.792-4.944-2.378-6.648-1.584-1.79-3.601-2.642-6.039-2.642-2.437 0-4.456.852-6.1 2.642-1.585 1.704-2.378 3.945-2.378 6.648 0 2.702.793 4.943 2.378 6.708 1.656 1.705 3.723 2.57 6.1 2.57 2.378 0 4.443-.853 6.039-2.57ZM233.742.164h7.865v46.133h-7.865v-3.884c-2.317 3.227-5.607 4.822-9.882 4.822-4.275 0-7.613-1.644-10.556-5.005-2.87-3.36-4.322-7.512-4.322-12.382s1.464-8.961 4.322-12.322c2.918-3.36 6.461-5.065 10.556-5.065 4.095 0 7.565 1.583 9.882 4.797V.164Zm35.496 36.149c1.585-1.705 2.378-3.884 2.378-6.526s-.793-4.797-2.378-6.526c-1.584-1.705-3.53-2.57-5.92-2.57-2.389 0-4.322.853-5.919 2.57-1.526 1.79-2.319 3.945-2.319 6.526s.793 4.736 2.319 6.526c1.585 1.705 3.542 2.57 5.919 2.57 2.378 0 4.324-.853 5.92-2.57Zm-17.375 5.857c-3.098-3.361-4.636-7.44-4.636-12.383 0-4.943 1.526-8.961 4.636-12.322 3.109-3.36 6.952-5.065 11.455-5.065 4.504 0 8.358 1.705 11.456 5.065 3.11 3.36 4.695 7.513 4.695 12.322 0 4.81-1.585 9.022-4.695 12.383-3.11 3.36-6.892 5.004-11.456 5.004-4.563 0-8.358-1.644-11.455-5.004ZM313.5 26.025V46.26h-7.865V27.084c0-2.18-.541-3.823-1.657-5.065-1.033-1.12-2.498-1.705-4.384-1.705-4.442 0-6.7 2.703-6.7 8.17v17.789h-7.865V13.35h7.865v3.701c1.885-3.092 4.887-4.614 9.079-4.614 3.349 0 6.099 1.18 8.238 3.616 2.197 2.435 3.289 5.735 3.289 10.008"
                    ></path>
                    <defs>
                        <linearGradient
                            id="a"
                            x1="37.121"
                            x2="37.121"
                            y1=".176"
                            y2="79.317"
                            gradientUnits="userSpaceOnUse"
                        >
                            <stop stop-color="#6364FF"></stop>
                            <stop offset="1" stop-color="#563ACC"></stop>
                        </linearGradient>
                    </defs>
                </svg>
            }),
            Domain::Cratesio => EitherOf8::G(view! {
                <>
                    <img
                        src="https://res.cloudinary.com/dilgcuzda/image/upload/v1713066505/thisweekinbevy/cratesio_m7edlj.avif"
                        class="w-8"
                    />
                    <span class="ml-4">crates.io</span>
                </>
            }),
            Domain::Docsrs => EitherOf8::H(view! {
                <>
                    <svg
                        class="w-8 fill-white"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 576 512"
                    >
                        <path d="m290.8 48.6 78.4 29.7-81.2 31.2-81.2-31.2 78.4-29.7c1.8-.7 3.8-.7 5.7 0zM136 92.5v112.2c-1.3.4-2.6.8-3.9 1.3l-96 36.4C14.4 250.6 0 271.5 0 294.7v119.2c0 22.2 13.1 42.3 33.5 51.3l96 42.2c14.4 6.3 30.7 6.3 45.1 0L288 457.5l113.5 49.9c14.4 6.3 30.7 6.3 45.1 0l96-42.2c20.3-8.9 33.5-29.1 33.5-51.3V294.7c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-1.3-.5-2.6-.9-3.9-1.3V92.5c0-23.3-14.4-44.1-36.1-52.4L308 3.7c-12.8-4.8-26.9-4.8-39.7 0l-96 36.4C150.4 48.4 136 69.3 136 92.5zm256 118.1-82.4 31.2v-89.2L392 121v89.6zm-237.2 40.3 78.4 29.7-81.2 31.1-81.2-31.1 78.4-29.7c1.8-.7 3.8-.7 5.7 0zm18.8 204.4V354.8l82.4-31.6v95.9l-82.4 36.2zm247.6-204.4c1.8-.7 3.8-.7 5.7 0l78.4 29.7-81.3 31.1-81.2-31.1 78.4-29.7zm102 170.3-77.6 34.1V354.8l82.4-31.6v90.7c0 3.2-1.9 6-4.8 7.3z"></path>
                    </svg>
                    <span class="ml-4">docs.rs</span>
                </>
            }),
        }.into_view()
    }
}

fn domain_heuristic(domain: &str) -> Option<Domain> {
    use url::Url;

    let parsed = Url::parse(domain);
    let host = parsed
        .as_ref()
        .ok()
        .and_then(|url| url.host_str())?;
    match host {
        "discord.com" => Some(Domain::Discord),
        "youtu.be" => Some(Domain::YouTube),
        "github.com" => Some(Domain::GitHub),
        "apps.apple.com" => Some(Domain::Apple),
        "mastodon.social" => Some(Domain::Mastodon),
        "hachyderm.io" => Some(Domain::Mastodon),
        "docs.rs" => Some(Domain::Docsrs),
        "crates.io" => Some(Domain::Cratesio),
        _ => None,
    }
    .or_else(|| {
        if host.ends_with(".itch.io") {
            Some(Domain::Itchio)
        } else {
            None
        }
    })
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod tests_server {

    use super::*;

    #[test]
    fn domain_heuristic_test() {
        let items = vec![
            "https://discord.com/channels",
            "https://irongremlin.itch.io/moar-ants",
            "https://apps.apple.com/app",
            "https://github.com/ddmills/boris",
            "https://ivanceras.github.io/globe/",
            "https://youtu.be/u1K3T5uzebE",
        ];

        let domains = vec![
            Some(Domain::Discord),
            Some(Domain::Itchio),
            Some(Domain::Apple),
            Some(Domain::GitHub),
            None,
            Some(Domain::YouTube),
        ];

        for (item, domain) in items.into_iter().zip(domains)
        {
            assert_eq!(domain_heuristic(item), domain);
        }
    }
}
