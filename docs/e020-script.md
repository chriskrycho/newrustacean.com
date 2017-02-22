# Putting Code in its Place

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast—a 15--20 minute show about learning the Rust programming language. This is *Episode 20: Putting Code in its Place*.

## Structuring Code

One of the things that may have thrown you a bit when coming to Rust is thinking about *how to organize your code*. This is—like every episode I put out!—still something I'm getting a handle on myself. But I've picked up a fair bit by reading Rust codebases over the last couple years, even if I don't have anything at that scale myself. And there are some guiding principles we can draw on not only from Rust but from other languages as well as we think about where to draw the boundaries for our modules and our crates.

### Other languages

Many of you coming to Rust are likely coming in from a language like Java or C#, where it's extremely common to group your code primarily by *class* or *interface*, and where it's most common for each file to have one class in it. Java has *packages* comprised of many classes and interfaces, and C# has *namespaces* comprised of many classes and interfaces. Rust's modules have some things in common with namespaces, but a couple important differences, which I'll mention below.

Others of you are coming from C or C++. C++ often has similar conventions to Java or C# about having one class per file, but the lines are a little blurrier there because C++ doesn't require *everything* to be attached to classes: you can have functions and static items off by themselves when it makes sense. Modules are usually marked off in terms of *compilation units*: basically source files and the things they include. You use header files to define your public interfaces. Rust doesn't have header files at *all*, of course, so there will be some sharp differences here, and its notion of a compilation unit is also quite different: it is the *crate*, not the *file* (though as we saw in episode 6, files are important---and indeed, that's where we'll spend most of our time today).

Some of you are coming in from languages like Ruby, Python, or JavaScript, which all have more or less the idea of standalone functions as well as classes. They also have their own distinct ways of structuring and supplying public interfaces to code. In Python, these map to files and directories with a special file in them. JavaScript is similar, but adds the idea of explicit exports from a given file which can be imported into another file. TypeScript further extends that with *namespaces*, which function much like C# namespaces do. The basic structural divisions in these languages include both *modules* and *classes*, and it's fairly typical to have only one class per module in well-organized code in them---and especially, only one *exported* class. This same idea is going to feature in our approaches in Rust.

Finally, those of you coming in from the world of strongly typed functional programming languages---Haskell, F#, etc.---will primarily be used to organizing things into related data structures and functions designed to operate on them. (This, I admit, is the area where I have the least experience, though I remain very interested in using these languages more actively.) And this approach, too, features fairly clear in Rust, just with its own twist.

Indeed, it's fair to say that Rust has taken the tack of trying to pull the best ideas from each of those other organizational systems and then mix them into a form that feels nicely Rustic in its crates-and-modules system.

### Rust

As we talked about all the way back in episode 6, the basic units of organization in Rust are *modules* and *crates*. Crates are both the overall unit of compilation and the overall unit of *functional separation*. The time to break things out into distinct crates is where you can see a given chunk of functionality being reused across more than one project. For example, small though it is, I may eventually pull the YAML metadata extractor I've written for Lightning (and about which I'll have more to say below) into its own little crate: I'm certainly not the only person out there who might find it useful to extract YAML metadata from a Markdown document. On the other hand, the vast majority of the implementation details of Lightning will be specific to it and won't be readily reusable.

I will take this opportunity to rave about Rust's crate system again for a moment before moving into the nitty-gritty details of how to structure the modules within a crate, though. The more time I spend taking advantage of crates.io and Cargo, the more grateful I am to have it. In C and C++, because they have historically lacked a good story for managing *packages*, it has often been painfully difficult to do that kind of extraction work---I've seen it be much easier in practice to simply copy-and-paste code into another codebase than to try to manage them as separate libraries, simply because there isn't a good way there to deal with version locking or those kinds of concerns. You can do it... but it's painful. Cargo for the win.

Now, onto talking about how we organize our crates within modules. Here I'm looking not so much at the mechanics, which I covered in some detail back in episode 6, but in terms of *philosophy*: what do we put where, and why?

First, though: one clarification I should add to those details from episode 6, given my references to C# and TypeScript above, though: there is one major difference between the C# or TypeScript namespaces in C# or TypeScript and Rust modules. C# and TypeScript allow you to extend namespaces in various places. A Rust module is only defined in a single place---either in a file, or in a `mod { ... }` block. This doesn't end up being that much of a problem, though: it's not that common (at least in my experience!) to want to extend a given namespace, and since (as I discussed in more detail back in episode 6) you can re-export items in other modules. And I like the tradeoff: extending namespaces from arbitrary locations---rather like monkey-patching a class or object in a dynamic language---has some serious downsides in terms of clarity and expected behavior.

But the big analogy to namespaces is there: both are ways of providing a space that *isn't* just a data structure for a given name to live. `Metadata::parse()` could mean a lot of things: `item::Metadata::parse()` tells us a lot more.

In any case, this leads us to an important point we need to have in mind when designing the structure of our modules in Rust. Rust *data structures* (`struct`s and `enum`s) are open for extension (via the trait system), but its *modules* are not: the interfaces they export are the interfaces they export, and that's it. You can't reach into them and pull things out they don't choose to expose. Those of you familiar with popular tenets of object-oriented programming may recognize that this has important consequences in terms of "information hiding." Put another (and in my opinion better) way: since not only data structures but also *modules* have public APIs, we have considerable flexibility in designing the our APIs. We have considerable flexibility in what we make private and public---we don't usually feel the need Java's `friend` or the more general `protected` modifier which sits between `public` and `private` in class-structured languages. Instead, you can have a struct with public fields and access those fields freely throughout the module---and all you need to do to prevent other modules from using it is... nothing. You just don't write `pub` in front of the struct declaration, and it's invisible to anything outside the module. This gives us as much granularity and flexibility as we could wish for.

### Putting it together

Now, given *crates* as compilation units and *modules* as namespace-like divisions of responsibility, we can now talk about how we want to structure our code across those various divisions. I'll sum this up by saying the basic principle is "think about boundaries in terms of responsibility." That is, of course, an obnoxiously general and vague principle, but I think one of the challenging things about this topic in *any* language is that it isn't obvious. The two most difficult problems in computer science may be naming things, cache invalidation, and off-by-one errors, but *organizing your code* has to be in the top ten. One reason we make such a big deal out of things like the Single Responsibility Principle---and one reason we have to come back to it over and over again!---in the context of Object Oriented Programming is that it's a lot of work determine what the sole responsibility of a given `class` actually is! And the same challenge confronts us here. The fact that we group by module rather than by class doesn't change the fundamental difficulty of the task.

But with that as our guiding principle, here are the basic ways *I* think about breaking apart my code in Rust (and, for that matter, a lot of times in TypeScript or JavaScript, too).

First, I just try to group closely-related data structures and associated functions. Rust's `impl` blocks go a long way toward making that straightforward in most cases, of course. But even where I have a function which isn't in an `impl` block for a given data structure, e.g. a function which is associated with *more than one* data structure and doesn't logically belong to one of them in particular, I'll try to keep that close to where one or the other of the data structures are contained. Many times, that'll be in a module which is parent to the two modules which define those data structures.

And that takes me to my second habit. In general, I tend to want one *public* `struct` or `enum` per module. That's not a hard and fast rule, but it's a habit I've maintained not only in Rust but also in JavaScript and TypeScript over the past couple years with very good results. In my experience, it makes it fairly obvious where to draw the lines, *most* of the time... at least, once you have a good handle on what the data structures themselves should be! Beyond that, there may be any number of *internal* data structures beyond that, but there's actually a really good reason for the one-class-to-one-file structure that's normal in e.g. Java. That reason is that *data structures* are the fundamental things we're dealing with day to day, and an enormous amount of the work we're doing is about transformation of data from one structure to another.

So, for example, in my Lightning static site generator project, one of the things I've been slowly plugging away at in the last couple weeks has been doing metadata extraction: given a block of YAML metadata at the top of a Markdown item, I want to extract all of that data. I'm dealing with two data structures there: a source file---which is just a big `String`---and an output structure of a `struct` consisting of the extracted metadata. So the things this module exports are that metadata `struct` and its implementation. There's also an associated `enum` type which is a *child* of that `struct`, which is also exported, as it has to be: you can't "leak" a private type. That right there is basically the only exception to my "only export one data structure from a module" habit. But the idea remains the same: I try to only export one *top-level* data structure from a module; any other data structures exported are pieces of that top-level structure.

As an aside, and amusingly---at least to me!---it was articulating this for this episode which helped me realize that I'd drawn the lines in the wrong place: prior to thinking this through, I was treating all the metadata pieces as part of the *item* logic, and accordingly the module was `builder::item` but it actually needed to be at `item::metadata`. There are two reasons for that. First, although the site builder needs to know about items, items are really their own domain, and it's conceivable that I might want to use that data structure elsewhere and for other reasons. Second, item `metadata` is its own concern and has its own data structures, which `item` in turn can *use*. It's subsidiary concern of items.

Finally, this brings us to the fact that "parent" modules may want to export some of their children items for convenience in terms of API design. I *could* make myself write `item::metadata::Metadata::parse_from_str()` anywhere I need it... but that's rather needless. Instead, right now I'm doing `pub use metadata::{Metadata,ExtraMetadata}` in the `mod.rs` which defines the `item` module, so that a caller can do `item::Metadata::parse_from_str()` instead.

As a rule, the only modules which I let export more than one (primary) data structure will be those kinds of parent modules. Note that I do therefore distinguish between data structures *defined and exported* by a module, and data structures *re-exported* by a module for convenience. The former is an organizational and division-of-concerns question; the latter a question of the ergonomics of *using* the module. And it's worth noting that those *are* different concerns. How I want to consume a module is different from how I want to think about the separation of concerns within my codebase. They're related, of course. You can't talk about separation of concerns without considering how the pieces will be used. In the end, it's possible I won't be exposing metadata distinctly from the `Item`s which use them (though, they'll still be exported in order to be attached to any such `Item`). But having them in their own module allows me to deal with concerns that have nothing to do with Markdown parsing: extracting YAML metadata from the top of the block and converting it to the kind of `HashMap`-based structure I need is totally distinct from converting the body of the item from Markdown to HTML. So in that sense, I separate my code into *modules* based on the Single Responsibility Principle, much as you might with classes in a class-oriented language.

Let's summarize, then:

- *modules* are the fundamental unit of organization in Rust.
- modules should group closely related data structures and functions
- modules should normally only export one primary data structure
- *crates* are for whenever you need to reuse a given module across more than one project

Remember: this is more something I've felt my way to than something I think you should treat as a hard and fast rule. But hopefully it serves as a helpful set of guidelines for starting to think about the structure of a Rust codebase---even if you end up adapting it yourself as you grow more experienced.

## Closing

Today's episode was brought to you specifically by Christopher Giffard. One of the higher sponsor tiers on my Patreon page includes getting to pick a topic, and this was a point of interest for him. Thanks, Christopher, both for sponsoring and for prodding me to think on this!

### Sponsors

Thanks to 

- Chris Palmer
- Dan Abrams
- Daniel Collin
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

for sponsoring the show this month! You can see a full list of sponsors in the show notes. And thanks to my friend Ben Makuh for helping me come up with the punny title! (Seriously, people: I spend a ridiculous amount of time trying to think of good titles, and my slightly sick self was not getting there today.)

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website, or if you're a company interested in advertising to developers, please email me!

### Info

You can find show notes with links, code samples, and more at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it! You can also help others discover the show by rating and reviewing it on iTunes, recommending it in other podcast directories, or sharing it around on whatever social media you use.

Until next time, happy coding... and giving back!
