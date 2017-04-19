# RLS

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 show about learning the Rust programming language. This is *Crates You Should Know: R.L.S.*

## The crate

In terms of *exciting* things happening in the Rust ecosystem right now, there are a lot we could mention – not least crates like Diesel and Rocket which are especially appealing to me as a web developer. And we'll get around to a bunch of those eventually. But today, the crate I want to talk about is one you're not very likely *ever* to use by adding it to your `crates.toml` file, but which by the end of 2018 you will probably be using in every single Rust project nonetheless: the Rust Language Service (or RLS for short).

The RLS is Rust's answer to the question: _How do we make it easy for everyone to have a great integrated development environment for Rust?_ For the most part, IDEs for other languages have always had to implement their own smarts about the language. So, for example, the JetBrains family of IDEs – tools like IntelliJ IDEA, WebStorm, PyCharm, RubyMine, CLion, etc. – all use some common underlying infrastructure for things like code analysis, completion, refactorings, and so on. That infrastructure is shared among JetBrains products, but they have to implement everything specific to Java, Scala, JavaScript, Python, PHP, C++, etc. for themselves. And if you open up Eclipse, it has its own infrastructure for code completion, refactoring, etc. Lighter-weight GUI text editors like Atom, Visual Studio Code, and Sublime Text have *their* own code completion And if you're running vim or emacs, they have their own tools, too. Every editor has to reimplement the same things.

But not so with Rust. The Rust Language Service means every editor which supports the open-source *Language Service Protocol*, can have completion, type inference, type checking, and (soon-ish) automated refactoring and code formatting tools for Rust… basically for free.

### Background: LSP

A few years ago, as part of its work on Visual Studio Code and the plugins it developed for supporting cross-platform .NET work in C#, Microsoft developed the *Language Service Protocol*. The protocol defines a standard way for a *client* (normally, though not *necessarily*) an editor of some sort to talk to a *server* (e.g. a compiler or other language analysis tool). The protocol defines the kinds of messages, the format they go in, etc. in a straightforward JSON format. And because it's such a straightforward format, you can talk to a LSP server via simple command-line call if you want – which makes it fairly easy to test at a basic level, and even (in principle) lets you do interesting things like build Read-Evaluate-Print-Loops (REPLs) with it. (There's nothing like that on the radar for Rust, at least to the best of my knowledge, but it's certainly interesting!)

I use the TypeScript Language Service every day at work, and it means that editors like VS Code, Sublime, and Atom all have pretty similar feature sets and behavior for doing TypeScript development, despite being very different editors in many other ways. It brings the experience of writing TypeScript in those languages – with community-supported plugins, in the cases of Atom and Sublime – very nearly to parity to the professionally developed and supported features of JetBrains IDEs, and even the official Microsoft support VS Code plugin... is just a TypeScript language server client.

### Status

If you listened to the second news episode, you'll recall that I mentioned this was coming down the line and was in a very early and very rough "alpha" stage back in late 2016. As of this week (in mid-April, 2017), it has graduated from the "early alpha" stage it was in then to "beta": you can get it with nightly builds of Rust *today* by doing `rustup update nightly` and then `rustup component add rls`, `rustup component add rust-analysis` and `rustup component add rust-src`. Even better, editor plugins can (at least in theory) do all of this *for* you, just by asking permission to run `rustup` for you.

The best ways to use this today are either to follow the instructions with the reference VS Code client from the official repository, or to use Kalita Alexey's `vscode-rust` VS Code extension – an actively-developed fork of the older and now unmaintained RustyCode extension. (I'll link that in the show notes, of course!)

### The Future

There's a lot that works well now, but there is also a lot of room for further improvement with the Rust Language Service. First, and most painful to me: there are no automated refactorings yet, even the simple ones like renaming. The combination of find-and-replace plus a reliable compiler takes you a long way on that, to be sure: it's worlds better than the story in JavaScript, for example. But being able to hit a keyboard shortcut and rename a function or a type or even just a local variable and have it work everywhere it needs to throughout your whole codebase is a big win. When you've gotten used to being able to do that, going back to not having it feels barbaric. The same is true of extracting chunks of code into their own functions. You don't always know exactly where things will break down into discrete transformations; good refactoring tools let you do it on the fly as you as you identify common things to reuse while working through a given set of functionality. Speaking from experience, that is an incredible productivity boost.

...and the RLS isn't there yet. However, renames are behind a feature flag, and although further refactoring is still just a twinkle in Nick Cameron and Jonathan Turner's eyes, there's hope.

In part, that's because of the other thing that isn't there yet: further improvements to the compiler itself to support the RLS. Some of the initial requirements (like incremental compilation in general) have just started making their way toward beta and stable builds. Others are further off: getting data on demand and incrementally for just the very specific local pieces of the code base. (And those kinds of changes could ripple out to have powerful effects elsewhere in the Rust development cycle: it's not hard to imagine how they could not only be helpful for compilation speed but also for REPL-like interactive programming approaches.)

## Closing

So that's the Rust Language Service in a nutshell. It's already making my life far more awesome when writing Rust, and I am excited to see where it goes over the next year. If you enjoyed hearing a bit about the RLS today and want to hear more... you're in luck! Coming up next is an interview with Jonathan Turner, who helped build first the 

### Sponsors

Thanks to 

- Anthony Deschamps
- Christopher Giffard
- Chris Palmer
- Dan Abrams
- Daniel Collin
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website, or if you're a company interested in advertising to developers, get in touch!

### Info

You can also find show notes and links for this episode and previous episodes, code samples, and more at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it! You can also help others discover the show by rating and reviewing it on iTunes, recommending it in other podcast directories, or sharing it around on whatever social media you use.

I'd love to hear your feedback, as well as suggestions for topics, interviewees, and so on, in the threads for the episode on the Rust user forum or Hacker News. I also love getting email---you can send me a note at hello@newrustacean.com.

Until next time, happy coding!
