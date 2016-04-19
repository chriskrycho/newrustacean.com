# We *can* have nice things.

## Intro {>> 15s <<}

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a show about learning the Rust programming language. This is *Bonus Episode 4: We **can** have nice things.*

## News and Follow-up {>> 4m <<}

First up, some news---and there's a bunch of it this week!

Rust 1.8 was release this week! It's been just under a year since 1.0 came out, and in that time we've seen fairly dramatic movement forward in the language, but without any substantive breaking changes. Just about any package that built on 1.0 still builds, despite the language having gained a lot of new capabilities. I *love* that.

There are a few nice changes in Rust 1.8:

1. The plus-equals and minus-equals and similar operators are now overloadable via the op-assign family of traits, like `AddAssign`. That's a small but very nice improvement; getting to a point where all those kinds of things are extensible via traits makes for a more consistent, and more robust, programming model for Rust.
2. The first steps toward building Rust itself with Cargo, instead of with Make, are now on stable. This is bootstrapping taken to another level entirely: we're headed toward a point when Rust itself will be used for all the pieces around building Rust. If you're worried about not-invented-here syndrome, that's a good caution, but having experienced quite a few different package managers and tools lately... I think this is a good move. Make is a great tool, but Cargo is a much better one for these purposes, and the combination of Cargo and crates.io is *really* good---far better than pretty much anything else out there.
3. In just a few release cycles, Rust will no longer require a special nightly build of Rust to build itself: it will always be built using the previous version of the compiler, so 1.10 will be built with 1.9, and 1.11 with 1.10, and so on.
4. There's a new tool for managing Rust installations currently in beta. It's called "rustup", just like the current standard install script, but it's actually a pure-Rust implementation of the multirust tool. If you're familiar with tools like rbenv, pyenv, nvm, etc., this is that kind of tool: it allows you to manage multiple installations of your language, and set local directory rules for which version to use, and so on. Try it out!

Second, a quick note on scheduling: the last few weeks have been really, *really* busy for me between work, my Master's degree, and some family stuff. But I've had Rust on my mind a *lot*, between watching the 1.8 release come out, watching Sean Griffin live-code on Diesel a bit, and especially by way of comparison with all the JavaScript I'm writing for my day job. So here's a bonus episode, with the Strings episode to be recorded this coming weekend!

## Body {>> 7m <<}

I've been writing a ton of EmberJS for my day job, and I keep looking around for something *like Rust* in the JavaScript world. There are plenty of compiled-to-JavaScript languages out there, from CoffeeScript (which kind of kicked off that whole trend); to things like TypeScript, which shows the influence of the .NET team and C# pretty heavily; to Elm and PureScript, which are heavily inspired by Haskell. But none of them have the really wonderful mix of pragmatism and elegance that seems to characterize Rust to me. Rust is a fairly unique mix, with its extremely capable type system, performance, and what we often call *ergonomics*---how it feels to use. It's no one thing: it's syntax and semantics; it's a strong, *expressive* type system; it's the lack of nulls; it's powerful enumerations; it's trait-oriented instead of inheritance-oriented polymorphism; it's the whole package together. Basically, every day I write JavaScript and look at the alternatives, or even fairly solid other languages like C#, the more impressed I am by Rust.

People often wonder what the best way to learn a programming language is. The common answer is "just use it," of course: but how? How do you *start*, especially if it's not something you can use day-to-day for work, and if you don't have a specific project in mind for it? One of the best answers is: use it to build all the little tools you need for *other* projects. A colleague noted that the way he learned F# in a way that *stuck* was using it with the Fake tool to write (and now maintain) a bunch of our in-house tooling. You can do the same thing with Rust. Just write *something* in it. Because as much as Rust gets pegged as a "systems programming language" because it has the ability to do low-level work, the truth is that it's also just really good in its own right for *general* programming tasks.

Over the weekend, I saw some posts on Hacker News about the Rust 1.8 release which hit on this same theme. Several users noted that they'd been using Rust more and more for the kinds of small projects where they once would have defaulted to something like Python. Between how solid a package management and build tool Cargo is, and the fact that once you get a Rust binary to *compile*, many times it already does what you want it to, it's extremely powerful. And it's also the kind of thing you can just install and run on *any platform*. Anybody who's spent time trying to distribute, say, Python or Ruby programs knows how valuable that is. Not having to install a runtime to install a program is a *big deal*.

I was talking about this on Twitter in light of that thread, and someone responded exactly how I've felt for a while now: it "Feels like Python, runs like C." In fact, this is actually what attracted me to Rust in the first place. When I started reading through the book, I remember very distinctly thinking, "Huh. This is the first language I think might replace Python as my go-to tool in general." And that's been born out by the last eight months of using it for little things here and there. And of course that goes for other languages, too; you could do the same with Elixir, or Haskell, or whatever else.

It might sound crazy to use Rust instead of Python for doing some relatively high-level work. Again: there's all that press out there about writing operating systems in it---and it's good at that. But that's not the only thing it's good at. The type system and good compiler mean that it's versatile and capable in all sorts of domains. Next time you need a utility... write it in Rust. You may be surprised at how straightforward it is, and you'll learn things about using Rust, and you'll come away with a better appreciation for just how well-designed the language is. I certainly intend to!

## Outro

### Sponsors

Thanks to Hamza Sheikh, Chris Palmer, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, or cash.me, or get in touch with me directly.


### Follow/support

You can find links to those as well as to the discussion threads I mentioned at NewRustacean.com. You can also follow the show on Twitter @newrustacean, or follow me there @chriskrycho. You can help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, tweeting about it, or just telling a friend!

So do respond on social media, in the thread for the episode on the Rust user forum at users.rust-lang.org, or via email at hello@newrustacean.com.

Until next time, happy coding!
