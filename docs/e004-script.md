Functionalized
==============

Intro (30s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is episode 4: TBD


News (2m)
----
First up, thanks for sending in more pull requests!

  - Thanks to kstep (Konstantin Stepanov), Dashed (Alberto  Leal) and ctjhoa
    (Camille TCHOA) for fixing some links in the show notes.

  - Thanks to zazdxscf (Emanuel Czirai) for pointing out some problems in the
    code samples from last week!

  - Thanks to David Krycho (yep, that's my dad!) for asking me some good
    questions about last week's episode content and show notes, which have led
    to some improvements I'll push up to the example code sometime later this
    week!

Second, progress continues apace on the IDEs I mentioned last week: the IntelliJ
IDEA folks have made a *ton* of progress since I recorded, and though you have
to build the plugin yourself, it does support some of the basic syntax
highlighting functionality and the like.

Third, a few more learning tools you might find handy:

  - Exercism.io has some Rust exercises (HT: Lechindianer on GitHub/Tw/ADN)
  - Camille TCHOA's GitHub repository rust-learning
  - If you speak German, rustplatz.de. I can't read a word of it, but it looks
    like there's some solid stuff on there (again via Lechindianer)


Overview (3m)
--------
Last week, I said we were going to talk about functions, methods, and closures,
and that's sort of true. However, to talk about functions in Rust, we're going
to have to spend a little time talking about *types* as well, and that means
we're not going to get quite as far in our discussion of functions as I
originally thought.

Today, after getting our feet under us a bit with what we actually mean when
talking about "functions" both in general and also in Rust specifically, we'll
cover the very basics of how Rust handles normal functions, methods, and
closures. We'll conclude by showing how some of those conceptual issues come
into play by showing how you can pass around functions as arguments, precisely
because functions are just another type in Rust. Next week, we'll pick up and
carry on with the idea of *returning* functions, and see how that leads us
further into the type system and into how Rust manages memory.


Functions (High Level) (5m)
----------------------
Functions: building blocks of sane programming practice. Who *hasn't* seen
nightmare code where functions were thousands of lines long and did dozens of
different things?

How functions work, and what they are in the language, make a huge difference in
how your language feels and what kinds of things are normal in it.

  - Are functions first-class members of the language, at the same level as any
    other object (whether an integer or a struct/class/etc. instance)?
  - How hard/easy is it to use functions in composition?
  - Are there anonymous functions?
  - Are there closures?
  - Are there methods?
  - Are there non-method functions?
  - Can you define functions inside other functions?
  - Do functions have types, and can you use those types the way you can use
    other types in the language?

How your language answers these questions determines what kind of language you
have. For example:

  - Old-fashioned Fortran has *subroutines* more than it does *functions*. They
    are capable... as subroutines. But they're not high-level functions, at all.
  - C and Java, for all their differences, answer many of these questions the
    same way. The main way they differ are in having methods. Neither can
    *really* treat functions the same way they treat, say, integers.
  - Python and Ruby, by contrast, have functions that can be assigned and
    managed just like any other type in the language. In Python, functions are
    objects, which can have methods attached! However, Python doesn't really
    expose the *type* of the function in a way that you can use (as indeed it
    doesn't with *any* types, relying philosophically on duck typing instead).
  - In Haskell and ML-descended languages (OCaml, F#, and Rust!), functions have
    types, and can be used as such.


Functions in Rust (3m)
-----------------
  - A theme of Rust is *strong, static typing*. Nearly everything in the
    language has a type, which is known at compile time---functions included!
  - Basic components of the type of a given function are its argument types and
    return types.
  - Talk more about types in the future, because those types can be further
    constrained in various ways.
  - Methods are a special case of ordinary functions: and (from what I
    understand so far) they only get special behavior when passed a reference to


Closures (2m)
--------
Big difference with closures from functions (anonymous functions) is that they
capture their environment. Use slightly different syntax to emphasize this.

  - Gets into ownership and another Rust concept, *lifetimes*, because they
    maintain access to elements defined in their scope.
  - One way of doing information hiding (and easily the most common in e.g. JS).

Closures and functions both implement the `Fn` `trait`. (This is one of the ways
that types can be constrained and defined!) We'll talk more about traits now;
today suffice it to say that traits make up the basic way of defining conformity
to a given interface---_a la_ mixins, protocols, and interfaces in other
languages.


Functions as arguments (3m)
----------------------
  - Both normal functions and closures can be passed as arguments.
  - Closures particularly useful for two reasons:
      + They can be defined where the function call happens, so for brief
        definitions this can be clearer (e.g. `map` or `reduce` on an array).

      + As noted a minute ago, they can bring their environment with them.

        Scenario: say you have a function that expects a function taking one
        argument as a callback, but you need to do some manipulation in the
        function you pass in using data in your *current* environment. Two
        alternatives:

         1. Use a closure directly, where you just have access to the local data
            you need without passing it as arguments.
         2. Create a factory to build the function you need... by creating and
            returning a closure. This is a bit more complicated, but there are
            times when it's the best way to approach the situation---especially
            when the body is complicated, or when you need to build similar
            functions repeatedly.

  - To define a function taking a function as an argument, you just supply the
    *type* of the function as a `trait`. Then the operation is *generic*: it can
    operate using any function that matches the constraints defined by that
    trait.

    See the code samples to get an idea of how this works in more detail.


Closing (1m)
-------
### Next time
Last week, I said we'd get all the way through functions and into the stack and
the heap, but as it turns out, there is a *lot* to cover in functions. So, next
time we'll pick up from here and talk about returning functions, and *that* will
lead us into a discussion of the stack and the heap, and possibly---but no
guarantees!---into some early discussion of functional programming.

### Follow/support

You can find show notes with detailed code samples illustrating these ideas, as
well as links to things mentioned on the show, at NewRustacean.com. You can also
follow the show on Twitter or App.net @newrustacean, or follow me there
@chriskrycho. If you like the show, please rate and review it on iTunes to help
others find it, and if you *really* like the show, I'd welcome your financial
support. You can set up recurring donations at Patreon.com/newrustacean, or
one-off donations at dwolla.com/hub/chriskrycho. Last but not least, I'd love to
hear from you on social media, in the thread for the show on the Rust user forum
at users.rust-lang.org, or via email at hello@newrustacean.com.

Until next time, happy coding!
