# Katas---learning by doing

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a show about learning the Rust programming language. This is *Bonus episode 7: Katas---learning by doing.*

## News and follow-up

First, an apology a bit of explanation for the delay between episodes! In the last 10 days, I've traveled something like 4,800 miles for a conference and a wedding. There's are a couple of reasons I'm not at this year's RustConf or Rust Belt Rust conferences, and one of the big ones is this is too much travel! Gladly, the rest of the year is *much* slower, and things should normalize pace-wise from here.

Now, a couple quick bits of news and updates: First, literally *as I record*, the first RustConf is going on! Video from the conference will be made available after the fact, and you can bet I'll be watching it. One particularly fun bit related to *this* show---as you may recall: Raph Levien is presenting on his work on the Xi editor. I'm particularly excited about that talk.

Second, a couple neat things from around the community more generally:

- I mentioned Claudia Doppioslash's work on gaming and Rust in a previous episode; now she and a few others have launched arewegameyet.com, which highlights the state of tools for building games in Rust.
- That reminds me: if you haven't yet, take a look at areweideyet.com and arewewebyet.org to see the state of IDE support and web development respectively in Rust.

Third, some updates from core:

- There's a great discussion around direction and vision for the Rust language in 2017 on the Rust internals forum, which I'll link. Lots of good discussion about where and how to focus efforts to improve the language's adoption---you should read it and chime in!
- Early, early, alpha-level support for incremental compilation has landed in the nightly builds of the compiler, and support for that with Cargo is being built out now---some of the long-expected fruits of MIR landing. That should help compile times a *lot*.

## Katas

- I've been interested in Elm for a while (and why: ML-like, use JS at work, guarantees around it, etc.)
- I've gone back to same problem in JS repeatedly
- Idea isn't new to me: "katas" are a thing out there
- The actual problem isn't hard, and that's a big part of what makes katas useful. The point isn't so much the problem---which you can figure out how to solve---as figuring out how to solve it *well* in *this* language.
- Going back to it as you become more familiar with the language, or as you pick up new ideas lets you apply newfound knowledge while not worrying about trying to understand the *problem*.
- So I did this "pizza test" with Elm. It took me a couple hours, because I was learning the language and its tooling and how the compiler works and so on
- Then I started in on it it on an even broader scale with Rust. Broader how?
    - Elm and JS are already in-browser setups. Rust is not.
    - Pull in a bunch of crates:
		- Hyper for HTTP requests
		- Serde for deserializing JSON
		- Pencil for serving the actual rendered page
		- handlebars-rust for simple HTML templates
	- Why? To get a feel for those crates as well as for what solving *these* problems in a Rustic way feels like, just as I was sorting through it in Elm.

The best ways to learn things in programming, in my experience are to *do* them, and to *teach* them. As I said in the very first episode of the show---almost a year ago, now!---that latter bit is one big reasons I'm making the podcast. It's helping *me* keep learning Rust while very busy with other things.

## Outro -- sponsors and following

Thanks as always to the sponsors who help make the show possible! This month, the sponsors included Chris Palmer, Daniel Collin, Raph Levien, Stephen Murawksi, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes, and the top tier of sponsors on a dedicated page on the website as well.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at a variety of other services listed at the show website: newrustacean.com, along with links to the bits I discussed on the episode.

You can also follow the show on Twitter \@newrustacean, or follow me there \@chriskrycho. If you enjoyed the show, please help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, sharing it online, or just telling a friend!

Also, please do respond on social media, in the threads for the episode on the Rust user forum or on Reddit, or via email at hello@newrustacean.com. I love hearing from you!

Until next time, happy coding!
