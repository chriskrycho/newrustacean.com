No More Nulls
=============

Intro (30s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is episode 3: No More 
Nulls.


News (2m)
----

  - Shout out to GitHub user romankl, who submitted a PR to the show to fix an
    error in a link. So good!

  - Work on IDE support!
      + My chosen tool: JetBrains/IntelliJ
      + Also: Eclipse, Visual Studio

  - Learning tools I just heard about:
      + Rustlings (Carol Nichols)
      + Rust FFI Omnibus (Jake Goulding)


Overview (3m)
--------
Why do we care?

  - Meaningful return types.
      + C special error values
      + C/C++ `void`, Java `null`, etc.

  - Pattern matching -> sophisticated way of handling things that goes way
    beyond what a `switch/case` statement can do, and `enum` types are a great
    way to start with those (though enums can do much more, and so can pattern
    matching).


enum types (4m)
----------

  - Sum types vs. record types (i.e. enums vs. structs)
  - Differences between these and C/C++ types
  - Things enums can hold


Pattern-matching (5m)
----------------
When matching against a pattern---any pattern, not just enums!---the pattern
match must exhaustive. If you forget something, the code won't compile.

  - Not like if/else blocks where you can forget conditions
  - Kind of like switch/case blocks, but not limited in the same ways (can match
    on any type, and on different types described by enums)

Lots of places where this might be useful:

  - handling return values exhaustively (more in a minute on this)
  - writing a finite state machine
  - interpreting responses from an API
  - parsing tokens in an actual parser

What kinds of things can you match?

  - Ranges of numbers
  - Enums
  - Strings
  - Tuples
  - etc.

You can also deconstruct these complex types to act on them. Match arms are
expressions, so you assign the result of a `match` to a variable (more on this
when we talk about expressions in a future episode).


Meaningful returns (2m)
------------------
One place pattern matching and enums combine to become extremely powerful is in
handling return statements.

  - `Option<T>`
  - `Result<T, E>`


Closing (1m)
-------
### Next time

  - Functions in Rust!
      + Regular functions
      + Methods
      + Closures

  - Gives us a good excuse to talk a bit about the "stack" and the "heap" as
    well; we'll *introduce* those ideas along the way.

### Follow/support

You can find show notes with detailed code samples illustrating these ideas, as
well as links to things mentioned on the show, at NewRustacean.com. You can also
follow the show on Twitter or App.net @newrustacean, or follow me there
@chriskrycho. If you like the show, please rate and review it on iTunes to help
others find it, and if you *really* like the show, I'd welcome your financial
support at Patreon.com/newrustacean. Last but not least, I'd love to hear from
you on social media, in the thread for the show on users.rust-lang.org, or via
email at hello@newrustacean.com.

Until next time, happy coding!
