Composing a Rustic tune
=======================

Intro (30s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is *Episode 9: Composing
a Rustic tune*.


News (2m 30s -> 3m)
----
Things have been a bit quiet the last couple weeks---the holidays will do that!
There are a couple tidbits of note, however.

Nick Cameron, one of the Rust core developers, wrote an interesting blog post
called "My Thoughts on Rust in 2016," which I'll link in the show-notes. It's
thoughts and guesses, not promises, but it still makes for interesting reading.

The one other pretty big piece of news, though it's a fair ways out yet, is a
pair of breaking changes in the language coming in the 1.7 release. Both of the
changes have to do with details in the type system which we haven't even talked
about on the show yet, and both of the changes *improve* Rust's type system to
make it more robust. One of these changes started issue warnings in 1.4 back in
October 2015, and will become compile errors when Rust 1.7 comes out on March 3.
The other will become warnings with 1.7 and errors at some later point.

One of the neat things about this is how the Rust team has been able to tell
before making the change how many packages will be affected by it and in fact to
inform the package developers (and people who depend on them) of the issues.
This all falls out of the tooling they've built over the last year, especially a
tool called Crater which runs the compiler over every package uploaded to
crates.io and sees what breaks with any given set of changes. As of the 1.5
release, they saw that 96% of the crates which compiled with 1.0 still compiled,
which is a big deal for a language still being developed this actively. In any
case, keep your eyes out for those changes coming in March. I'll link the
forum announcement and the relevant RFCs in the show notes!

Now, let's talk some more about traits!


Overview (1m -> 4m)
--------
Last week, we talked about both generics and traits, but at a fairly high level.
This week, we're going to build on that by getting down into the nitty-gritty of
what using generics and traits looks like in practice. There's not going to be
much test code in the show notes this week. Writing enough code to be meaningful
for this would be, well... a lot. Instead, I'm going to refer to the standard
library docs, because they have a *ton* of generics and traits in them. In fact,
a lot of what might feel like core behavior of the language is actually just
standard library functionality, implemented in terms of generics and traits.


`Iterator` (8m -> 12m)
----------
Let's start by talking about the `Iterator` trait, which is a perfect example of
something that provides enormous functionality, but is just a trait---and, apart
from the syntactic sugar that the compiler provides for using it, one you could
write yourself.

As an aside, if the main ways you've been working at learning Rust are through
the Rust book, or Rust by Example, then you should definitely start digging into
the documentation proper. In many cases, the standard library documentation
includes not only definitions of the specific types and functions and methods
defined in a given module, but also great information beyond what you'd find in
the more tutorial-like walkthrough in the books. Those tutorials, like the show
notes pages, link directly to the underlying source, as well, which is also
extremely illuminating.

### `Iterator`
I'll quote from the `std::iter` module to start, in fact:

> The heart and soul of this module is the `Iterator` trait....
>
> An iterator has a method, `next()`, which when called, returns an
> `Option<Item>`. `next()` will return `Some(Item)` as long as there are
> elements, and once they've all been exhausted, will return `None` to indicate
> that iteration is finished. Individual iterators may choose to resume
> iteration, and so calling `next()` again may or may not eventually start
> returning `Some(Item)` again at some point.
>
> Iterators are also composable, and it's common to chain them together to do
> more complex forms of processing.

Let's break this down.

The module itself is built on the `Iterator` trait. There are about 3700 lines
of code in this module, including all the documentation, so that's a pretty
substantial chunk of Rust's standard library dedicated to implementing and
explaining this one trait. As it turns out, though, that's really helpful as an
example for people *learning* the language: if core features of everyday Rust
are built out of the underlying machinery this way, then we can come up with our
own, equally powerful abstractions if we need them.

(As an aside, this is exactly what's going on in things like the regex module,
for example, which takes a number of pieces of core functionality, including
macros, and builds a very sophisticated library out of it. You can see the same
thing in any number of the other libraries out there.)

Back to the iterator module docs. The next thing we heard is a description of
one of the basic methods available to anything that implements the `Iterator`
trait. All iterators have a `next()` method, and it returns an `Option`. If the
iterator still has items in it, it'll be `Some(item)`; otherwise, if you've
exhausted the iterator, it'll be `None`. This same behavior exists whether
you're iterating over a vector, a string, a hash-map, a linked list, or quite
literally *any other* data type which implements `Iterator`.

Importantly, those types don't have to be "collection" types. To borrow a page
from the _Rust by Example_ book, for example, you could implement a Fibonacci
number generator in terms of `Iterator`, with `next()` always generating the
next value in the sequence. (Because it's an infinite sequence, you'd never
return `None`, always `Some(integer)`).

Now, the `Iterator` trait also defines dozens---and I'm not exaggerating here,
literally dozens---of other methods besides `next()`. But as we discussed
before, you can supply default implementations for trait methods, and that's
exactly what `Iterator` does.

A great many of Rust's facilities for functional-style programming appear in the
`Iterator` trait, so if you want to be using map or fold or any other such usual
suspects from functional programming, `Iterator` is your source.

### Composition
Wrapping up that description, the last thing I quoted was the assertion that
iterators are *composable*. This is precisely why those functional patterns work
so well: you can take one kind of iterable and create another one, and another,
and in each case you can use the *exact same operations* on the resulting
iterable type. Why? Because all iterable types have those methods defined on
them---either from the defaults that `Iterator` itself supplies, or with a
custom definition which overrides those defaults.

So that's all great, but where it becomes really powerful is this: if you come
up with your own custom data type, and it makes sense to iterate over it, you
can implement `Iterator` on it by supplying a `next()` method, and you'll get
all of that behavior for free. You can run `for` loops over your bespoke data
type. For free.

Moreover, if you want to put multiple traits together, you can. You'll see this
all over the Rust standard library, and all over idiomatic Rust code in general.
Defining a new type? There's a good chance you'll want to implement a bunch of
traits for it---`Add` to add things, `Drop` to define custom destructor
behavior when the item goes out of scope, equivalence via `PartialEq` or `Eq`
depending on what kind of equivalence you have, ordering via `PartialOrd` or
`Ord` depending on what kind of ordering your set supports... you get the idea.

### `[derive()]`
I've skipped over this in previous episodes, but Rust also provides tools for
implementing some of the more common traits automatically. If you think back to
our discussion of attributes in episode 7, you'll recall that I mentioned being
able to "derive" the `Debug` attribute. Well, what's actually going on there is
that `Debug` is a trait, and the Rust compiler is smart enough to know what a
normal debug representation of many data types is, so we can get that for free
instead of going through the tedium of implementing that trait over, and over,
and over again. In fact, *many* built-in traits can be "derived" this way, which
lets you create quite sophisticated behaviors around your custom types without
needing to do the heavy lifting yourself except for the places your type
*differs* from the baseline.

In short, we get some really powerful abstractions, allowing us to create
sophisticated behaviors around even complex custom types, just by composing
different traits together---but we're not paying a cost at runtime for it. It
all comes at compile-time. Neat!


Syntax (4m -> 16m)
------
Let's talk about `trait` syntax for a minute. Precisely because composition of
traits is the main way we define complex types in Rust, the syntax is a bit less
verbose than for interfaces in languages like Java or C#. Instead of an
`implements` keyword, we can simply include the name of the trait bounds we
specify on a given generic immediately after it. If we are generic over a type T
which implements the *add* trait, we'd spell that `T: Add`.

We can also use the `where` clause to do this. You can think of `where` clauses
as a way of spreading things out a little so that your type signatures,
especially for complicated functions, don't make your eyes explode. Instead of
annotating the generic bounds where they are defined, you can define them later.
So you might define a generic type `T` in your function definition, and at the
end of the definition---after the arguments, but before the curly brace which
opens the function body---you would include `where T: Add`. This isn't a big
deal where you only have one argument with one trait applied, but it becomes
quite useful if you have multiple arguments which have to conform to certain
trait constraints.

What about defining a new trait? It's about as you'd expect: you simply write
`trait` followed by the name of the trait, and then the usual curly-braces to
mark off the body of the `trait`. If the body of the trait is empty, it's what
we call a *marker trait*. You can use these to say, "This can be used in a
specific context" when that context doesn't require given functions to be
implemented. The Rust book gives the example of the `Send` trait, which
indicates that it is safe to send a given type between two threads. That doesn't
require a function, but it *does* require you to make sure that the type is
memory-safe in certain ways!


Limitations (1m -> 17m)
-----------
As powerful as all these tools are, Rust's traits can't do everything. And
specifically, as of today you can't implement the same trait in a more specific
way if it makes sense for a given type if there's already a concrete
implementation in place. This is a serious blocker for some kinds of
optimizations, and fixing it would lay the groundwork for a Rustic approach to
inheritance, which would be a nice addition to the language (even if we still
think composition is better in most cases). I'll link that RFC, as well as a
helpful blog post by the RFC author, to the show notes. That way you can take a
look if you're interested in some of the language design issues and their
ramifications for you as a developer.


Closing (2m 30s -> 19m 30s)
-------
A little teaser for the future: you can actually act on what are called *trait
objects*, where you provide a reference to the name of a trait as shorthand for
a reference to *some object which implements that trait*. When you do this, you
end up with *runtime* resolution, rather than *compile-time* resolution. In
other words: dynamic dispatch. We'll leave aside the details for now, but it's
helpful to know it's out there. If you see a function returning a boxed up trait
you'll know what you're seeing.

There's also an open RFC to act on trait objects in general, which will further
expand the utility of traits and of generic programming in Rust in some neat
ways.

Next time, we're going to talk about something that will sound awfully familiar
but prove extremely unfamiliar to you if you've spent a lot of time in C or C++:
Rust's *macro* system.

### Sponsors
Thanks to Hamza Sheikh and Chris Palmer for sponsoring the show this month!
Check out the list of other sponsors in the show notes. You are all amazing; I
really appreciate your support as I make the show. (So does my wife!)

If *you'd* like to sponsor the show, you can set up recurring donations at
Patreon.com/newrustacean, or one-off donations at Venmo, Dwolla, or cash.me.

### Follow/support
You can find show notes with detailed code samples illustrating these ideas, as
well as links to things mentioned on the show, at NewRustacean.com. You can also
follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If
you like the show, you'd make my week by rating it on iTunes, recommending it in
another podcast directory, tweeting about it, or just telling a friend!

Thanks to everyone who's added corrections and fixed typos in the show notes;
pull requests are great. I also love hearing from you! Definitely say hi on
social media, add your thoughts in the thread for the episode on the Rust user
forum at users.rust-lang.org, or shoot me an email at hello@newrustacean.com.

Until next time, happy coding!
