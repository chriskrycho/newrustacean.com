# Let's Talk Roadmap

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *News Episode 2: Let's Talk Roadmap*.

## Status update

To listeners and especially sponsors who've waited patiently these last couple months, thanks so much for your patience. 2016 ended up being one of the most personally challenging years of my life, and a lot of stuff that happened earlier in the year resulted in tons of work piling up at the *end* of the year. I had no bandwidth whatsoever for anything but work and school.

But it's my end-of-year vacation, and I'm not in class *or* working, so here we are! A news episode at last! I've also made a reasonable amount of progress on my "Lightning" static site generator project this week—it's still a long way from being ready to use, but a lot of the foundational pieces are in place now. You can follow along with that at github.com/chriskrycho/lightning-rs.

Now, for news about *Rust*.


## 2016

2016 was a big year for Rust—not so much in terms of any huge features, but instead in terms of a lot of small wins. In addition to landing MIR and *starting* to reap some of the performance rewards from it discussed back in the first news episode, there are a bunch of other bits and pieces that have made Rust nicer to use.

### The language

Here are a few of the biggest things to hit the language itself since May. (I mentioned a few of these in news sections of previous episodes, but it's worth collecting them all here to get a sense of the rest of the year as a whole):

- **More options for handling `panic!`s:** Rust 1.9 and 1.10 together made it possible to define much more specifically how your application should behave in the face of panics. First, you can now catch panics and deal with them in a pre-specified way, which is useful for cross-thread or cross-language boundary issues. You can do that with the `std::panic` module.

    Second, you can choose to simply abort the process entirely, with no stack unwinding, when it makes sense---and that can decrease the size of your binary and make your compile times faster. Both of those tools are things you should explore carefully before applying them, because as always there are tradeoffs. But it's nice to have the tools when we need them. You can configure that in your Cargo configuration, or pass it explicitly as a command line flag.

- Rust also got some **new and improved error output**. That comes in two parts: the introduction of a JSON-based output for smart text editors and IDEs to use, and a nicer error message format for users. The new JSON error format for editors means they can simply define how to handle the JSON payload instead of having to parse information out of a plain-text error.

    The new error format for users makes it far, *far* easier to understand your errors. For one, as the announcement blog post noted, it puts your code front and center, making it obvious where the errors actually are. The old format included that info, but often buried it in a wall of other text. That other text is still there, but it's arranged differently, so that the code which doesn't work is more obvious. I've appreciated this a *lot* in the last week as I've been hammering away at Lightning.
    
- One of the big hopes for MIR landing was that we'd get **incrememental compilation**, which should speed up the develop-and-compile-and-test cycle dramatically. That feature hasn't *quite* landed on stable Rust yet, but it's coming! It's been in-development-and-available-on-nightly mode for several months now, and if you take a look at the GitHub milestone where work is being tracked, it's getting awfully close to landing the first phase on stable.

- One of the other hopes for MIR is that it will enable better optimizations. There aren't a ton of these yet, but they're trickling in and they're adding up. A bunch of these landed in 1.13 and 1.14 in particular, so if you're interested in the details, you should take a look at those blog posts and release notes, which I've linked in the show notes. (The same is true for the bits I mentioned above, too, of course.)

- Finally, and perhaps most intriguingly, 1.14 came with experimental support for WebAssembly. If you're not a web developer, you probably haven't heard of WebAssembly (and even if you *are* a web developer, it may  not have crossed your radar yet). WebAssembly builds on work done in the browser space in the last few years, aiming to provide what its website describes as "a portable, size- and load-time-efficient format suitable for compilation to the web." Put another way, it's a way to compile a variety of languages to a high-performance representation (better than normal JavaScript by a considerable margin) that can run in contexts like browsers.You can actually do some pretty interesting things targeting the browser from Rust *now*, since that experimental support landed in 1.14. There's a long ways to go there on both the compilation front and the WebAssembly standard itself, but we've taken the first steps.


### `rustup`

So let's say you wanted to play with WebAssembly: how would you do it? Well, a tool I mentioned as newly in beta in the last news episode has now made it to 1.0! `rustup` is now the officially recommended tool for managing your Rust installations. The tool lets you install any specific version of Rust, as well as grabbing the latest stable, beta, and nightly versions of the whole toolchain (including Cargo).

It also supports installing more than one *target* for the compilation, and that's how you can compile to WebAssembly today. For *any* target supported by Rust, you can just type `rustup target add` followed by the name of the target. You may also need a couple other pieces to link to, but this actually gets you everything you need for cross-compilation for a *lot* of targets. So for WebAssembly, you need to install one other dependency, a tool called _emscripten_. Then you type `rustup target add wasm32-unknown-emscripten`, and you're off to the races, using your normal toolchain. You can do `cargo build --target wasm32-unknown-emscripten`, for example, to build a whole crate targeting WebAssembly. And you can do the same kind of thing if you're cross-compiling to Windows from Mac or Linux, for example---you just need to snag a few dependencies and you're off to the races, able to ship native binaries on every platform.

If you'd like to see a fairly trivial but nonetheless interesting example of compiling for Windows from macOS, you can take a look at a blog post I wrote describing the process. Having `rustup` in our toolbelt for the years ahead is going to help a *ton* for cross-compiling. Anyone who's ever tried to do the same with C or C++ will appreciate this a lot. And anyone who's ever tried to ship standalone binaries of Ruby or Python programs knows that that has its own challenges. Having the ability to do this *and* integrate easily with existing C libraries puts Rust in a pretty unique spot.

You should see more work on this, and on "xargo", the cross-compilation tooling around Cargo, in the months and years ahead—but the truth is, it's already in a pretty marvelous spot, and only getting better.


### Cargo

Speaking of Cargo, it's mostly just chugged along steadily over the past few months, but it did get one very significant new feature this year: the ability to manage a *workspace*. The idea is: you often have a set of related crates which you want to be distributed and compiled separately, but which work together closely, and which you want to be able to use with each other easily. A workspace lets you define exactly that setup, without having to do all of the development in a single version control repository or a single file tree. To define a workspace, you just have to add a "workspace" section in the main crate's `Cargo.toml` file, and then specify the root workspace in each child crate's own `Cargo.toml` file. (You can also optionally specify the child crates in the main crate, but you don't have to.) Also, handily enough, you don't even have to specify it for the child crates if they *do* live in a single file tree (e.g as git subrepositories or similar); you only have to specify them if they're not in that "conventional" location.

When you `cargo build` any of the child crates, the compiled binaries end up in the relevant target directory for the parent crate, ready to be linked into the final application. And when you `cargo build` the parent crate, it will pull those dependencies in together just as if they were located elsewhere, but with the binaries already available for linkage, it will speed up the process substantially (and make organizing these kinds of projects a lot easier along the way).

This isn't necessarily something every project will deal with, but if it's something *your* project deals with, you'll definitely be happy to have it. (And it's the kind of thing that many projects will *eventually* run into as they grow in scale. So you may not need it today, but you might *tomorrow*.)


### Rust Language Service

The last *big* thing that is worth mentioning as a 2016 development is the Rust Language Service. This one's a huge "work-in-progress": we haven't seen a lot publicly since it was demoed at RustConf, but it's been seeing steady, if quiet, incremental development since then.

The Rust Language Service is a piece of software that sits between your editor---whether that's something like the JetBrains IDE plugin or something like Vim---and provides information for everything from autocomplete suggestions to refactoring. Historically, these kinds of things were done on a per-IDE or per-editor basis, and that meant a lot of work to be redone for every editor. With their work on TypeScript, Microsoft took a different tack: they created a language service that any editor could connect to and get top-notch information for all of those pieces of IDE-level functionality: syntax checking, type checking, finding usages, finding definitions, doing renames, etc. If you've ever used the official TypeScript plugins for VS Code, Atom, Sublime Text, etc., it's using that service---and it can be very, very good; it's easily one of the best editing experiences I've ever had in any programming language.

One of the developers who was responsible for creating the TypeScript language service is Jonathan Turner, now an employee of Mozilla working on (wait for it...) the *Rust Language Service*, which will bring all that same shiny IDE-type goodness to *our* editing experience. Of all the things that should land in 2017, a beta- or even stable release of the RLS could be one of the biggest game-changers. When you're just getting going, this kind of things is invaluable because it helps you explore the space. When you're an expert, it speeds you up and increases your productivity a *lot*---especially when doing refactoring work, where it beats the pants off of either find-and-replace or making a change and then working through a list of compiler errors on the command line. Knowing you have a type error in roughly real-time in your editor is *fantastic*. Tools like `cargo check` can give you some of those benefits today, but they get slower and slower the larger your project gets, because they depend on doing actual compilation work.

Suffice it to say I'm excited about the work that's already been done here, and even more so about what's ahead.


### The community

I'm certainly biased, but I think of the other big changes that came out of 2016 was a new, hard commitment to documenting all new language and standard library features. I'm biased, of course, because I wrote the RFC that articulated both the need and---eventually, after 80-some-odd comments---the process we'll be using to tackle this going forward. But bias aside, I think it's a wonderful comment on the Rust community that the discussion was entirely about *how to tackle the problem*, not *whether it was a good idea*.

If you've ever gone hunting for information on language features introduced since Rust 1.0 came out, you've probably discovered that a lot of those features aren't documented anywhere except in the RFCs which proposed them. You've probably also noticed that the Rust reference is extremely out of date. In fact, the reference currently includes a note indicating that it *tends* to be out of date---direct quote! So when I proposed that all new features be required to be documented before they become stable, that means there's a lot of work to be done. But again: the response was "Let's do this! Now, how?" And that's already started to play out as features preparing to merge to stable have been getting documented. We have a lot of work to do here in 2017---more on that in a minute. But the big takeaway for me here was, again: the Rust community has its collective head on straight about these things.


## 2017

For 2017, the Rust core team developed a set of overall goals to guide the development of the language. You can see the full proposal and discussion at the RFC (linked in the show notes) for the year goals. Here, I'll just quote from the overview and discuss a little of what it should mean.

> This year's focus is improving Rust's productivity, while retaining its emphasis on fast, reliable code. At a high level, by the end of 2017:
>
> - Rust should have a lower learning curve
> - Rust should have a pleasant edit-compile-debug cycle
> - Rust should provide a solid, but basic IDE experience
> - Rust should provide easy access to high quality crates
> - Rust should be well-equipped for writing robust, high-scale servers
> - Rust should have 1.0-level crates for essential tasks
> - Rust should integrate easily into large build systems
> - Rust's community should provide mentoring at all levels
>
> In addition, we should make significant strides in exploring two areas where we're not quite ready to set out specific goals:
>
> - Integration with other languages, running the gamut from C to JavaScript
> - Usage in resource-constrained environments

The big takeaway here is that the goal isn't shiny new, big bang features, but instead of a bunch of sort of "infrastructural" improvements. Things like the previously mentioned Rust Language Service fit right into a bunch of these.

The other big points of interest---high-quality, 1.0-level crates; being well equipped for servers; and being able to integrate into large build systems---will make a big difference for making it possible for companies (maybe including yours) to adopt Rust in production. I don't really expect that to be the case for my current employer (though who knows? If an opportunity comes up for us to write something in Rust you better believe I'll be taking it). You can expect to see 2017 be the year where the "primitives" for servers all get solidified as projects like the Futures library and Tokio mature. Then in turn, mid to late in the year you will probably start seeing some more easily usable mid-to-high-level web frameworks appear or substantially mature. For an example of one mid-level framework that just launched, and which should be able to take advantage of the low-level work provided by Futures and Tokio this coming year, you can (and should!) check out the recently announced Rocket framework, which looks quite nice. (I'll of course have links to all of those in the show notes.) One reason I'm really excited about *these* features is that they hit my niche: web development with Rust is a fascinating corner.

One of my personal Rust goals for 2017---besides getting my Lightning project working---is to do (at least a large chunk of) the work necessary to "implement" RFC 1636 and get all our existing features documented. I also expect to chip in here and there with documenting new language features as they land. If you're looking for a way to jump in, this is a great spot to help: check out the list of undocumented features I'm putting together on Rust issue #38643---I'll link it in the show notes. I think it also fits well with these overall goals, in that *having a lower learning curve* certainly includes having all the language features documented!


## Closing

So that gives you a pretty good idea of where Rust has been in the latter half of 2016 and a little preview of where we're going. 2017 looks like it should be a great year of just making things nicer and easier to use all around. Thanks for following along with me this year. In 2017 you can look forward to a lot more New Rustacean---interviews, the new Crates You Should Know format, the occasional bonus episode, news episodes, and of course the dedicated topic episodes. January should hopefully see at least one Crates You Should Know and one topic episode!


### Sponsors

Thanks to 

- Chris Palmer
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Steven Murawski
- Raph Levien
- Daniel Collin
- and Vesa Khailavirta

for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website.

### Follow

Speaking of the website... you can also find links to everything I mentioned on the show today at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it. It also helps others discover the show if you leave a rating and review it on iTunes, recommend it in other podcast directories, or share it around on whatever social media you use.

You can respond in the thread for the episode on the Rust user forum at users.rust-lang.org, or you can shoot me an email at hello@newrustacean.com.

At the end of this year, it seems particularly appropriate to say: Thank you so much, everyone who has sponsored the show---whether through Patreon or any of those other channels. Thanks as well to all of you who have sent me kind and encouraging notes. This show has been a bright note in the midst of a year that was, at times, extremely challenging. And your support---verbal no less than financial---encourages me to keep working to craft this into the best show I can make. Finally, thank you to everyone who just listens to the show: it's no small thing to me that there are thousands of people out there who listen whenever I post an episode. Thanks, Rustaceans!

Happy new year, and until next time, happy coding!
