+++
title = "Shut-Up and Build: Music Tracker"
date = 2026-12-14
description = "An on-going series on how to not overthink and just build."
categories = ["Series", "Shut Up and Build"]
tags = ["music tracker"]
draft = true
+++

Yes, even Claude can lose patience.

I've lately been doing a lot of therapy with Claude. I know I shouldn't be, but it's a non-judgemental thing and it's beneficial to just have something to type at and get back some ideas an guidance.

I think I succeeded in pissing Claude off this week though, even it recognized and called me out on my on-going pattern of excuse making.

<!-- more -->

It challenged me to quit talking and just build. The proof I crave, that I can problem solve and build something real, will only come if I build *real things*. No more theory, no more navel gazing, no more waiting for the perfect words, perfect idea, whatever...

Just...fucking...build.

Ok Claude.

### What to Build?

For some time now, I've wanted an app to track my music listening habits. In the space of about 3 hours earlier this week, I built the skeleton and had something that I could record the date, artist, and album via cURL on the command line. It doesn't have a database, it's not fancy, but it was *useful*.

Claude challenged me to actually ship after I built, so I immediately deployed to my account on [Shuttle](https://shuttle.dev). Felt good.

I didn't overthink it, I didn't try to pull in the kitchen sink, I just built the barest, simplest thing that worked.

Tonight, I added a web form (was using Tera for HTML behind the scenes) and re-deployed!

I now have something I can go to on my phone and enter whatever album I listened to that day.

### Challenges

In the vein of simple, I didn't add a database. What I built doesn't have persistance, it's in-memory only. This is not great in the long run, but fine as the first iteration. I use a `struct` type called `MusicEntry`, which has date, artist, and album fields for holding data. The fields are just simple `String` types. I made a wrapper type called `MusicEntries` which basicalled holds `MusicEntry` in a `Vec` collection type. One of the challenges I encountered was getting my data structure shared properly in application state. I used `Axum` and I know how to use the `.with_state()` method on the `Router` type, but at first didn't realize how to set up state properly.

In my first go, basically every time a data was put in, it was put into a fresh instance of the application state. What I really wanted was for the *same* state to be updated.

Enter `Arc` and `Mutex`.

The Rust `Vec` type doesn't implement anything to help you in this effort. You need to wrap it in the `Arc` type, which enables cheap cloning. *Then* you use the `Mutex` type, which is one of Rust's concurrency tools, to make your type safe to modify across threads.

So, you need to initialize your type like this:

```rust
let music_entries = MusicEntries {
                        data: Arcnew(Mutex::new(Vec::new())),
};
```

Then, you add it to state normally, like this:

```rust
let router = Router::new()
                .route("/health_check", get(health_check))
                .route("/", get(get_index))
                .route("/new", post(post_new_music))
                .route("/music", get(get_all_music))
                .with_state(music_entries);
```

You can then pass state into an Axum handler to access it. In my case, the `POST` handler, which receives new entries from a web form, looks like:

```rust
// submission form handler - takes a new music entry as input - returns 200 OK and the entry
#[debug_handler]
async fn post_new_music(
     State(state): State<MusicEntries>,
     Form(new_music_entry): Form<MusicEntry>,
) -> Json<MusicEntry> {
     let new_entry = MusicEntry {
                      date: new_music_entry.date,
                      artist: new_music_entry.artist,
                      album: new_music_entry.album,
      };

      let mut data = state.data.lock().await;
      data.push(new_entry);
      let new = data.last().cloned().unwrap_or_default();
      Json(new)

}
```

This function receives the incoming form data, turns it into our *domain* type, `MusicEntry`, locks the state data (that's existing) so that the new data can be pushed to our `Vec` type, grabs the last entry (which will be equivalent to the data we just entered, and returns it as JSON.

There are some subtles I'm glossing over, but that's the basic idea.

## What's Next?

While it was very gratifying to do this, the app *isn't all that useful*. It needs more, a lot more.

- a database for persistance
- validation of incoming data
- less manual entry for the date field
- expand the front end to have a button to actually display *all* the music data.
- many other things...

Still, not a bad start.

I don't know that I full believe yet, but maybe Claude has a point.
