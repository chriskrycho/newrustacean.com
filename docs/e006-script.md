Modularize this!
================

Intro (30s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is **Episode 6:
Modularize this!**


Resources (1m)
---------
A couple things came across my radar this week that you might find interesting
as you're learning Rust.

  - Roguelike in Rust---a tutorial walking through the process of building a
    text adventure using Rust.
  - A screencast by Intercom with Yehuda Katz (of Ruby, EmberJS, and Rust fame)
    on using Rust as a replacement for C for performance-critical modules with
    Ruby code (but readily applied to other languages like Python, etc.).


Overview (1m)
--------
We've spent the last few weeks talking a lot about functions, and for good
reason: functions are the basic tool for building programs. Today, we're going
to turn our attention to a related set of ideas in Rust: modules and "crates,"
Rust's name for what you might know in other languages as *namespaces* and
*packages* respectively.

Together, these two give us the tools we need to *organize* all those structures
and enumerations and functions we discussed. Why do we care? Well, because it is
difficult to understand, to organize, to reuse, or to maintain any large set of
code without these tools! We'll take them in turn.


APIs (5m)
----
An API---originally "application programming interface" but now used more
generally to describe the interface between libraries as well---is defined by
the publicly available set of functions and data types, and their organization.
We have enough data now to start thinking about APIs and the way they are
structured, because we now know about all the basic building blocks for defining
them in Rust.

We had to wait until today, though, because until you know how a given language
allows you to *organize* your data, you can't really talk about an API. But now
we need to talk about APIs, because the API for a given library or program
matters, and it matters a *lot*.

A little over a week ago, I took a look at some code a friend was working on as
he played with a Rust library. He was having a hard time getting the code to
read nicely, with multiple nested `match` statements. As we looked at it, we
both found ways to work around that, including for my part writing a small macro
for the first time, but our conclusion was that the API itself wasn't great. In
this particular library, the problem was the way the `Option` and `Result` types
were used: things were wrapped too many times in one or the other or both, so
that nearly every operation in the library required pattern matching or
unwrapping values.

That highlights one of the major issues in API design: you don't want to nest
things deeply---whether in modules or in complex data types---unless there is a
*very* good reason to. To quote the Zen of Python: *flat is better than nested*,
and *simple is better than complex*. With Rust's module and type systems, we
have powerful tools we can use to model data in a powerful way. However, we
still need to take care to design our APIs with those maxims in mind. The fact
that we *can* nest things fourteen modules deep and wrap everything in `Result`s
inside of `Option`s inside of `Result`s inside of `Option`s doesn't mean we
*should* do that.

You might be thinking, "Well, but in *my* library, that complexity is
necessary!" Beyond the obvious question---is it *really*?---it is important to
remember that one of the points of building a library is to *hide* the
complexity of doing whatever your library does from an end user (first of all
yourself in most case). In other words, our goal in most cases is to provide a
useful abstraction which makes it so that end users *don't* have to deal with
that complexity.

One helpful approach is to take advantage of the difference between public and
private modules and functions: if you need that complexity *within* your
library---and sometimes you really do---then use it appropriately. But don't
unnecessarily expose your API's consumers to that complexity: supply flatter,
simpler data structures and functions to them however possible, with as flat a
namespace as possible, to make it easier to use your library.

So, how do we do that in Rust?


Packages (5m)
--------
Packages, the highest level of abstraction for organizing Rust code, are the organizational level which correspond to and indeed get built into libraries in Rust. Here, we call them "crates." (Thus the name of the Rust package manager, Cargo! Rust's naming choices are quirky... but I like them.)

Language-level packages, especially when combined with a hosting solution and a tool for managing them, are extremely handy. Although they can have their downsides---dependency resolution can be painful---these tools are a major part of what has made ecosystems like those around Python, Ruby, and NodeJS so effective. Nor is this just a "scripting language" concern: even languages like Java and C# have had package managers for some time, now. Having used them extensively, I feel their absence keenly in C and C++, where no solution has really caught on with the community: largely, I suspect, because those communities grew up long before package managers became a norm.

With Rust and Cargo, we are trying to learn from the best and worst parts of the
existing ecosystems. Every generation of programming languages and tools has the chance to learn from preceding generations, and just as Ruby's packaging tools improved on Python's in a variety of ways, and Node's npm represents, if not a clean win, then still a useful experiment, Rust now has a chance to take those lessons and apply them again, but in the slightly different context of a
statically compiled language.

We define packages in Rust using a `Cargo.toml` file; TOML files are just a
lightweight markup a lot like the old ini format. The format for these files is
specified in some detail on crates.io. The main thing is that we define the path
to the main source file for a package and give it a name there. (Helpfully,
running `cargo new` on the command line will help us do this automatically.) For
libraries we want to reuse, or distribute, that root file is usually at
`src/lib.rs`, and for binaries, at `src/main.rs`.

Analogous to a Bundler lock file, or the output from `pip freeze`, Cargo generates a `Cargo.lock` file when you generate a build. This specifies not just the basic dependencies you specified, but the *exact* versions used when you made the build. This makes it possible to have readily reproducible builds---which is a big deal, as anyone who's spent time with npm knows!

Once we have a crate we're happy with, we can then upload it (and the docs we
built for it!) to crates.io, and make it available for others to use in the same
way if we so desire. Crates are also the basic unit we can use for linking Rust
with *other* programming languages---see the screencast by Yehuda Katz that I
mentioned at the beginning of the show for an example.


Modules (5m)
-------
One of the two major ways we break down the functionality of a given library is through what are often called *namespaces*. Names are one of the hard problems in software, and in particular we need to come up with a way to handle the situation where you and I both name a function the same thing. You can imagine, for example, a regular expression library, a compiler library, and an HTTP request library all implementing a `parse` function. Which one are we referring to? How should the compiler know whether the function call we are making is valid or invalid if it needs to choose between variants (especially if we don't have overloading---and we don't, in Rust)?

The common pattern for this is *namespaces*. In C++, these are managed through namespaces. In Python, they come in *modules* and *packages*. In Rust, they come in *modules*.

  - Modules are the basic structural tool for organizing discrete groups of code
    in Rust.

  - They're very similar in many ways to modules in Python (though with some
    important we'll get to momentarily), and to namespaces in C++, even sharing
    some of the basic syntax of the latter.

  - You define a module in Rust by using the `mod` keyword.

  - Just as with other items which make up the contents of module, modules themselves can be declared public (with the `pub` keyword), but are private by default. That is, you can define a bunch of modules (whether in the same file or an external file) for organizational and namespacing purposes, but you don't have to expose all of those---whether to other external modules, or at the package/crate level.

  - As my comments so far have suggested, you can define multiple modules in a given file, using curly braces to a block which defines the contents of that particular module.

  - As you can imagine, though, this gets unwieldy quickly for a package of any substantial size.

  - Thus, the modules declared with the `mod` keyword can map to *files*. For example, the show notes for this episode are declared as `pub mod e006` in the `lib.rs` source file which defines the root of the package. The file itself is just named `e006.rs` and lives at the same level as `lib.rs` in the project structure.

  - You can also use folders to organize your modules (somewhat like you can in Python, but with the difference that files and folders do not *define* the module structure the way they do by default in Python).

  - If, for example, I wanted the show notes for each episode to live in a folder instead of just a single file, the notes for this episode would be at `e006/mod.rs`.

  - To use another module, you use the `use` keyword and specify the name of the module. If it is a nested module, you separate each name with a pair of colons. The full path to this episode would be `show_notes::e006`.

  - You can also reference specific items in a given module by referencing them in the same way. If you want to reference more than one item within a module, you can supply the path up to that module as usual, and then wrap all the items you want to `use` with curly braces, separated by commas.

  - The `use` statement lets you alias modules or items, too. If you have some long module name that you're going to reuse frequently, you might give it a short name. (Pythonistas might think of the common pattern of importing numpy as `np`, for example.)

  - One last thing we can do with modules: we can re-export a given module. This is convenient for defining the top-level interface to a given package. If you have a reason to have a deeply nested file structure, you might still be able to supply a nicer API to the package by re-exporting those modules at the top level, or at least a higher level, so there is a friendlier interface for people using the crate.


Closing (1m)
-------
### Next time
And that's a good spot to wrap up for this week! Next time, we'll talk about
*testing in Rust*.

### Sponsors

Thanks to Chris Patti for sponsoring the show this month! See the list of other
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
