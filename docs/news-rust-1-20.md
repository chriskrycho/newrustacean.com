# Rust 1.20

## Intro

Hello, my name is Chris Krycho, and this is the New Rustacean podcast – a show about learning the Rust programming language. This is a News episode, for Rust 1.20.

## Rust proper

As the title of the episode suggests, Rust 1.20 came out today! That's a pretty amazing number if you take a step back and look at it. We've gotten used to this pace of releases in the Rust community, but it's still fabulous. In the last roughly 15 months, Rust has shipped 21 releases, starting with Rust 1.0, steadily improving the language and adding features without breaking backwards compatibility. In fact, if you go back and look at what Rust 1.0 looked like, you can see how it's both the same language as today's and yet a recognizably less mature language as well.

Starting with this episode, I'll be putting out news episodes to coincide with each minor Rust release. Every six weeks seems like a reasonable amount of time to cover with as much as is going on in the Rust world---not least because there's more happening all the time!

So, what's in Rust 1.20 itself?

### Associated constants

The first big thing is the stabilization of *associated constants*. These are like other associated *items* that already exist, specifically associated *types* and associated *functions*. "Associated functions" are quite ordinary in Rust: they're just static functions that belong to a given trait, struct, or enum. Associated types are bit more complex: they're types – kind of like, but not the same as, a generic type – which are associated with a given trait, and which you must specify for any implementation fo that trait. (If that went a little over your head, don't worry: I have a planned episode to talk about them in detail in the future.) "Associated constants" are more like "associated functions": they're just constant items which can be attached to any trait, struct, or enum.

In the case of a trait, you can define the constant item on the trait declaration itself, or on the `impl` block of a struct or enum. In the case of a trait, you can define the actual value, or you can leave it just a type and require the types which implement the trait to specify the value. And you can only use a trait constant with an actual implementation of the trait – you can not write a trait named `Foo` with a constant named `BAR` and then use it like `Foo::BAR`. But if you have a `Quux` which implements `Foo`, you *can* write `Quux::BAR` and it'll be what you expect.

All of this might seem relatively arcane, but it's quite useful for a number of things. The Rust 1.20 announcement blog post points out that it lets you define quite a few things in e.g. an trait for floating point numbers simply as part of the *floating point type itself*, rather than having to have standalone constants in a floating point module. Moreover, this is one of several building blocks for more advanced, compile-time "metaprogramming" – or, as you might also describe it in this case, programming via the types themselves and at compile time, rather than only with the runtime behavior of the programming. This is a concept that we'll come back to on the show in the future because it's of considerable interest to me, and also because it's of considerable interest to the broader Rust community and development teams.

### Other bits

Now, what about the other big things in Rust 1.20? Well, as with most Rust releases, there aren't a lot of big things! There are a bunch of small improvements and stabilizations in the standard library: as usual, these are just quality of life changes. But 20 releases worth of quality-of-life changes have added up quite a bit! A couple little tidbits that I thought were most interesting: the `unimplemented!()` macro now lets you specify *why* a function isn't implemented yet, so that when your program hits that point and dies, you can get an explanation printed to the console. Helpful when leaving notes for yourself! There are a couple handy new methods on the `Option` type, as well: `get_or_insert` and `get_or_insert_with`, which both supply mutable references to the item inside the `Option`, or insert a new one and get a mutable reference to *that* if the `Option` is `None`. Like I said: not game-changers, but the kinds of convenience methods that add up over time. There are a bunch of other changes like that, so I encourage you to take a look both at the release blog post and at the detailed change log for the release, both of which I've linked in the show notes.

## Community

Now, for something old and new again in the show! In the early days of the podcast, I often called out ongoing community news bits, new resources, and so on in the opening minutes of each episode. I eventually realized, however, that that stuff would get out of date in a way that much of the tutorial-type material wouldn't, and dropped it. However, I want to keep the ability for the show to highlight a bunch of different things going on in the community, and these every-6-weekly-episodes are the new home for those kinds of things!

### Podcasts

First up on that list, I'm delighted to say that this is once again *not the only podcast* in the Rust space! Back in mid-June, some other members of the community (the ringleaders seem to be Manish Goregaokar and Carol Nichols-or-Goulding, perhaps better known to you as "manishearth" and "carols-2-cents" where 2 is spelled in binary as 10) kicked off a new show called _Request for Explanation_. Every episode is a deep dive on some Rust RFC or another. If you want to hear about a specific RFC, or even to possibly be a guest on the show, you just file an issue on the GitHub repo where they're hosting the blog. You should listen! They're doing something *totally* different in the podcasting space than I am, and besides: more Rust podcasts is just more awesomeness as far as I'm concerned.

### Conferences

As I release this, we're right in the middle of Rust conference season!

- RustConf happened just a couple weeks ago, and videos will be coming out soon-ish. Last year the videos seem to have gone up about two months after the conference itself, and that seems to be about par for the course for other conferences I'm familiar with.

But don't worry -- if you missed it, there are still a couple conferences you can get to!

- RustFest is September 30--October 1 in Zürich, Switzerland. Tickets are still on sale, and the list of speakers is slowly being rolled out. I've never made it out there, but the attendees of the last few years have raved about it. The focus on Saturday is talks on everything from embedded hardware to testing and concurrency. On Sunday, the focus will be on learning and hack sessions! If you are in Europe, or Europe is easier for you to get to than the U.S., or you just feel like going to Europe would be awesome, you should get there!

- Rust Belt Rust is October 26--27 in Columbus, Ohia, in the U.S. It's organized by Carol Nichols-or-Goulding, and includes a keynote by several people on the core team: Carol herself, Aaron Turon, and Niko Matsakis. The full schedule was announced today and includes a full day of workshops on Thursday and a full day of presentations on Friday. And---I'm *incredibly* excited about this---I'll be giving one of those talks on Friday! So if you'd like to meet up with me, that's the place and time! There'll be another news episode out between now and then, and I'll say it again.

Related, I also hope (and will follow up with further info about) to have New Rustacean stickers available *at* the conference, and perhaps to make New Rustacean shirts available for purchase before then so we can be a whole troop of New Rustaceans.

### Meetups

I also want to draw your attention to two meetups people asked me to mention! Both of them, sadly, I was unable to get this episode out in time for this month. One is a NYC Rust meetup; they have a meeting tonight, August 31, and will have more meetings in the future. I don't have a long-term schedule for that meetup, but you can follow up with the organizer, Robert Balicki for more details.

The other one is the Triangle Rustaceans meetup, here in the Raleigh-Durham-Chapel Hill area in North Carolina. I've been delighted to attend two of the three meetings so far, and I'll be at one more, on September 25. That group meets on the fourth Monday of every month. If you live around here, or if you happen to be in the area then, you should definitely stop by! I'll be moving out of the area after that, but I also expect I *may* be helping start up a new meetup in the area I'm moving to (which, to my knowledge, doesn't have one  yet).

Don't worry: info about time and place for both of those is on meetup.com, and I'll have those in the show notes.

## Outro

And that's everything I've got for today. If you'd like your stuff to land in the next News episode when Rust 1.21 comes out in six weeks, send it my way on twitter @newrustacean or @chriskrycho, or send me an email at hello@newrustacean.com. I'll be back in early September with a Crates You Should Know, looking at easy data parallelism in Rust with Rayon.

Thanks to this month's $10-or-more sponsors:

- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Matt Rudder
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

As always, thanks as well to everyone who helps other people find the show---whether by just telling a friend, or by rating or reviewing it in iTunes or recommending it in another podcast directory, or by sharing it on social media!

You can find show notes for this episode, including links to blog posts and conferences and meetups and more, at `newrustacean.com/show_notes/news/rust_1_20/`.

Until next time, happy coding.
