Just like something else
========================

Intro (15s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is *Episode 8: Just like
something else*.


News (1m 30s -> 1m 45s)
----
Before we jump in, I apologize for the delay between episodes. Unfortunately,
I've been sick! I was just getting over one cold, and then got another---and
both seriously impacted my voice. But I'm back, and excited to be recording
again.

Next, a couple things that might be interesting to you. The first is the "This
Week in Rust" project, which highlights interesting projects in the Rust
community. You can follow it @ThisWeekInRust on Twitter or subscribe to their
weekly email at this-week-in-rust.com.

The second is for people interested in the high-level issues around implementing
languages, compilers, and even operating systems, and indirectly related to
Rust. Joe Duffy, the current director of Microsoft's compiler and language
platform group, has been blogging about his experience working on a research
project at Microsoft which involved a safe, highly concurrent language and OS,
with a number of lessons learned from that project. Aside from calling Rust
"just plain awesome," there's nothing directly related to Rust in the blog
posts, but they're quite informative about how another group tackled some of the
same challenges that Rust has been.

Now, let's talk about generics and traits!


Overview (2m 30s -> 4m 15s)
--------
One of the most important lessons you learn writing software is "don't repeat
yourself". Experience---often painful experience---shows that when you have the
same code in more than one place, things will go wrong when you change it. And
change is inevitable in software. So we always try to minimize repetition. This
can be challenging when we're dealing with different types, though. Multiplying
two floating point numbers isn't the same as multiplying two integers: at the
assembly instruction level, you actually have to implement those differently,
because you represent integers and floating point numbers differently. And in
fact, these kinds of differences even crop up within individual kinds of
floating point numbers. As a result, if you have a function that's designed to
operate on 32-bit floating point numbers, you need a separate function to
perform the same operation on 64-bit floating point numbers.

This isn't just a theoretical problem: C, for example, has three functions in
its standard library for calculating the power of a number: `powf`, `pow`, and
`powl`, for `float`, `double`, and `long double` types respectively. In C, you
*can* just use the `double` version, because the C compiler will implicitly
convert a `float` to a `double`---but you shouldn't, and if you cast back to a
`float` when you're done, you'll end up truncating the value. For that reason,
Rust, with its much stronger types, *won't* make implicit casts like that. Good
form is using the type-specific function. But doing that, whether in the
standard library or in your own code, means having multiple versions of
functions. And we don't want that. More than one implementation almost
inevitably means introducing bugs when we make changes.

We need a way to write functions---and other things, like types!---which can
work more *generically*. And if you've used a language newer than C or Fortran,
you probably have an idea of where we're headed.


Generics (6m 45s -> 11m)
--------
There are two basic ways of tackling this problem. In modern languages,
including C++, Java, C♯, Swift, Python, Ruby, JavaScript, and so on, we have
either *generics* or *duck typing*. (We might also talk about *interfaces*, but
there we'd be getting ahead of ourselves.) In general, the decision to use
generics or duck typing is at the language level: generics are the solution for
statically-typed languages, while duck typing is the solution for
dynamically-typed languages.

Generics allow you to specify to the compiler that a given function can operate
on more than one type. Rather than just the standard argument types for an
integer, a string, a float, an enum or struct, etc., we can specify a *generic*
parameter. Then we can call the function with any type which supports the
operations executed by the function. For a trivial example, we could write a
single `multiply` function which takes two arguments and multiplies them
together. We don't need to deal with overloading the function definition with
different type arguments---unsigned integers, signed integers, different sizes of
floating point numbers, etc.---because the compiler can handle them under a
single *generic* definition.

You get a similar benefit from "duck-typing" in dynamically typed languages. The
name "duck-typing" tells us what we mean: if it walks like a duck, quacks like a
duck, swims like a duck, etc., it's a duck---or at least, close enough to being
a duck for our purposes. To put it another way, as long as a given type has all
the attributes we need to operate on it---data, methods, etc.---it's a valid
parameter for a function taking a type which behaves in those ways. In a way,
*all* function calls, and all container types, are inherently "generic" in
dynamically-typed languages. That just falls out of dynamic typing. That's a big
part of what makes dynamically-typed languages so productive. So if you want to
increase the productivity of your statically-typed language, providing tools for
generic programming is a big step.

And of course, if you do provide tools for generics to statically-typed
languages, you get the other usual benefits that come with that: getting your
types wrong when you're trying to do something with a "generic" function or
container in a dynamically-typed language---a.k.a., any function or
container!---is a *run-time error*. In a statically-typed language with
generics, it is a compile-time error instead. So, for example, in Python, if you
pass a goose to a function that expects a duck, and the function calls the
object's *quack* method, when you run the program you'll get an `AttributeError`
telling you that the object simply doesn't *have* that method. Whoops. By
contrast, if your generic code in Rust *compiles*, it will *run*.

As always, this doesn't mean you can't introduce bugs. It just means you can't
introduce that particular kind of bug. Small victories, right?

Like in other statically-typed languages with generics---C++, Java, C♯,
etc.---Rust marks generic types with angle brackets around the "name" of the
generic type in function signatures or type definitions. By convention, a
run-of-the-mill generic type just uses a capital 'T' there: 'T' for "type". If
you have more than one generic, by convention you just keep cycling through the
alphabet, 'U', 'V', etc. (Of course, if you make it to 'Z' in a given generic
function definition, you should probably reevaluate your API design!) Exceptions
to this naming scheme include times when it's handy to indicate something else
about the type by using a different capital letter to represent it. For example,
in Sean Griffin's Diesel library, an ORM and SQL query builder, he uses `R` to
represent generic *row* types.

In all of this discussion about generics, I've left two major points unstated,
though. First, while we've limited our discussion so far to generics in
*functions*, Rust and other similar languages allow us to define types
themselves as generic. In fact, back in episode 3, I talked about two of the
major examples of generic types in Rust: `Option` and `Result`. And, as it turns
out, *lots* of foundational types in Rust are generic, including vectors, the
`Box` type we use for heap allocation, and all the reference-counted
types---lots of types.

As the example of `Option` indicates, the type is generic *before* compilation,
but after compilation it becomes a specific instance of that type. So you start
with a generic `Option`, but you end up with an optional *string*, or an
optional `i32`, etc. This is a process called "mono-morphization": the Rust
compiler takes the "polymorphic" item---one which works over many types---and
creates a concrete version at compile time, with a name specific to that
version---one which works over only that one type. That's what gets compiled
into your final executable.

The second thing I left out is that while being generic is great, it's fairly
obvious that we won't always want to be generic over *everything*. A function
for multiplication probably doesn't make any sense to be generic over strings,
or over random data structures we build. Actually, off the top of my head, I
haven't been able to come up with a meaningful function which is actually
generic over *all* types. (If you have an example, let me know!) Even a function
which just prints a type's value isn't, strictly speaking, generic over all
types. So how do we deal with this?


Traits (5m 45s -> 16m 45s)
------
In any case, Rust gives us a way to address our need for boundaries on just how
generic a function or type actually is: we call them `trait`s. And as it turns
out, `traits` are also the Rustic way to share behavior between different types.

If you've used a language like Java or C♯, you'll be familiar with the concept
of *interfaces*. If you've used Python or Ruby, you're probably familiar with
the idea of *mixins*. Interfaces and mixins respectively are the closest
analogies in those languages to Rust's traits, but there are some very important
differences we'll come to in a moment.

Traits in Rust are a way of specifying that a given type has a given
behavior---but, unlike polymorphism in a classical object-oriented type system,
they're completely orthogonal to types. That is, you define traits separately
from the types which implement them.

Let's take an extremely common example: creating a string representation of a
given data type. One way you might tackle this is by having all objects inherit
from a base type, which includes a method defining how to print it. In Python,
for example, all types derive from `object`, which has a private method called
`__repr__`. When you call the `print` function on some object, Python calls the
`__repr__` method on the object instance, and it hands back a string. There are
lots of benefits to doing things that way, but it also limits you. If you want
to include behavior from multiple different sources, you end up with multiple
inheritance---usually via a mixin system.

The other option in classical inheritance, and the one more common in statically
typed languages like Java or C♯, is to have interfaces. In that approach, you
define what methods must exist for a given object (something like a `toString`
method) to conform to a given interface (which might be named `IPrintable`).
Classes then specify that they implement that interface (usually via something
like `implements IPrintable` on the class definition) and supply conforming
method implementations.

The big upside to mixins over interfaces is that you get the actual
implementation, because you're actually inheriting the existing behavior---and
of course, you can override it in a subclass as you need to. With interfaces,
you have to reimplement the behavior on every class which uses the interface,
which takes us back to our repeated-code problem. The advantage to interfaces,
and the reason Java and C♯ prefer them over mixins, is that you don't run into
the many, *manY* problems which come with multiple inheritance.

Rust sidesteps this tension by stealing a page from Haskell's "type classes". If
you remember from our early discussion of custom types in episodes 2 and 3, Rust
doesn't define the implementation details where it defines the data
representation. This might seem a little strange at first, but there's a good
reason for it. By separating the behavior of the type from its data, we are able
to extend that behavior arbitrarily. The `impl` blocks we supply for Rust types
allow us to implement the interfaces specified by `trait`s elsewhere, as well as
our own behavior. If we want an object to be printable, we just `impl` the
`Display` trait for the type. (You can see an example of this in the source for
this week's show notes.)

This approach lets us define the behavior of types by *composing* different
elements together, rather than *inheriting* down a linear (or diamond-shaped)
chain. And while inheritance is powerful, and there are good reasons to include
it in your programming language---the Rust team is actively exploring how
inheritance might fit in the language---composition is *more* powerful.

1.  It gives us the benefits of mixins: we can define default implementations,
    which types can override if necessary.
2.  It gives us the benefits of interfaces: we can define the behavior which
    conforming types must implement, while not being bound by inheritance.

Add in strong type guarantees, and this is fantastic. It gives us the ability to
specify common patterns of behavior, and the same language machinery lets us
constrain generics so we can say exactly what a given function or type is
generic with respect to. For example, if we want to say that a function is
generic over any type that implements Rust's `Display` trait---that is, any
type which can be nicely printed---we can just specify that in the function
signature: add a colon after the name of the generic and the trait name, like
"T: Display", and you're off to the races.


Closing (1m 30s -> 18m 15s)
-------
That should give you a taste of the power of generics and traits. Next time,
we'll move from this high-level discussion of traits to a discussion of what it
looks like to use them in practice in Rust. We'll look at the different kinds of
traits Rust offers, how we might combine them, and what some of the current
limitations are.

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
