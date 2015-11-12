Allocate It Where?
==================

Intro (30s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is episode 5: Allocate
It Where?


News (2m)
----
Rust 1.4 release! Highlights:

  - New chapter in book on error handling (great complement to e003)
  - Full MSVC support for 64-bit builds
  - One language change, the details of which are (frankly) still a bit beyond
    me! I'll link it in the show notes. As usual, it's a "functionally
    semantically versioned" change: it *technicalLy* breaks backwards
    compatibility, but because the Rust core team can run the compiler on every
    single package on crates.io, the actual impact to the community isn't that.
    Given the limitations of a programming language, as opposed to a library or
    framework, that's excellent.
  - I contributed a little bit! Not much, and nothing important---just cleaning
    up a few things in the build tooling around documentation---but it's still
    fun to be able to say. As I've said before: jump in and contribute!


Overview and review (1m 30s)
-------------------
Last time, we were talking about functions in Rust, and in particular about
using functions as arguments to other functions. I had originally intended to
discuss *returning* functions as well, but needed to take some time to discuss
both type systems in general and Rust's type system in particular as preparation
for the discussion of Rust's functions. Today, we'll pick back up and jump in
with a discussion about returning functions, and that in turn will lead us into
an initial discussion of the stack, the heap, and why we need to care about the
difference between the two when dealing with functions. Note that I'm going to
assume everything from last week's episode as a baseline here. Now, let's dive
in and see how to return functions from other functions!


Returning functions (a) (2m)
-----------------------
  - Both normal functions and closures can be returned from a given function,
    and the definition of each is essentially the same, with the usual
    variations in syntax between the two of course.

  - If you've done this in other languages, including Swift, you'd expect that
    the return type would just be a function specification, just as when
    defining a function type as an argument. And you'd be almost right, but if
    you try that in Rust, you'll get a compiler error: it'll tell you that the
    types are mismatched, and part of the error will point to the lifetime of
    the object created: it'll say you need a `'static` lifetime (we'll come back
    to lifetimes fairly soon... right after I get a better handle on them).

  - If you start chasing the normal course of how to fix that, you'll quickly
    end up in a messy spot... and still won't be able to fix it. For particular
    details on that, see both Rust by Example and the Rust book.

  - As it turns out, the solution here is to use the `Box` type and construct a
    "boxed" version of the function.

  - So, what is `Box`? It's a way of allocating an item on the heap, rather than
    on the stack.


The stack and the heap (6m)
----------------------
Okay, but some of you might be coming in from languages like Ruby and Python,
where you rarely (if ever) think about the stack and the heap.

  - For starters, I'll link a really fantastic explanation on Stack Overflow in
    the show notes, and really if you're going to do any kind of low-level or
    systems programming, you're going to need to get familiar with this.

  - Here's a *really* brief overview, summarized to the best of my ability. I
    will freely admit that this is an area where *I* need to be stronger, so do
    let me know if I get a detail wrong along the way; I'll happily add any such
    corrigenda to the show notes.

  - Both the stack and the heap are allocated by the operating system, but they
    are very different in nature, purpose, and behavior.

  - The stack is a last-in-first-out data structure, and if you've ever seen a
    "stack trace" in your debugging, you've seen a bit of how it works: each
    function call essentially layers on top of the previous one. All memory
    needed for the data for a given function---all its local variables---gets
    allocated on the stack for that particular function.

  - This works because the stack size for a given function can be known at
    compile time, and therefore can be fixed for run-time calls; this is what
    makes it possible to use it as a stack for function calls. It is also the
    most significant limitation of the stack: what about a data structure whose
    size you *don't* know ahead of time? Or what if a given data structure needs
    to change in size over time? You can allocate the pointer to the data
    structure when you create the stack, but you can't allocate the *whole*
    stack size in those cases. This is why C doesn't let you create dynamically
    sized arrays, for example: arrays are stack-allocated.

  - What we need is a way to allocate space dynamically. That way is the heap.

  - As you can imagine, the heap is a substantially more complicated data
    structure. Not only does it entail not knowing ahead of time what size the
    allocations will have to be, it also involves data that isn't local to a
    given function, so you suddenly start having to deal with access control and
    allocation and deallocation timing issues.

  - One common example of a type in Rust which is always heap-allocated is its
    `Vector` type, which is a dynamically growable array-like type---and because
    it's dynamically growable, it *must* be heap-allocated.

  - What about other types? Well, normally all values in Rust are allocated on
    the stack, rather than on the heap (with the obvious exception of special,
    always-on-the-heap types like `Vector`s). That includes functions!

  - `Box::new` heap-allocates them instead. Unlike memory dynamically allocated
    with `malloc` in C, though, `Box`ed items get still cleaned up automatically
    in Rust when they go entirely out of scope; the compiler makes sure that any
    cleanup code necessary is executed (in the `drop` method on a given type)
    and then the reference becomes unavailable.


Returning functions, (b) (6m)
------------------------
Now that we have some idea what's going on with the stack and the heap, we can
come back to talking about returning functions from other functions. As I said
at the beginning of the episode, the solution to this particular dilemma is to
use `Box::new` when returning functions from functions.

We can do this because functions are still just another type in Rust, we can
handle this case just like any other case where we need to heap-allocate
something! This is one of the *many* places where having functions that are
first-class members of your language comes in handy.

It's worth noting that functions get allocated in memory somehow or another in
*all* languages; the question is how obvious this becomes to you as a
programmer. In this case, we have to be pretty explicit about it. But it does
raise some questions. After all, it's great that this solves our problem. But...
*why* does this solve the problem, and for that matter, why do we have to solve
this problem at all?

  - Functions are generic types. Right now, the Rust compiler only supports
    returning concrete types, and generics aren't concrete.

  - As an aside, this concrete type issue is actually true of generic types of
    arguments, too, but the Rust compiler gets around that by going through and
    creating actual concrete types for all forms of a given generic function
    which get called in a given program.

  - References *are* concrete, though, so we might be able to return a
    reference, right? Well, unfortunately, no: references need *lifetimes* so
    that the compiler knows how long to hold onto the memory being referenced.
    Again, lifetimes are a topic we will come back to in the future. For today,
    it is enough to know that for reference types, the Rust compiler needs to
    know how long to hold onto the reference, and lifetimes give us a way to do
    that explicitly when we need to.

  - Fight with this as we may---and there are a lot of ways you might try to
    work through this, as both the code samples in this episode's show notes and
    the Rust book's discussion of the topic demonstrate---you can't win.
    Returning a stack-allocated reference to a stack-allocated function is not
    something the compiler will let you do.

  - For the record, this is a good thing: it is part and parcel of the memory
    safety guarantees that Rust makes. It just makes certain cases a bit more
    work. Like this one.

  - In any case, there is a solution to our problem. As you've by now guessed,
    we don't have to stack-allocate the functions. We can use `Box::new` and
    heap-allocate them instead. Then we have a smart pointer reference to the
    function instead. Conveniently, this solves all of our problems at once.

      + They will be cleaned up automatically, so we don't need to worry about
        lifetimes anymore.
      + Boxed types are smart pointers---that is, a kind of reference---so we
        have a concrete type we can return.

So there you have it: you *can* return functions---whether plain old functions,
methods, or closures---from other functions. You just have to heap-allocate them
explicitly. And now we know *why*!

There's still one really important part of closures we haven't talked about, and
that's using the `move` keyword, but we'll come back to that in the future when
we talk more about ownership semantics!


Closing (1m)
-------
### Next time
Next time, we're going to take a bit of a step back up to a slightly easier,
slightly higher-level concern and talk about modules and packages---a.k.a. ]
crates---and API design. Because I've been playing with some APIs in Rust, and
some of them I like, and some of them... not so much. How we structure our code
matters, it turns out!

### Sponsors

Thanks to Chris Patti for sponsoring the show this month! If you have any
interest in Python, go take a listen to `Podcast.__init__`, a podcast he
co-hosts; they're interviewing tons of interesting people in the Python
community.

### Follow/support

You can find show notes with detailed code samples illustrating these ideas, as
well as links to things mentioned on the show, at NewRustacean.com. You can also
follow the show on Twitter or App.net @newrustacean, or follow me there
@chriskrycho. If you like the show, please rate and review it on iTunes to help
others find it, and if you *really* like the show, I'd welcome your financial
support. You can set up recurring donations at Patreon.com/newrustacean, or
one-off donations at Venmo, Dwolla, or cash.me. Last but not least, I'd love to
hear from you on social media, in the thread for the show on the Rust user forum
at users.rust-lang.org, or via email at hello@newrustacean.com.

Until next time, happy coding!
