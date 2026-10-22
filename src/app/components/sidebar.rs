// use crate::components::Footer;
use leptos::prelude::*;

use crate::app::{GitHubIcon, RSSIcon, YouTubeIcon};

struct MenuItem<'a> {
    url: &'a str,
    shortcode: &'a str,
    label: &'a str,
    border_color: &'a str,
    icon: Option<Icon>,
}
const MENU_ITEMS: [MenuItem; 3] = [
    MenuItem {
        url: "/projects",
        shortcode: "P",
        label: "Projects",
        border_color: "border-[#38bdf8]",
        icon: None,
    },
    MenuItem {
        url: "/crates",
        shortcode: "C",
        label: "Crates",
        border_color: "border-[#38bdf8]",
        icon: Some(Icon::CratesIo),
    },
    MenuItem {
        url: "/pull-requests",
        shortcode: "PR",
        label: "Pull Requests",
        border_color: "border-[#ececec]",
        icon: Some(Icon::PullRequest),
    },
];

enum Icon {
    YouTube,
    GitHub,
    Rss,
    PullRequest,
    CratesIo,
    // CrateRelease
}
impl Icon {
    fn to_view(&self, class: String) -> impl IntoView {
        match self {
            Icon::YouTube => view! {
                <YouTubeIcon class/>
            }
            .into_any(),
            Icon::GitHub => view! {
                <GitHubIcon class=format!("{} fill-slate-400 group-hover:fill-slate-600", class)/>
            }
            .into_any(),
            Icon::Rss => view! {
                <RSSIcon class=format!("{} fill-white group-hover:fill-white", class)/>
            }
            .into_any(),
            Icon::CratesIo => view! {
                <img
                    src="/cargo.png"
                    class=format!("{} fill-white group-hover:fill-white", class)
                />
            }
            .into_any(),
            Icon::PullRequest => view! {
                <svg
                    aria-hidden="true"
                    focusable="false"
                    class=format!("{} fill-white group-hover:fill-white", class)
                    viewBox="0 0 16 16"
                    fill="currentColor"
                    display="inline-block"
                    overflow="visible"
                    style="vertical-align: text-bottom;"
                ><path d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Z"></path></svg>
            }.into_any()
            // Icon::CrateRelease => view! {
            // }.into_any()
        }
    }
}

const LINKS_ITEMS: [MenuItem; 3] = [
    MenuItem {
        url: "https://www.youtube.com/playlist?list=PLWtPciJ1UMuAyAER9ASVEDRIz0DUspOeZ",
        shortcode: "YT",
        label: "YouTube Playlist",
        border_color: "border-[#ff0000]",
        icon: Some(Icon::YouTube)
    },
    MenuItem {
        url: "/https://thisweekinbevy.com/feed.xml",
        shortcode: "A",
        label: "Atom Feed",
        border_color: "border-[#38bdf8]",
        icon: Some(Icon::Rss)
    },
    MenuItem {
        url: "https://github.com/rust-adventure/thisweekinbevy",
        shortcode: "GH",
        label: "GitHub",
        border_color: "border-[#E4EBE6]",
        icon: Some(Icon::GitHub)
    }
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum SidebarStatus {
    Open,
    Closed,
}

#[island]
pub fn Sidebar(
    // #[prop(optional)] header: Option<Header>,
    children: Children,
) -> impl IntoView {
    let (status, set_status) =
        signal(SidebarStatus::Closed);
    // let t = YouTubeIcon;
    view! {
        <div>

             <div
                class=move || format!("relative z-50 xl:hidden {}", match status() {
                    SidebarStatus::Open => "",
                    SidebarStatus::Closed => "hidden"
                })
                role="dialog"
                aria-modal="true"
             >
                <div class="fixed inset-0 bg-slate-50/80 dark:bg-slate-900/80" aria-hidden="true"></div>
                <div class="fixed inset-0 flex">
                    <div class="relative mr-16 flex w-full max-w-xs flex-1">
                         <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
                            <button
                                type="button"
                                class="-m-2.5 p-2.5"
                                on:click=move |_| set_status.set(SidebarStatus::Closed)
                            >
                                <span class="sr-only">Close sidebar</span>
                                <svg
                                    class="h-6 w-6 text-slate-950 dark:text-white"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    aria-hidden="true"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M6 18L18 6M6 6l12 12"
                                    ></path>
                                </svg>
                            </button>
                        </div>
                        <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-slate-100 dark:bg-slate-900 px-6 ring-1 ring-white/10">
                            <a href="/" class="flex h-16 shrink-0 items-center">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 286.38 43.69"
                                    fill="currentColor"
                                    class="h-8 w-auto text-sky-100"
                                >
                                    <path d="M.54 10.68V4.9h21.32v5.78zM7.79 29.1V4.9h6.8v24.21h-6.8ZM24 29.1V4.22h6.66v24.89H24Zm10.65 0v-9.38c0-.66-.19-1.16-.56-1.51s-.84-.53-1.41-.53c-.39 0-.73.08-1.04.24s-.54.39-.71.7-.25.67-.25 1.1l-2.58-1.16c0-1.36.28-2.53.83-3.5s1.33-1.72 2.31-2.24q1.485-.78 3.45-.78c1.965 0 2.54.27 3.54.8s1.76 1.26 2.29 2.18.8 1.98.8 3.18v10.91h-6.66ZM47.46 10.81c-.97 0-1.79-.33-2.43-1-.65-.67-.97-1.49-.97-2.46s.32-1.8.97-2.47 1.46-1 2.43-1 1.81.33 2.45 1c.63.67.95 1.49.95 2.47s-.32 1.8-.95 2.46c-.64.67-1.45 1-2.45 1M44.13 29.1V12.38h6.66v16.73h-6.66ZM60.62 29.61c-1.04 0-2.09-.14-3.13-.41s-2.01-.65-2.91-1.12c-.9-.48-1.65-1.02-2.26-1.63l3.5-3.6c.54.54 1.2.97 1.96 1.29q1.14.48 2.4.48.585 0 .9-.15c.21-.1.32-.24.32-.42 0-.32-.19-.56-.56-.71s-.85-.31-1.43-.44c-.58-.14-1.19-.31-1.84-.53s-1.26-.51-1.85-.88-1.07-.9-1.43-1.56c-.36-.67-.54-1.52-.54-2.57s.3-2.01.9-2.84 1.45-1.48 2.55-1.97 2.37-.73 3.82-.73 2.9.25 4.28.75 2.48 1.25 3.3 2.24l-3.54 3.6c-.57-.63-1.19-1.06-1.87-1.27-.68-.22-1.25-.32-1.7-.32s-.73.06-.9.19c-.17.12-.25.28-.25.46 0 .25.18.45.54.61s.83.31 1.41.44c.58.14 1.18.32 1.82.54.63.23 1.24.54 1.82.95s1.05.95 1.41 1.61c.36.67.54 1.54.54 2.6 0 1.65-.66 2.97-1.97 3.94s-3.08 1.46-5.3 1.46ZM82.86 29.1 75.24 4.89h6.77l5.37 18.63h-3.13l5.3-18.63h5.61l5.3 18.63h-3.09l5.37-18.63h6.7l-7.62 24.21h-5.64l-5.3-18.6h2.92l-5.3 18.6zM118.11 29.51c-1.95 0-3.66-.37-5.13-1.1s-2.62-1.77-3.45-3.09-1.24-2.85-1.24-4.57.4-3.21 1.21-4.52c.8-1.31 1.91-2.35 3.32-3.11s2.99-1.14 4.76-1.14 3.29.36 4.62 1.07c1.34.71 2.39 1.71 3.15 2.98q1.14 1.905 1.14 4.35c0 .34-.02.7-.05 1.07s-.11.81-.22 1.31l-15.1.1v-4.08l12.61-.1-3.09 1.8c0-.91-.11-1.65-.32-2.23-.22-.58-.54-1.01-.97-1.31-.43-.29-.98-.44-1.63-.44s-1.27.17-1.77.51-.88.83-1.14 1.46-.39 1.41-.39 2.31.14 1.71.43 2.33c.28.62.7 1.09 1.24 1.41s1.22.48 2.04.48 1.49-.13 2.09-.39 1.15-.67 1.65-1.24l3.33 3.3c-.84.95-1.85 1.67-3.03 2.14s-2.53.71-4.05.71ZM137.19 29.51c-1.95 0-3.66-.37-5.13-1.1s-2.62-1.77-3.45-3.09-1.24-2.85-1.24-4.57.4-3.21 1.21-4.52c.8-1.31 1.91-2.35 3.32-3.11s2.99-1.14 4.76-1.14 3.29.36 4.62 1.07c1.34.71 2.39 1.71 3.15 2.98q1.14 1.905 1.14 4.35c0 .34-.02.7-.05 1.07s-.11.81-.22 1.31l-15.1.1v-4.08l12.61-.1-3.09 1.8c0-.91-.11-1.65-.32-2.23-.22-.58-.54-1.01-.97-1.31-.43-.29-.98-.44-1.63-.44s-1.27.17-1.77.51-.88.83-1.14 1.46-.39 1.41-.39 2.31.14 1.71.43 2.33c.28.62.7 1.09 1.24 1.41s1.22.48 2.04.48 1.49-.13 2.09-.39 1.15-.67 1.65-1.24l3.33 3.3c-.84.95-1.85 1.67-3.03 2.14s-2.53.71-4.05.71ZM147.56 29.1V4.22h6.66v24.89h-6.66Zm10.78 0-4.66-8.81 4.62-7.92h7.17l-5.98 8.94.17-2.18 6.15 9.96h-7.48ZM176.7 10.81c-.98 0-1.79-.33-2.43-1-.65-.67-.97-1.49-.97-2.46s.32-1.8.97-2.47 1.46-1 2.43-1 1.81.33 2.45 1c.63.67.95 1.49.95 2.47s-.32 1.8-.95 2.46c-.63.67-1.45 1-2.45 1m-3.34 18.29V12.38h6.66v16.73h-6.66ZM183.19 29.1V12.38h6.66v16.73h-6.66Zm10.64 0v-9.38c0-.66-.19-1.16-.56-1.51s-.84-.53-1.41-.53c-.39 0-.73.08-1.04.24s-.54.39-.71.7-.25.67-.25 1.1l-2.58-1.16c0-1.36.3-2.53.9-3.5s1.43-1.72 2.48-2.24 2.25-.78 3.59-.78c1.2 0 2.27.27 3.21.8s1.68 1.26 2.23 2.18c.54.92.82 1.98.82 3.18v10.91h-6.66ZM209.88 29.1V4.9h6.66v24.21h-6.66Zm5.51 0v-4.9h4.45c.86 0 1.55-.23 2.07-.7.52-.46.78-1.11.78-1.92q0-.81-.36-1.41c-.36-.6-.57-.7-.99-.9s-.92-.31-1.51-.31h-4.45v-4.69h4.15c.72 0 1.33-.18 1.8-.54.48-.36.71-.93.71-1.7s-.24-1.34-.71-1.7q-.72-.54-1.8-.54h-4.15v-4.9h6.05c1.47 0 2.76.28 3.86.83 1.1.56 1.96 1.31 2.57 2.28.61.96.92 2.07.92 3.31 0 1.72-.61 3.13-1.84 4.22-1.22 1.09-3.01 1.71-5.37 1.87l-.07-2.35c2.56.16 4.52.86 5.88 2.11s2.04 2.84 2.04 4.79q0 2.115-1.05 3.72-1.05 1.62-2.94 2.52c-1.26.6-2.73.9-4.4.9h-5.64ZM240.58 29.51c-1.95 0-3.66-.37-5.13-1.1s-2.62-1.77-3.45-3.09-1.24-2.85-1.24-4.57.4-3.21 1.21-4.52c.8-1.31 1.91-2.35 3.32-3.11s2.99-1.14 4.76-1.14 3.29.36 4.62 1.07c1.34.71 2.39 1.71 3.15 2.98q1.14 1.905 1.14 4.35c0 .34-.02.7-.05 1.07s-.11.81-.22 1.31l-15.1.1v-4.08l12.61-.1-3.09 1.8c0-.91-.11-1.65-.32-2.23-.22-.58-.54-1.01-.97-1.31-.43-.29-.98-.44-1.63-.44s-1.27.17-1.77.51-.88.83-1.14 1.46-.39 1.41-.39 2.31.14 1.71.43 2.33c.28.62.7 1.09 1.24 1.41s1.22.48 2.04.48 1.49-.13 2.09-.39 1.15-.67 1.65-1.24l3.33 3.3c-.84.95-1.85 1.67-3.03 2.14s-2.53.71-4.05.71ZM255.54 29.1l-6.56-16.73h7.07l3.84 14.25h-2.41l3.81-14.25h7.04l-6.56 16.73h-6.22Z"/>
                                    <path d="m274.1 29.1-6.6-16.73h7.07l3.88 14.25h-2.92l3.81-14.25h7.04l-6.56 16.73h-5.71Zm-3.63 7.14 4.86-10.98 4.49 3.84-2.82 7.14z"/>
                                </svg>
                            </a>
                            <div>
                            <p class="font-medium text-slate-200">A curated roundup covering week-to-week activity in the Bevy ecosystem</p>
                            </div>
                            <nav class="flex flex-1 flex-col">
                                <ul role="list" class="flex flex-1 flex-col gap-y-7">
                                    <li>
                                        <ul role="list" class="-mx-2 space-y-1">
                                        <li>
                                            <a
                                                href="/issues"
                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                            >
                                            <svg
                                                class="h-6 w-6 shrink-0 text-sky-400"
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke-width="1.5"
                                                stroke="currentColor"
                                                class="size-6"
                                            >
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 7.5h1.5m-1.5 3h1.5m-7.5 3h7.5m-7.5 3h7.5m3-9h3.375c.621 0 1.125.504 1.125 1.125V18a2.25 2.25 0 0 1-2.25 2.25M16.5 7.5V18a2.25 2.25 0 0 0 2.25 2.25M16.5 7.5V4.875c0-.621-.504-1.125-1.125-1.125H4.125C3.504 3.75 3 4.254 3 4.875V18a2.25 2.25 0 0 0 2.25 2.25h13.5M6 7.5h3v3H6v-3Z" />
                                            </svg>
                                                Issues
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/showcases"
                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                            >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke-width="1.5"
                                                stroke="currentColor"
                                                class="h-6 w-6 shrink-0 text-sky-400"
                                            >
                                                <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
                                            </svg>
                                                Showcases
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/crate_releases"
                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                            >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke-width="1.5"
                                                stroke="currentColor"
                                                class="h-6 w-6 shrink-0 text-sky-400"
                                            >
                                                <path stroke-linecap="round" stroke-linejoin="round" d="m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9" />
                                            </svg>
                                                Crate Releases
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/educational"
                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                            >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke-width="1.5"
                                                stroke="currentColor"
                                                class="h-6 w-6 shrink-0 text-sky-400"
                                            >
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.26 10.147a60.438 60.438 0 0 0-.491 6.347A48.62 48.62 0 0 1 12 20.904a48.62 48.62 0 0 1 8.232-4.41 60.46 60.46 0 0 0-.491-6.347m-15.482 0a50.636 50.636 0 0 0-2.658-.813A59.906 59.906 0 0 1 12 3.493a59.903 59.903 0 0 1 10.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.717 50.717 0 0 1 12 13.489a50.702 50.702 0 0 1 7.74-3.342M6.75 15a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm0 0v-3.675A55.378 55.378 0 0 1 12 8.443m-7.007 11.55A5.981 5.981 0 0 0 6.75 15.75v-1.5" />
                                            </svg>
                                                Educational
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/devlogs"
                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                            >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke-width="1.5"
                                                stroke="currentColor"
                                                class="h-6 w-6 shrink-0 text-sky-400"
                                            >
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 20.25h12m-7.5-3v3m3-3v3m-10.125-3h17.25c.621 0 1.125-.504 1.125-1.125V4.875c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125Z" />
                                            </svg>
                                                Devlogs
                                            </a>
                                        </li>
                                        </ul>
                                    </li>
                                    <li>
                                        <div class="text-xs font-semibold leading-6 text-slate-600 dark:text-slate-400">
                                            Shortcuts
                                        </div>
                                        <ul role="list" class="-mx-2 mt-2 space-y-1">
                                            <For
                                                // a function that returns the items we're iterating over; a signal is fine
                                                each=move || MENU_ITEMS
                                                // a unique key for each item
                                                key=|menu_item| menu_item.label.to_string()
                                                // renders each item to a view
                                                children=move |item| {
                                                    view! {
                                                        <li>
                                                            <a
                                                                href=item.url
                                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-700 dark:text-slate-200 hover:bg-white hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-white"
                                                            >
                                                            {
                                                                match item.icon {
                                                                    None => view! {
                                                                        <span class=format!(
                                                                            "flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border bg-slate-50 dark:bg-slate-800 text-[0.625rem] font-medium text-slate-800 dark:text-slate-400 group-hover:text-slate-900 dark:group-hover:text-white {}",
                                                                            item.border_color
                                                                        )>
                                                                            {item.shortcode}
                                                                        </span>
                                                                    }.into_any(),
                                                                    Some(icon) => {
                                                                        icon.to_view("h-6 w-6".to_string()).into_any()
                                                                    }
                                                                }
                                                            }
                                                                <span class="truncate">{item.label}</span>
                                                            </a>
                                                        </li>
                                                    }
                                                }
                                            />

                                        </ul>
                                    </li>
                                    <li>
                                        <div class="text-xs font-semibold leading-6 text-slate-600 dark:text-slate-400">
                                            Links
                                        </div>
                                        <ul role="list" class="-mx-2 mt-2 space-y-1">
                                            <For
                                                // a function that returns the items we're iterating over; a signal is fine
                                                each=move || LINKS_ITEMS
                                                // a unique key for each item
                                                key=|menu_item| menu_item.shortcode.to_string()
                                                // renders each item to a view
                                                children=move |item| {
                                                    view! {
                                                        <li>
                                                            <a
                                                                href=item.url
                                                                class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-700 dark:text-slate-200 hover:bg-white hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-white"
                                                            >
                                                                {match item.icon {
                                                                    Some(icon) => icon.to_view("h-6 w-6 shrink-0".to_string()).into_any(),
                                                                    None => view! {
                                                                        <span class=format!(
                                                                            "flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border bg-slate-50 dark:bg-slate-800 text-[0.625rem] font-medium text-slate-800 dark:text-slate-200 group-hover:text-slate-900 dark:group-hover:text-white {}",
                                                                            item.border_color
                                                                        )>
                                                                            {item.shortcode}
                                                                        </span>
                                                                    }.into_any()
                                                                }}
                                                                <span class="truncate">{item.label}</span>
                                                            </a>
                                                        </li>
                                                    }
                                                }
                                            />

                                        </ul>
                                    </li>
                                    <li class="-mx-6 mt-auto">
                                        <a
                                            href="https://bsky.app/profile/chrisbiscardi.bsky.social"
                                            class="flex items-center gap-x-4 px-6 py-3 text-sm font-semibold leading-6 text-slate-950 dark:text-white hover:bg-slate-white dark:hover:bg-slate-800"
                                        >
                                            <img
                                                class="h-6 w-6 rounded-full bg-slate-50 dark:bg-slate-800"
                                                src="/logos/logo-circle.svg"
                                                alt=""
                                            />
                                            <span class="sr-only">Links</span>
                                            <span aria-hidden="true">@chrisbiscardi</span>
                                        </a>
                                    </li>
                                </ul>
                            </nav>
                        </div>
                    </div>
                </div>
            </div>

            <div class="hidden xl:fixed xl:inset-y-0 xl:z-50 xl:flex xl:w-72 xl:flex-col border-r border-slate-800">


                <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-slate-950/10 px-6 ring-1 ring-white/5">
                    <a href="/" class="flex h-16 shrink-0 items-center" >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 286.38 43.69"
                            fill="currentColor"
                            class="h-8 w-auto text-sky-100"
                        >
                            <path d="M.54 10.68V4.9h21.32v5.78zM7.79 29.1V4.9h6.8v24.21h-6.8ZM24 29.1V4.22h6.66v24.89H24Zm10.65 0v-9.38c0-.66-.19-1.16-.56-1.51s-.84-.53-1.41-.53c-.39 0-.73.08-1.04.24s-.54.39-.71.7-.25.67-.25 1.1l-2.58-1.16c0-1.36.28-2.53.83-3.5s1.33-1.72 2.31-2.24q1.485-.78 3.45-.78c1.965 0 2.54.27 3.54.8s1.76 1.26 2.29 2.18.8 1.98.8 3.18v10.91h-6.66ZM47.46 10.81c-.97 0-1.79-.33-2.43-1-.65-.67-.97-1.49-.97-2.46s.32-1.8.97-2.47 1.46-1 2.43-1 1.81.33 2.45 1c.63.67.95 1.49.95 2.47s-.32 1.8-.95 2.46c-.64.67-1.45 1-2.45 1M44.13 29.1V12.38h6.66v16.73h-6.66ZM60.62 29.61c-1.04 0-2.09-.14-3.13-.41s-2.01-.65-2.91-1.12c-.9-.48-1.65-1.02-2.26-1.63l3.5-3.6c.54.54 1.2.97 1.96 1.29q1.14.48 2.4.48.585 0 .9-.15c.21-.1.32-.24.32-.42 0-.32-.19-.56-.56-.71s-.85-.31-1.43-.44c-.58-.14-1.19-.31-1.84-.53s-1.26-.51-1.85-.88-1.07-.9-1.43-1.56c-.36-.67-.54-1.52-.54-2.57s.3-2.01.9-2.84 1.45-1.48 2.55-1.97 2.37-.73 3.82-.73 2.9.25 4.28.75 2.48 1.25 3.3 2.24l-3.54 3.6c-.57-.63-1.19-1.06-1.87-1.27-.68-.22-1.25-.32-1.7-.32s-.73.06-.9.19c-.17.12-.25.28-.25.46 0 .25.18.45.54.61s.83.31 1.41.44c.58.14 1.18.32 1.82.54.63.23 1.24.54 1.82.95s1.05.95 1.41 1.61c.36.67.54 1.54.54 2.6 0 1.65-.66 2.97-1.97 3.94s-3.08 1.46-5.3 1.46ZM82.86 29.1 75.24 4.89h6.77l5.37 18.63h-3.13l5.3-18.63h5.61l5.3 18.63h-3.09l5.37-18.63h6.7l-7.62 24.21h-5.64l-5.3-18.6h2.92l-5.3 18.6zM118.11 29.51c-1.95 0-3.66-.37-5.13-1.1s-2.62-1.77-3.45-3.09-1.24-2.85-1.24-4.57.4-3.21 1.21-4.52c.8-1.31 1.91-2.35 3.32-3.11s2.99-1.14 4.76-1.14 3.29.36 4.62 1.07c1.34.71 2.39 1.71 3.15 2.98q1.14 1.905 1.14 4.35c0 .34-.02.7-.05 1.07s-.11.81-.22 1.31l-15.1.1v-4.08l12.61-.1-3.09 1.8c0-.91-.11-1.65-.32-2.23-.22-.58-.54-1.01-.97-1.31-.43-.29-.98-.44-1.63-.44s-1.27.17-1.77.51-.88.83-1.14 1.46-.39 1.41-.39 2.31.14 1.71.43 2.33c.28.62.7 1.09 1.24 1.41s1.22.48 2.04.48 1.49-.13 2.09-.39 1.15-.67 1.65-1.24l3.33 3.3c-.84.95-1.85 1.67-3.03 2.14s-2.53.71-4.05.71ZM137.19 29.51c-1.95 0-3.66-.37-5.13-1.1s-2.62-1.77-3.45-3.09-1.24-2.85-1.24-4.57.4-3.21 1.21-4.52c.8-1.31 1.91-2.35 3.32-3.11s2.99-1.14 4.76-1.14 3.29.36 4.62 1.07c1.34.71 2.39 1.71 3.15 2.98q1.14 1.905 1.14 4.35c0 .34-.02.7-.05 1.07s-.11.81-.22 1.31l-15.1.1v-4.08l12.61-.1-3.09 1.8c0-.91-.11-1.65-.32-2.23-.22-.58-.54-1.01-.97-1.31-.43-.29-.98-.44-1.63-.44s-1.27.17-1.77.51-.88.83-1.14 1.46-.39 1.41-.39 2.31.14 1.71.43 2.33c.28.62.7 1.09 1.24 1.41s1.22.48 2.04.48 1.49-.13 2.09-.39 1.15-.67 1.65-1.24l3.33 3.3c-.84.95-1.85 1.67-3.03 2.14s-2.53.71-4.05.71ZM147.56 29.1V4.22h6.66v24.89h-6.66Zm10.78 0-4.66-8.81 4.62-7.92h7.17l-5.98 8.94.17-2.18 6.15 9.96h-7.48ZM176.7 10.81c-.98 0-1.79-.33-2.43-1-.65-.67-.97-1.49-.97-2.46s.32-1.8.97-2.47 1.46-1 2.43-1 1.81.33 2.45 1c.63.67.95 1.49.95 2.47s-.32 1.8-.95 2.46c-.63.67-1.45 1-2.45 1m-3.34 18.29V12.38h6.66v16.73h-6.66ZM183.19 29.1V12.38h6.66v16.73h-6.66Zm10.64 0v-9.38c0-.66-.19-1.16-.56-1.51s-.84-.53-1.41-.53c-.39 0-.73.08-1.04.24s-.54.39-.71.7-.25.67-.25 1.1l-2.58-1.16c0-1.36.3-2.53.9-3.5s1.43-1.72 2.48-2.24 2.25-.78 3.59-.78c1.2 0 2.27.27 3.21.8s1.68 1.26 2.23 2.18c.54.92.82 1.98.82 3.18v10.91h-6.66ZM209.88 29.1V4.9h6.66v24.21h-6.66Zm5.51 0v-4.9h4.45c.86 0 1.55-.23 2.07-.7.52-.46.78-1.11.78-1.92q0-.81-.36-1.41c-.36-.6-.57-.7-.99-.9s-.92-.31-1.51-.31h-4.45v-4.69h4.15c.72 0 1.33-.18 1.8-.54.48-.36.71-.93.71-1.7s-.24-1.34-.71-1.7q-.72-.54-1.8-.54h-4.15v-4.9h6.05c1.47 0 2.76.28 3.86.83 1.1.56 1.96 1.31 2.57 2.28.61.96.92 2.07.92 3.31 0 1.72-.61 3.13-1.84 4.22-1.22 1.09-3.01 1.71-5.37 1.87l-.07-2.35c2.56.16 4.52.86 5.88 2.11s2.04 2.84 2.04 4.79q0 2.115-1.05 3.72-1.05 1.62-2.94 2.52c-1.26.6-2.73.9-4.4.9h-5.64ZM240.58 29.51c-1.95 0-3.66-.37-5.13-1.1s-2.62-1.77-3.45-3.09-1.24-2.85-1.24-4.57.4-3.21 1.21-4.52c.8-1.31 1.91-2.35 3.32-3.11s2.99-1.14 4.76-1.14 3.29.36 4.62 1.07c1.34.71 2.39 1.71 3.15 2.98q1.14 1.905 1.14 4.35c0 .34-.02.7-.05 1.07s-.11.81-.22 1.31l-15.1.1v-4.08l12.61-.1-3.09 1.8c0-.91-.11-1.65-.32-2.23-.22-.58-.54-1.01-.97-1.31-.43-.29-.98-.44-1.63-.44s-1.27.17-1.77.51-.88.83-1.14 1.46-.39 1.41-.39 2.31.14 1.71.43 2.33c.28.62.7 1.09 1.24 1.41s1.22.48 2.04.48 1.49-.13 2.09-.39 1.15-.67 1.65-1.24l3.33 3.3c-.84.95-1.85 1.67-3.03 2.14s-2.53.71-4.05.71ZM255.54 29.1l-6.56-16.73h7.07l3.84 14.25h-2.41l3.81-14.25h7.04l-6.56 16.73h-6.22Z"/>
                            <path d="m274.1 29.1-6.6-16.73h7.07l3.88 14.25h-2.92l3.81-14.25h7.04l-6.56 16.73h-5.71Zm-3.63 7.14 4.86-10.98 4.49 3.84-2.82 7.14z"/>
                        </svg>
                    </a>
                    <div>
                        <p class="font-medium text-slate-200">A curated roundup covering week-to-week activity in the Bevy ecosystem</p>
                    </div>
                    <nav class="flex flex-1 flex-col">
                        <ul role="list" class="flex flex-1 flex-col gap-y-7">
                            <li>
                                <ul role="list" class="-mx-2 space-y-1">
                                    <li>
                                        <a
                                            href="/issues"
                                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                        >
                                        <svg
                                            class="h-6 w-6 shrink-0 text-sky-400"
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="size-6"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 7.5h1.5m-1.5 3h1.5m-7.5 3h7.5m-7.5 3h7.5m3-9h3.375c.621 0 1.125.504 1.125 1.125V18a2.25 2.25 0 0 1-2.25 2.25M16.5 7.5V18a2.25 2.25 0 0 0 2.25 2.25M16.5 7.5V4.875c0-.621-.504-1.125-1.125-1.125H4.125C3.504 3.75 3 4.254 3 4.875V18a2.25 2.25 0 0 0 2.25 2.25h13.5M6 7.5h3v3H6v-3Z" />
                                        </svg>
                                            Issues
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            href="/showcases"
                                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                        >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="h-6 w-6 shrink-0 text-sky-400"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
                                        </svg>
                                            Showcases
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            href="/crate_releases"
                                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                        >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="h-6 w-6 shrink-0 text-sky-400"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9" />
                                        </svg>
                                            Crate Releases
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            href="/educational"
                                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                        >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="h-6 w-6 shrink-0 text-sky-400"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.26 10.147a60.438 60.438 0 0 0-.491 6.347A48.62 48.62 0 0 1 12 20.904a48.62 48.62 0 0 1 8.232-4.41 60.46 60.46 0 0 0-.491-6.347m-15.482 0a50.636 50.636 0 0 0-2.658-.813A59.906 59.906 0 0 1 12 3.493a59.903 59.903 0 0 1 10.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.717 50.717 0 0 1 12 13.489a50.702 50.702 0 0 1 7.74-3.342M6.75 15a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm0 0v-3.675A55.378 55.378 0 0 1 12 8.443m-7.007 11.55A5.981 5.981 0 0 0 6.75 15.75v-1.5" />
                                        </svg>
                                            Educational
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            href="/devlogs"
                                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white"
                                        >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="h-6 w-6 shrink-0 text-sky-400"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 20.25h12m-7.5-3v3m3-3v3m-10.125-3h17.25c.621 0 1.125-.504 1.125-1.125V4.875c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125Z" />
                                        </svg>
                                            Devlogs
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li>
                                <div class="text-xs font-semibold leading-6 text-slate-400">
                                    Shortcuts
                                </div>
                                <ul role="list" class="-mx-2 mt-2 space-y-1">
                                    <For
                                        // a function that returns the items we're iterating over; a signal is fine
                                        each=move || MENU_ITEMS
                                        // a unique key for each item
                                        key=|menu_item| menu_item.shortcode.to_string()
                                        // renders each item to a view
                                        children=move |item| {
                                            view! {
                                                <li>
                                                    <a
                                                        href=item.url
                                                        class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-700 dark:text-slate-200 hover:bg-white hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-white"
                                                    >
                                                    {
                                                        match item.icon {
                                                            None => view! {
                                                                <span class=format!(
                                                                    "flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border bg-slate-50 dark:bg-slate-800 text-[0.625rem] font-medium text-slate-800 dark:text-slate-400 group-hover:text-slate-900 dark:group-hover:text-white {}",
                                                                    item.border_color
                                                                )>
                                                                    {item.shortcode}
                                                                </span>
                                                            }.into_any(),
                                                            Some(icon) => {
                                                                icon.to_view("h-6 w-6".to_string()).into_any()
                                                            }
                                                        }
                                                    }
                                                        <span class="truncate">{item.label}</span>
                                                    </a>
                                                </li>
                                            }
                                        }
                                    />

                                </ul>
                            </li>
                            <li>
                                <div class="text-xs font-semibold leading-6 text-slate-400">
                                    Links
                                </div>
                                <ul role="list" class="-mx-2 mt-2 space-y-1">
                                    <For
                                        // a function that returns the items we're iterating over; a signal is fine
                                        each=move || LINKS_ITEMS
                                        // a unique key for each item
                                        key=|menu_item| menu_item.shortcode.to_string()
                                        // renders each item to a view
                                        children=move |item| {
                                            view! {
                                                <li>
                                                    <a
                                                        href=item.url
                                                        class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-slate-700 dark:text-slate-200 hover:bg-white hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-white"
                                                    >
                                                        {match item.icon {
                                                            Some(icon) => icon.to_view("h-6 w-6 shrink-0".to_string()).into_any(),
                                                            None => view! {
                                                                <span class=format!(
                                                                    "flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border bg-slate-50 dark:bg-slate-800 text-[0.625rem] font-medium text-slate-800 dark:text-slate-200 group-hover:text-slate-900 dark:group-hover:text-white {}",
                                                                    item.border_color
                                                                )>
                                                                    {item.shortcode}
                                                                </span>
                                                            }.into_any()
                                                        }}
                                                        <span class="truncate">{item.label}</span>
                                                    </a>
                                                </li>
                                            }
                                        }
                                    />

                                </ul>
                            </li>
                            <li class="-mx-6 mt-auto">
                                <a
                                    href="#"
                                    class="flex items-center gap-x-4 px-6 py-3 text-sm font-semibold leading-6 text-slate-950 dark:text-white hover:bg-slate-white dark:hover:bg-slate-800"
                                >
                                    // <img
                                    //     class="h-6 w-6 rounded-full bg-slate-50 dark:bg-slate-900"
                                    //     src="/logos/logo-circle.svg"
                                    //     alt=""
                                    // />
                                    <span class="sr-only">Links</span>
                                    <span aria-hidden="true">@chrisbiscardi</span>
                                </a>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
            <div class="xl:pl-72 flex flex-col">
                <div class="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-6 border-b border-slate-950/5 bg-white dark:border-white/5 dark:bg-slate-900 px-4 shadow-sm sm:px-6 lg:px-8">
                    <button type="button" class="-m-2.5 p-2.5 text-white xl:hidden"
                        on:click=move |_| set_status.set(SidebarStatus::Open)
                    >
                        <span class="sr-only">Open sidebar</span>
                        <svg
                            class="h-5 w-5 text-slate-950 dark:text-white"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10zm0 5.25a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </button>

                    // <div class="flex flex-1 gap-x-4 self-stretch lg:gap-x-6">
                    //     <form class="flex flex-1" action="#" method="GET">
                    //         <label for="search-field" class="sr-only">
                    //             Search
                    //         </label>
                    //         <div class="relative w-full">
                    //             <svg
                    //                 class="pointer-events-none absolute inset-y-0 left-0 h-full w-5 text-slate-500"
                    //                 viewBox="0 0 20 20"
                    //                 fill="currentColor"
                    //                 aria-hidden="true"
                    //             >
                    //                 <path
                    //                     fill-rule="evenodd"
                    //                     d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                    //                     clip-rule="evenodd"
                    //                 ></path>
                    //             </svg>
                    //             <input
                    //                 id="search-field"
                    //                 class="block h-full w-full border-0 bg-transparent py-0 pl-8 pr-0 text-white focus:ring-0 sm:text-sm"
                    //                 placeholder="Search..."
                    //                 type="search"
                    //                 name="search"
                    //             />
                    //         </div>
                    //     </form>
                    // </div>
                </div>
                <main class="relative textured-bg flex-1">
                    // {header.map(|header_inner| (header_inner.children)())}
                    {children()}
                </main>
                // <Footer/>
            </div>
        </div>
    }
}
