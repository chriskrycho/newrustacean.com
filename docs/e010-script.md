Macros rule!
============

Intro (30s -> 0:30)
-----

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is *Episode 10: Macros
rule!*


News (1m 30s -> 2:00)
----

A couple items of interest from around the Rust community:

  - Julia Evans has a great set of slides titled "Why I (Heart) Rust" which was
    a ton of fun and has a really delightful art style to it. She also has a
    series of blog posts about writing a small operating system in Rust. Those
    posts were back in 2014, well before the language stabilized, so there are
    a bunch of things which have changed since then, but they're still quite
    interesting.
  - Speaking of operating systems, Rust community coordinator Steve Klabnik has
    started a project call IntermezzOS, which a small teaching operating system
    built in Rust. It specifically focuses on (and I quote from the landing
    page), "introducing systems programming concepts to experienced developers
    from other areas of programming."

For my part I'm hoping to play with IntermezzOS here and there over the next few
months, just because I'd like to learn some of those pieces myself.

For today, though, let's talk about something totally different: macros!


Overview of macros (2m -> 4:00)
------------------

So far, we have focused entirely on what I would call the more "normal" features
of Rust. Even though there have been some substantial differences between Rust
and other languages you might know because of its type system, most of the
things we've done are still pretty familiar. Today's topic takes into a bit
stranger territory, though. Today we're talking about *macros*, which are Rust's
way of providing ways of transforming your code apart from the usual suspects of
functions.

I said "transforming your code" because macros actually working on the abstract
syntax tree that makes up the result of the Rust compiler's first pass on the
source you hand it. When you define macros, you actually *transform* that syntax
tree into a *different* syntax tree.

Rust has two kinds of macros today: the transformations we'll be dealing with
today, which we might call *syntactic* macros, and compiler plugins, often
called *procedural* macros. There are also plans for dramatic improvements to
the syntactic macro system in the future, which I'll come back to and talk about
a little bit at the end of the show.

There are two reasons I'm focusing on substitution macros today:

 1. They're way easier to deal with, and I understand them *decently* well---at
    least well enough to use them a little here and there, and to explain them
    decently. I can't say the same for compiler plugins, which let you run
    arbitrary code in the compilation phase! That's still a bit beyond me.
 2. They're *stable*. Compiler plugins presently only work in nightly Rust. The
    language development team *is* interested in making that functionality
    available to everyone... eventually. But they're still thinking about the
    best way to do that, and until they have a solution everyone is comfortable
    with, they're not going to stabilize it, precisely because of the guarantees
    Rust makes about stability.

So for now, we'll just stick with the simpler substitution macros, which are
stable and a bit easier for regular Rust developers to pick up and understand.


Macros by Example (13m 30s -> 17:30)
-----------------

### Discussion (4m 30s -> 8:30)

Properly speaking, we call these substitution macros "macros by example" after
the paper which formulated them back in the eighties; I'll link it in the show
notes. Fair warning: it's heavy on parentheses, because it came out of Lisp, and
on math.

In any case, the macro's own body defines how the substitution behaves. Rust's
macros are not running *arbitrary* changes on the syntax tree; that's limited to
the compiler plugin approach. Instead, they're a fairly sophisticated way of
doing *syntax substitutions*.

If you're familiar with preprocessor macros in C and C++, you can think of these
macros-by-example as being like those---only much more powerful *and* way safer.

Another important distinction: in C and C++, preprocessor macros are exactly
what they sound like: *pre*-processing. They transform the *text* in the source
code, before any compilation occurs. And that's *all* they do in terms of
transformations: they substitute one set of text for another. In Rust,
macros-by-example are instead an early part of the compilation process proper,
and they don't operate on *text*; they operate on the *abstract syntax tree*.

Another analogy would be to the text transformations you can accomplish with a
sophisticated regular expression engine---but again, operating on syntax instead
of on text elements.

This difference---that they operate on the *abstract syntax tree* rather than on
the strings in the source code, is the reason that Rust's macros are so much
more powerful and so much safer than C or C++'s.

You can perform macro substitutions on items, blocks, statements, patterns,
expressions, types, identifiers, paths, the sides of the fat-arrow operator
(like the one used in `match` arms), and attribute contents, and of course you
can mix and match those as you like. The big takeaway here is that you're
transforming an abstract representation of the syntax. You can certainly write
bugs in your macros, and because of how macro processing happens, they can be
non-trivial to chase down. But---and this is a big deal---you cannot do things
like clobber local variable names!

One of the most common mistakes people make when writing macros in C or C++ is
forgetting that it's just simple text substitution. If you use "x" in your macro
in C, and there's a local variable named "x" someplace where you use that macro,
those two things *are the same variable* when all is said and done, regardless
of what you intended. There are lots of ways people get around that---naming,
wrapping things in blocks, etc.---but they all derive from the fact that you can
only transform the *text of the source code* there.

Since that's not what we're doing in Rust, and because the macros are
implemented with the notion of "syntax context," we get a wonderful property
called "hygiene." If you define what seems like it should be an ordinary local
variable in a Rust macro, it *is* a normal local variable; it doesn't clobber
the names in use wherever the macro is applied. Thus, you can *safely* use
normal names, without worrying that you're accidentally shadowing and
potentially blowing away the values in some existing variable.

The combination of *syntactic substitution* with *hygiene* lets us write
powerful, safe macros which can make for much more readable code. Let's make
this concrete by looking at some examples of macros in the Rust library, and
then we can look at what's involved in building your *own* macros.

### Some built-in macros (3m 30s -> 12:00)

Okay, so now we have some idea what we're talking about with macros, but how do
we actually use them in Rust? And how do we build our own?

Using macros is basically like using a function, with two significant
differences. One, all of these macros have an exclamation point after their name
when you're using them. For example, the most common macro you'll run into is
probably the *try* macro, which you write `try!`.

And of course, if you've looked at almost any of the example code I supply with
the different episodes, or with any other Rust tutorials or documentation
online, you've probably already seen a bunch of other common items: I've used
the `println!` macro heavily, and once we got into the test code I started using
the `assert!` and `assert_eq!` macros, which both make use of the `panic!` macro
under the covers. So I'll offer some brief comments on each of these, as they're
among macros you'll see most often in Rust.

  - The `try!` is used for dealing with `Result` types conveniently. Imagine a
    function where you want to do a bunch of file operations, each of which can
    fail, and which accordingly return `Result`s. Every time, you want to return
    a `Result` from your own function if there is an `Err`, and continue if the
    operation succeeded: you don't want to try to write to a file if you didn't
    get back a file pointer, for example.

    Instead of writing `match` expressions for every `Result` in the function,
    you can just wrap them with `try!`, which does exactly that---but makes your
    own code way easier to read. (Note: you can't use the regular `try!` in
    `main()` because `main()` doesn't return a `Result`. But one of the example
    in the show notes this week is a version of `try!` which works in `main()`!)

  - The `print!`, `println!`, `write!`, `writeln!`, and `format!` are convenient
    wrappers around string formatting and printing or writing to buffers.
    `format!` is used by all the others to supply string formatting like you'd
    see in C's `printf` or Python's `str.format`. The *print* and *write* macros
    provide convenient ways to call the `std::io` module's functions for
    printing and writing to buffers, with format operations applied first.

  - The `assert!` and `assert_eq!` macros are usually used for testing; they let
    you test general boolean expressions and equality of values respectively. If
    they fail, they use the `panic!` macro internally.

  - The `panic!` macro is for unrecoverable, unexpected errors. You can think of
    it *kind of* like an exception you *can't* catch in another language. It
    causes a thread panic, which unravels the stack and dumps it to stderr. You
    shouldn't see these very often, but they are appropriate from time to time.

### `macro_rules!` (2m -> 14:00)

What about *writing* macros?

You create a new macro with (wait for it)... a macro! The macro-by-example
macro-creation macro is called `macro_rules`. You write it like any other macro:
"macro-underscore-rules-exclamation-point". It is used as a standalone item,
rather than in the midst of a function or expression like those discussed above.
(Of course, other macros can be written that way as well, and it may make good
sense to do so, depending on the kind of macro you're writing.)

So for example, in the show notes for this episode, I defined two macros so you
can see how they work. One is them is the variation on `try!` that works in
`main()` functions which I mentioned earlier. The other example macro I wrote
takes any identifier---a variable, a function, a type, etc.---and prints a
little message including its name. That second macro shows how macros can use
other macros, and in fact you can have recursive macro usage. (Good luck
debugging those, though! It's possible, but it's a lot of work.)

I actually came up with the idea for that `main_try!` when helping a friend get
going with Rust a few months ago, and discovered that it's not hard to write
some simple macros in Rust. I ended up needing to write a similar shortcut in
some C++ I was working a few weeks later, and let me just say: it's definitely
easier to write a macro without shooting yourself in the foot in Rust than in C
or C++.

### Downsides (1m 30s -> 15:30)

It isn't all sunshine and roses, though. Macros are powerful, but they are a
secondary tool, not the first thing you should reach for. As the Rust book says:

> The drawback is that macro-based code can be harder to understand, because
> fewer of the built-in rules apply. Like an ordinary function, a well-behaved
> macro can be used without understanding its implementation. However, it can be
> difficult to design a well-behaved macro! Additionally, compiler errors in
> macro code are harder to interpret, because they describe problems in the
> expanded code, not the source-level form that developers use.
>
> These drawbacks make macros something of a "feature of last resort". That’s
> not to say that macros are bad; they are part of Rust because sometimes
> they’re needed for truly concise, well-abstracted code. Just keep this
> tradeoff in mind.

In general, you should use the more standard language machinery: functions,
types, and traits. When you need them, though, macros are around, and they're
pretty great.

### Limitations and upcoming changes (2m -> 17:30)

There are a few things that macros can't do, or can't do well, even with
everything we talked about above. First, you can't use a macro in arbitrary
locations in code. You can't use them as the field in a struct, or as a trait
item, for example, because of how the compiler parsing sequence unfolds. Another
limit is that hygiene---that nice property that means we don't stomp all over
existing names, etc.---isn't fully fleshed out. It's better than in C or C++ by
a mile, but it doesn't apply to types, or lifetimes, so it still has some places
where it can throw a wrench into things.

There are also some quirks around macro names, because they don't get the same
namespace treatment that normal types, etc. do. (You can actually see this in
play in the show notes, where they don't end up in the module for this episode,
but in the crate at the top level.) And, annoyingly and unlike the rest of Rust,
the order you define them in matters! You can get around a lot of these issues
to varying degrees, but the limitations are still frustrating.

For a much more detailed discussion of these issues, you should see Nick
Cameron's series of blog posts on macros, which I'll link in the show notes. You
can also read about some of his plans for a proposed new, better approach for
macros that would stabilize the procedural macros we skipped over today, as well
as fixing a bunch of these issues along the way.


Closing (2m -> 19:30)
-------

A quick heads-up: this coming week, I'll be traveling to start a new job, and
probably won't have a lot of extra time, so you can expect either a bonus
episode or no episode next week. I have a short episode I'd *like* to do, and
which I'm hoping I'll be able to knock out next weekend after I get back, but it
depends on just how wiped out I am after traveling!

### Sponsors

Thanks to Hamza Sheikh and Chris Palmer for sponsoring the show this month!
You can see a full list of sponsors in the show notes.

If *you'd* like to sponsor the show, you can set up recurring contributions at
Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, or cash.me.

### Follow/support

You can find links to each of those, to other things mentioned on the show, and
notes and detailed code samples illustrating macros at NewRustacean.com. You can
also follow the show on Twitter @newrustacean, or follow me there @chriskrycho.
You can help *others* find the show by rating and reviewing it on iTunes,
recommending it in another podcast directory, tweeting about it, or just telling
a friend!

Thanks to everyone who's added corrections and fixed typos in the show notes;
pull requests are great. I also love hearing from you! Definitely say hi on
social media, add your thoughts in the thread for the episode on the Rust user
forum at users.rust-lang.org, or shoot me an email at hello@newrustacean.com.

Until next time, happy coding!
