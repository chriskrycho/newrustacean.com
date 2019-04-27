# e030: Can You See Me Now?

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is e030: Can You See Me Now?

## Sponsor: Manning Press

Manning is back sponsoring the show this episode! This time around they have a discount on some Rust-adjacent materials—about one of my favorite things Rust excels at: WebAssembly. Manning has a book in their Early Access Preview period, <cite>WebAssembly in Action</cite>, which takes a deep dive into the language itself. This is *not* Rust-specific content: what it is instead is content that would be super helpful if you want to contribute actively to the development of Rust’s support for WebAssembly, or just want a deeper dive into wasm than I gave in my own episode on the subject. I’ve linked it in the show notes, and don’t forget that you can use the code `newrustacean40` at checkout to get 40% off on *anything* at Manning—including this WebAssembly book, all their Rust materials, and everything else they sell! Thanks so much to Manning for sponsoring the show once again!

## Visibility

This is not the next episode I originally had planned—that one is coming in the next week or so. In the midst of working on e029, I started to put together a discussion of the idea of *visibility* in Rust, and then cut it for time. However, I realized it’s an important feature of the language that I’ve never covered, and it’s worth incorporating into its own episode.

So: what *is* visibility, and why do we care? Visibility is the answer to the question: who can see this item? An “item”, here, is:

- a module,
- a function,
- a trait,
- an enum,
- a struct,
- a struct *field*,
- or a (non-trait) method

As for why we care: it turns out that having control over who can see any given item can be a really important factor in our API design. Things we expose publicly become part of our versioning story, and good and careful use of semantic versioning is a way we serve our library consumers.

When Rust first hit 1.0, it only had two levels of visibility: `pub` and not `pub`. In 2016, with [RFC #1422], however, Rust got a bunch of new tools for controlling item visibility, and I think it’s worth understanding both how those tools work and how we can use them effectively.

[RFC #1422]: https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md

### The modifiers

There are 3 top-level kinds of visibility:

- the default, of *not*-public, which means the item is visible only within its own module and its child modules;
- `pub`, meaning the item is fully visible to all consumers of the crate who can see the path to the item;
- and `pub(<restricted>)`, where the item is visible to some *restricted* range. There are also 3 variants of `restricted`:
    - `crate`, written `pub(crate)`, which means the item is visible everywhere in the crate but is *not* visible outside the crate
    - `super`, written `pub(super)`, which means what you’d expect: the item is visible to the parent module but nowhere else
    - `in <path>`, written like `pub(in module_a::module_b)` where `module_b` is a submodule of `module_a`. Note that the `path` here can only include direct parents of the module where the restrict is written.

The first two are the two that were always there. The `restricted` variants are what landed in 2016, and they’re mostly what makes this conversation interesting! I’m going to talk through these and how we can use them effectively, but you should also take a look at the show notes [source], where I’ve included a very detailed sample showing how all the different visibility markers interact.

[source]: https://newrustacean.com/src/show_notes/e030.rs.html

### Traditional `pub`

Now, let’s start talking about plain `pub` for a minute, because understanding `pub` clearly will help us make sure we understand `pub(<restricted>)` better—I actually had to read [the relevant section of the reference][reference] and play with things a bit myself to get 100% clear here!

[reference]: https://doc.rust-lang.org/reference/visibility-and-privacy.html

As I said a minute ago, `pub` on an item means that item is fully visible to all consumers of the crate who can see the path to the item. That distinction is really important, though! It’s not that putting `pub` an item makes it visible regardless of other modifiers. It’s that it makes it visible *given the constraints of other relevant visibility modifiers*.

Let’s imagine we have a crate which has two modules in the root; we’ll call them `left` and `right`. If both of those are `pub`, they’re visible both to each other and to other crates which reference this crate. If `left` has a child named `public_child` which has the `pub` modifier on it, then `left::public_child` is *also* visible everywhere in the crate and to all outside consumers. If `left` has a child named `private_child` which does not have any modifier at all, it is *not* visible outside `left` at all. Now, if `left` had a sub-module named `child`, then items inside `left::child` could still see `private_child`: child modules can always see the items in their parent modules.

Sibling modules do *not* have that characteristic, though. From inside our `right` module, we can see `left::public_child` because both `left` and `public_child` are marked as `pub`—but we can’t see anything inside `left` that *isn’t* marked as `pub`. We could work around this by restructuring our crate… but that really isn’t optimal, especially if you structure your modules around data types the way I talked about back in [episode 20][e020].

[src]: https://newrustacean.com/src/show_notes/e030.rs.html
[e020]: https://newrustacean.com/show_notes/e020/

This limitation isn’t necessarily a *huge* deal for designing a binary application. However, it’s *really* annoying for library design. We want to be able to design our public APIs carefully, so that we only expose to consumers the things they can actually depend on… in part so that we can evolve the internals of our libraries while maintaining those external invariants. Having to make breaking changes all the time *or* come up with workaround hacks for where we put our internal-only-but-shared-throughout-the-codebase kinds of types and functions is a *not-great* tradeoff. This is where the `pub(<restricted>)` tooling comes in!

### `pub(<restricted>)`

The `pub(<restricted>)` family of specifiers lets us get *much* more granular about the level of visibility any given item has, and this is extremely* helpful in solving the kinds of problems I highlighted a second ago.

The most common modifier you’re going to see, I expect, is `pub(crate)`, because it’s the most *convenient*: it lets you say “This item is visible *everywhere* that has a path to it… but only inside *this* crate.” This is like `pub` in terms of convenience, in other words, but not in terms of its impact on your public API—because things which are marked as `pub(crate)` are *not* visible outside your crate! At this point, `pub(crate)` is basically my default visibility for items I want to expose. Sometimes I go back later and tweak that, but I find it’s often a perfectly reasonable balance between convenience and exposure.

The *most* restrictive `pub` modifier is `pub(super)`. This is convenient for when you have a type which you’ve extracted to its own module for name-spacing purposes, in line with the recommendation I gave back in e020, and you want it to be freely available for its parent to use but you don’t really want it exposed to the rest of the crate you’re working on.

You might be tempted, given `pub(super)`, to think you could just chain your way up with pairs of double-colons all the way to root of a crate: `pub(super::super::super::super...)`. After all, that is a valid way to write a path! However, that’s not a valid way of specifying visibility. `pub(super)` is just there as a shortcut for that one, very specific but fairly common situation of wanting to be visible in your parent module but not otherwise.

Instead, if you want to write a restriction like that, you use the `pub(in <path>)` form. So if you had the module path `foo::bar::baz` and wanted to make an item `quux` visible anywhere in `foo`, you could write that one of two ways:

- `pub(in super::super)`
- or, much more clearly, `pub(in foo)`

You can write `pub(in ...)` with any path that contains the module the item is in. However, if some module somewhere in that tree is itself not visible the rest of the way up that tree, your item will only be visible up to that point. So if we wrote `pub(in foo) fn quux() {}` here but the declaration of the `baz` module inside `bar` was itself not any kind of `pub`, `foo` couldn’t see `quux`. This gets at one of two really important points about these visibility modifiers: they specify the *maximum* visibility an item can have—but other modifiers (or the lack thereof) can and sometimes will limit that. You may have to adjust accordingly.

The other important thing to note here is that outer modules cannot override the visibility rules of their descendants. So with that last example, where we had the module structure `foo::bar::baz` and a function `quux` inside `baz` which is marked `pub(in foo)`, `foo` *cannot* write `pub use bar::baz::quux` to make it publicly visible everywhere. The restriction at the point of definition sets the visibility restriction universally. That’s the flip side of the same thing I said before: a visibility modifier is the *maximum* visibility of an item.

### Using visibility effectively

The last thing to cover here, then, is how to use these most effectively. I covered some of this above, and I covered some related pieces back in e020, but I want to take a few here and talk through how *I* think about visibility modifiers and how you can use them as a tool both for external *and* internal consumers of your code.

First and most obvious is the point I’ve already made a couple times in this episode: `pub` items are exposed to your external API consumers. Whether those are internal to projects at your company, or to the broader open source software ecosystem, those are a kind of *contract*. When you change what’s public, that’s a version bump—a minor release, if you expose something new; or a major release, if you remove or change something which was already public. That responsibility with versioning is one reason you should always think carefully about what you mark `pub`, and how you expose it. Another, and equally important, though, is that your API is a way of communicating to your users. You really can help people understand and navigate their way through your code just by the shape of your API!

The way that items are exposed and the way they’re grouped and namespaced in modules shouldn’t be arbitrary, but rather a way of helping your users think the right way about your codebase. You’ve probably had the experience of looking at a library which just had a grab-bag of functions and types in it with no structure: it’s a mess and takes a long time to understand! But you’ve probably also had the experience of working through the APIs for a really carefully-designed library and thinking, “I wonder if… yeah, perfect, that’s right where I thought it would be!” or even “Oh, of course, that makes sense!” when you stumble into some new corner of it. Care in what you make `pub` and *where* you make it `pub` from is really valuable—so much so that you may want to use `pub use` statements to take items which are indeed public throughout your crate and re-export them in strategic locations, whether for convenience or just because it makes for a clearer sense of how different pieces relate to each other from the perspective of a *user* rather than an *author* of a library.

All of those points about communication hold for internal visibility, too. It’s just that you have a different audience in mind: your *users*, in this case, are *other developers*. When someone starts working their way through the codebase, it can be incredibly helpful and illuminating to see `pub(crate)` or `pub(in some:nested::modules`) as the visibility modifier for a given item.

First of all, those can tell you the number of places you might have to care about when dealing with a given type. If you’re looking at a crate with 50,000 lines of code, and you see `pub(crate)`, you know that a refactor *could* end up touching thousands of lines of code. And while the Rust compiler will guide you through that, there are still important signals there: this item is probably deeply entangled with the current semantics of the system I’m looking at. That means that changes to that item are, for one thing, likely to have large ripples through the rest of the system and, for another, likely to have ended up in the shape they did for a reason. Those are useful signals in understanding a codebase!

And by the same token, if you see `pub(super)` on an item, you can be 100% confident that it’s *not* going to be used in very many places. It might still have an equally well-considered design, but changes to it are going to cause many fewer ripples and have a much smaller impact on the design of the rest of the system. Again: that’s really useful information to have! The same way that thinking about what is `pub`, and where it is `pub`, is useful for your *external* users, `pub(<restricted>)` is helpful for your *internal* users.

Finally, there’s one other thing you’re communicating with by way of visibility modifiers, and that’s the compiler. You can use these as a way of constraining yourself. The same way that we isolate unsafe code in `unsafe` blocks inside safe interfaces which maintain all the right invariants, we can use visibility modifiers to isolate certain kinds of complexity—data structures, algorithms, etc.—which are important for the rest of our codebase not to muck with. If a given piece of code needs to uphold a certain contract in order to maintain its *performance* characteristics, for example, you can manage with visibility modifiers just the same way you would use safe wrappers around `unsafe` to maintain *safety* characteristics.

The way you should think about visibility markers, then, is that they’re *communication tools* and *tools for enforcing invariants*. That combo means, as I’ve realized in the course of writing this episode, that while my default of using `pub(crate)` for convenience is fine on small projects, thinking a bit harder about which kind of modifier to use might get me a long way—especially when dealing with a larger project or one with more complex edge cases than I’ve gotten myself into so far.

## Outro

I hope you *also* found this discussion useful, both in understanding how other codebases are structured, and in considering how you might structure your own codebase—especially how to use visibility as a tool for providing a nice experience both for maintainers of your code (including yourself!) and, if you’re a library author, for *consumers* of your code.

### Patreon Sponsors

Thanks as always to this month’s $10-or-more sponsors:

- Arun Kulshreshtha
- Matt Rudder
- Soren Bramer Schmidt
- Dan Abrams
- Oluseyi Sonaiya
- Anthony Deschamps
- Evan Stoll
- Nick Gideo
- Dominic Cooney
- Embark Studios
- Scott Moeller
- Benjamin Manns
- Daniel Mason
- Jonathan Knapp
- Nick Stevens
- Jeff May
- Behnam Esfahbod
- Johan Andersson
- Nathan Sculli
- James Hagans II
- John Rudnick
- Zach Peters
- Chip
- Jerome Froelich
- Andrew Dirksen
- Joseph Schrag
- Brian McCallister
- Bryan Stitt
- Raph Levien
- Nicolas Pochet
- Daniel Bornkessel (April only)
- Ryan Osial
- Jason Bowen
- Jako Danar
- Michael Mc Donnell
- Adam Green
- Alexander Payne
- Rob Tsuk
- David Carroll
- Ramon Buckland
- Martin Heuschober
- Peter Tillemans
- Paul Naranja
- Graham Wihlidal
- Ola Fadeyi
- Cristian Paul
- Daniel Collin

### Show info

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/cysk/bindgen/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!
