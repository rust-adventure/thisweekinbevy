= SQL Concepts

There are a few high-level concepts in the database.
These are long-lived "containers" which link to other "event"-style data.

- Issue
- Project
- Crate

== Issue

An issue of #link("https://thisweekinbevy.com/")[This Week in Bevy].
The intention is for these to be published weekly on Monday/Tuesday, which means the content cutoff is Saturday at 23:59.

Content-wise, an issue is a written post and a collection of different structured content items.

The description is a summary of recent PR merges, with explanations as to why they're interesting/what they are, links to interesting ecosystem events that don't fit in another category, etc.

Structured content includes:

- Showcases
- Crate Releases
- Educational
- Devlogs

== Project

A Project is a Game, Application, or higher-level project like Avian.
These include itch.io, steam, in-progress games, CAD applications, custom editors, etc.
Avian is an interesting case because it is both #link("https://github.com/avianphysics")[An Organization] *and* specific crates: `avian2d`/`avian3d`, `bevy_heavy`, `peck`, etc so fits into both categories with different entries.

== Crate

A Crate is a single crate or a collection of very related crates.

Examples of "crate collections":
- Avian, which has `avian2d` and `avian3d`
- crates that publish a single concept as a workspace of crates, or attempt to foster their own sub-ecosystem, similar to how `bevy` itself publishes 50+ crates as part of a release
  - These function as a loose "unit", for example while there are a number of `bevy_render` crates, (`bevy_sprite_render`, `bevy_gizmos_render`, etc) most are not used without the rest of bevy and thus don't make sense to display on their own.

Individual crates are relatively self-explanatory, ex: `glam`.

== Structured Content

=== Showcases

A Showcase is a YouTube video showing gameplay, posts in the `#showcase` channel in the Bevy discord, bluesky/mastodon/etc or other social media posts, etc.
These are intentionally shared by the projects, so we don't go diving into their individual chat channels to extract showcases.
Showcases must be published on a public channel (ex: youtube/bsky/reddit) or a channel that is intended to be shared with a wider audience (`#showcase`).

=== Crate Releases

Crate releases represent a weekly release roundup for a particular Crate, which are then shared in an Issue.
If a crate ships 5x in one week, that generally shows up as a single "Crate Release", which can then be associated with an Issue, Crate, and/or Project.

=== Educational

Tutorials, videos, blog posts, and other interesting educational content.

=== Devlogs

Blog posts or videos focusing on the continued development of a game or project.

== Metadata

There is various metadata associated with any other high-level or structured content item.
These are metadata tables which hold a specific data type and are accessed via a join table shaped like `(item_id, metadata_id)`.

- links
  - There is a lot of external linking going on; This is links to steam, itch, youtube, the bevy discord, bluesky, etc.
- images
  - Basically everything has optionally multiple images
- crates_io
  - crate_io data encapsulates *actual* crates.io release events and some associated metadata, like dependencies, etc. This is relevant to link in to other types, but not independently.
- opened issues
  - Every week some issues are opened and we display them in This Week in Bevy for anyone interested in picking one up.
- merged PRs
  - Every week new PRs are merged. Probably the *most* relevant of the "github metadata" tables each week.
- opened PRs
  - PRs that were opened this week, but were not merged yet and might need review.
