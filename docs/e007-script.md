Testify
=======

Intro (15s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is *Episode 7: Testify*.


News (2m)
----
The big news as I release this is that Rust 1.5 came out last week. From the
perspective of other languages, that seems a little crazy: since I started this
podcast, we've had two point releases, 1.4 and 1.5, and 1.3 came out shortly
before I recorded the introductory episode. Rust is keeping up with its promise
of "stability without stagnation", and frankly it's really fun to watch. One of
the great things about the release trains model pioneered by Chrome and since
adopted by many other projects, including Rust, is that you get forward motion
frequently, though the size of the changes in any given release may be
relatively small.

Rust 1.5 isn't a big release in terms of features. For the most part, it was
just fixing a bunch of little things in the language design and updating tooling
around the language: **cargo** got a new *install* command, and a bunch of the
standard library functionality for dealing with the file system and file paths
was stabilized and so is now available on the main release. In practice, this
means that if your code worked last week, it will continue to work this week; it
will just compile faster and you'll have some new tools to use.

Only one other thing particularly caught my attention in the last couple weeks,
as I've been busy with travel, but that thing was extremely interesting. Sean
Griffin, a guy who is currently employed to work on Rails and particularly its
SQL tooling has released Diesel: a Rust SQL ORM and query builder. My initial
impressions are that it is really interesting. It is an attempt to build an
easy-to-use Rust toolkit for dealing with SQL, while also providing the great
type safety guarantees that make Rust so attractive You can listen to Sean talk
about it a bit on episodes 31 and 32 of The Bike Shed.


Overview (4m)
--------
Onward! Time to talk about tests! Why? Because tests help us know whether our
code is broken. A good test suite gives you two good things:

 1. It tells you whether your code does what you think it should presently.
 2. It lets you make changes and tell whether your code *still* does what you
    think it should.

Importantly, to the extent that you've written a good set of tests, it can do
those things far more rigorously than you can by repeating a set of tests
manually. The computer won't forget a test in your test suite. If you catch a
new bug, you can just add a test to the suite, and when you fix it, you can be
sure that it'll be caught by the computer if you do something that breaks it in
the future. That's incredibly valuable.

Again, the qualifier, though: your tests can't guarantee the correctness of your
program. They can, however, help you be *more* sure that it is correct, if there
are enough tests, of the right sort, and they can help you be sure that even if
things are broken, they stay broken in the same way between changes. That's
still quite valuable in its own right, though!

There are two big categories of tests: unit tests, and integration tests. Rust
has support for both. But first, a note on how unit tests especially might look
a little different in Rust than in other languages.

In dynamically-typed languages like Python, Ruby, or JavaScript, one of the
major functions of unit tests is to verify that functions treat their arguments
correctly, including if you hand them invalid types, and that they return not
only the right values, but even the right *kinds* of values. You don't want a
function which is supposed to convert from a string to a number to hand back a
string, for example. Many of *those* kinds of things are taken care of by the
compiler itself in Rust and other statically typed languages.

(This is one reason that test-driven development is much more popular in the
communities around those dynamically-typed languages. Another is simple history:
unit testing as a practice simply developed after C was well-entrenched, and the
tooling around it has never been as good as a result. Many modern languages,
including Rust and Elixir, have built-in support for testing, and this makes it
far more likelier that developers *will* write tests than if they have to use a
third-party framework.)

Even though static compilation with a strong type system solves one set of
problems, though, it doesn't tell you whether the *values* you got out of a
function were correct. Knowing that you did in fact get a number out of a
function which converts strings to numbers helps---but did you get the *right*
number back? Testing that requires that we actually call the function with an
input whose result we know ahead of time and see what we get. Unit tests, in
other words!

In Rust, you probably won't find yourself writing as *many* tests as you would
in Ruby or Python or JavaScript, but you still need them. The compiler doesn't
guarantee anything about your algorithm; all it does is tell you whether you
handled your types correctly.


Tests in Rust (11m)
-------------

### Overview (0.5m)

There are three basic kinds of tests in Rust: test functions, documentation
tests, and integration tests. In addition, nightly Rust supports benchmarking,
which a different sort of test but an important one.

### Attributes (2m)

In order to talk about testing, however, we first need to talk about a set of
flags we will be using around the modules and functions we will use for testing:
*attributes*.

  - Attributes are special flags set to tell the compiler to do something with a
    given definition---whether of a module, a `struct` or `enum`, or a function.
  - They are currently defined by the compiler, and *only* the compiler, in
    stable Rust. You can define them yourself using compiler plugins, but
    compiler plugins are an unstable feature: the Rust core team has not decided
    on the best way to handle them going forward.

What are attributes used for? Lots of things!

  - As we will discuss in a minute, they can be used to specify tests.
  - `#[derive]` can be used with relatively straightforward data types to create
    default debug and display formats, so that you don't have to define the
    formatting for most structs, for example.
  - `#[cfg()]` can be used to specify configuration for modules used by
    **cargo** as well as by **rustc** and **rustdoc**.
  - `#[feature()]` is used to turn off or on specific features, useful when
    dealing with nightly Rust builds, where some features may be available in
    master but disabled by default because they aren't ready for release.
  - As both of these last two imply, some attributes can take *arguments* which
    further specify their behavior.

You get the idea; there is a lot more we can say about attributes in the future,
and we will, but that's enough to get started for talking about tests.

### Test functions (3m)

  - Functions marked with `#[test]` are treated in a unique way: when running
    **cargo test** on the command line, **cargo** executes each function marked
    with the attribute, and indicates whether the test returned successfully.
  - These functions are never compiled into your release code. This is great: it
    means you can write your tests right next to the code they check, without
    impacting the size of the executable you distribute.
  - Within tests, you can also use a set of dedicated macros like `assert` and
    `assert_eq` to indicate how you expect something to behave.
  - If the result of some function call *should* be broken, that is, it should
    "panic", you can set the `#[should_panic]` attribute on the test! Also, you
    can supply an argument to `#[should_panic]` to account for specific *kinds*
    of failures in a test.
  - You can also set an `#[ignore]` to tell **cargo** to ignore certain tests by
    default. You can then override that by passing an extra argument to
    **cargo**---handy if you have certain tests which take a long time to run,
    for example, and only want to run them occasionally.
  - The best practice is to wrap tests in a module private to the containing
    module, and set the `#[cfg(test)]` on the module. This lets you write
    support functions within that module which are excluded from normal builds
    just like functions marked with `#[test]`.
  - (By convention, we call that module `test`, but it *must* be marked with the
    `#[cfg(test)]`.)

So that's how you *write* these kinds of tests. What would you use them for?
Well, this is where you would put unit tests. Within the `test` module, you can
verify that the various functions you call work as expected, by comparing the
results of a function with their expected results.

This is particularly powerful when combined with Rust's expressive type system,
because you can, for example, test functions to be sure that you actually get
both the right enumeration and the right value within it from a function which
returns an enumerated type. For the simplest example, you can just check that
calling a function which returns an `Option` gives you either a `None` or a
`Some` with a value in it, and also check that the value is what you expect.

### Documentation tests (2m)

The second type of test we have available is a "documentation test." Like Python
and Elixir, Rust can run any samples you include in your documentation strings,
and in fact **cargo test** will do exactly this. And this gets you something
distinct from what unit tests give you: it helps you make sure that your
documentation itself is correct and up to date. If you change an API, but forget
to update the comments on the module to reflect that, **cargo test** will give
you a failure, and point you to the docstring! (Note that this only works for
library crates, not binary crates. In fact, this is true of tests in general. I
haven't yet had a chance to look for the reason that is so; I'd love to hear
about it.)

Any and all code samples included in your documentation blocks will be executed.
Conventionally, we mark off a section with a first-level header titled
"Examples" for these kinds of things, but **cargo test** will catch *any* code
in your samples. If you want to turn off a given example (e.g. because it is
supposed to fail), you can just add `ignore` to the very start of the code
block, and **cargo test** will ignore it. On that note: thanks to GitHub user
**raindev** for submitting a pull request for the show notes which did just that
for some of the negative examples in the notes for episode 5!

### Integration tests (2.5m)

That covers most of the kinds of things we might want to do with the code in a
given module. What about testing that all the pieces we've written work
together, though? For this, we have integration tests. The best way to do this
is to come at it just like we were any other external user of out library:
import the pieces we want to test and check that they work as expected. So we
can create another library-type crate, add functions marked with `#[test]`, and
run **cargo test** on that crate. The standard practice for Rust projects is to
name that test crate `tests`, just to be clear what the crate is for.

These kinds of integration tests can help us be sure that our public API is
behaving as we expect, that its pieces work together as they should, and indeed
that our public API is actually what we think it is. If, for example, we forgot
to make a given module or function public with the `pub` keyword, an integration
test that tried to `use` it would fail.

Now, what kind of thing would you use this for in particular? Imagine you were
writing yet another Markdown parser: you might want to have parse and render
functions. Within the unit tests, you would have tests that handle specific
kinds of elements, which you would compose together to create the actual parsing
functionality. To test whether a given document resulted in the result you
expected, though, you would probably need a non-trivial Markdown document to run
through your main public parsing and rendering function or functions. As with
the unit tests, this would give you a straightforward way of determining whether
any changes you introduced broke existing functionality---especially important
if you were to be extending the Markdown syntax, for example.

### Benchmarks (2m)

Finally, we have benchmarks. These let us tell how *fast* something runs. One of
the most common places we see benchmarks employed is comparing competing
libraries. Far more usefully, though, if there is a performance-critical section
of your code, you can use this to make sure that changes you make don't cause
things to slow down. To return to that Markdown-parsing example, *both* of these
would be useful. If your library can parse Markdown twice as fast as any other
Rust implementation, that really is useful for people to know. But at least as
important, if you decide you want to add footnote support, rather than just
supporting the original footnote-less syntax, it's extremely helpful to know
whether the additional syntax-checking you do has a serious impact on the speed
of your code. If you suddenly take twice as long to parse a document because
you're checking for footnotes, you know that you've probably done something
wrong in your implementation, and it will help you find what and where.

Benchmarks are currently behind a feature flag, which also means that they're
only available on nightly Rust release. Using a nightly build, you can do
benchmarks by adding the `#[feature(test)]`, and then adding the `#[bench]` to a
function definition, and adding a special argument---a mutable reference to a
`Bencher` type. Within the body of the function, you call the `iter` method on
the `Bencher` instance  with a closure taking the functionality you want to
benchmark. Then when you run **cargo bench**, you will get benchmark results for
that particular functionality. Handy!

Closing (1m)
-------
Next time, we'll take an introductory look at Rust's generic types and its
`traits`, which are the foundation of really expressive programming Rust. I say
"introductory" because I expect a full discussion of those concepts first of all
to take more than one week and secondly to be a bit beyond me yet!

### Sponsors
Thanks to Chris Palmer for sponsoring the show this month! See the list of other
sponsors in the show notes.

If you'd like to sponsor the show yourself, you can set up recurring donations
at Patreon.com/newrustacean, or one-off donations at Venmo, Dwolla, or cash.me.


### Follow/support
You can find show notes with detailed code samples illustrating these ideas, as
well as links to things mentioned on the show, at NewRustacean.com. You can also
follow the show on Twitter or App.net @newrustacean, or follow me there
@chriskrycho. If you like the show, please rate and review it on iTunes to help
others find it. Last but not least, I'd love to hear from you on social media,
in the thread for the show on the Rust user forum at users.rust-lang.org, or via
email at hello@newrustacean.com.

Until next time, happy coding!
