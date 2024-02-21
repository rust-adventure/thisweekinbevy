use crate::app::components::Container;
use leptos::*;
pub mod index;
pub mod issue;
pub mod admin;

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
fn EpisodeEntry(episode: Episode) -> impl IntoView {

    view! {
      <article
        aria-labelledby=format!("episode-{}-title", episode.id)
        class="py-10 sm:py-12"
      >
        <Container>
          <div class="flex flex-col items-start">
            <h2
              id=format!("episode-{}-title", episode.id)
              class="mt-2 text-lg font-bold text-slate-900"
            >
              <a href=format!("/{}", episode.id)>{&episode.title}</a>
            </h2>
            // <FormattedDate
            //   date={date}
            //   class="order-first font-mono text-sm leading-7 text-slate-500"
            // />
            <p class="mt-1 text-base leading-7 text-slate-700">
              {episode.description}
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
                href=format!("/{}", episode.id)
                class="flex items-center text-sm font-bold leading-6 text-pink-500 hover:text-pink-700 active:text-pink-900"
                aria-label=format!("Show notes for episode {}", episode.title)
              >
                Show notes
              </a>
            </div>
          </div>
        </Container>
      </article>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    //   let episodes = await getAllEpisodes()
    let episodes = vec![Episode {
        id: "test data".to_string(),
        published: "test data".to_string(),
        title: "test data".to_string(),
        description: "test data".to_string(),
    }];

    view! {
      <div class="pb-12 pt-16 sm:pb-4 lg:pt-12">
        <Container>
          <h1 class="text-2xl font-bold leading-7 text-slate-900">
            Episodes
          </h1>
        </Container>
        <div class="divide-y divide-slate-100 sm:mt-4 lg:mt-8 lg:border-t lg:border-slate-100">
          {episodes
            .into_iter()
            .map(|episode|
                view!{
                    <EpisodeEntry episode={episode} />
                }
            )
            .collect::<Vec<_>>()}
        </div>
      </div>
    }
}

struct Episode {
    id: String,
    published: String,
    title: String,
    description: String,
}
