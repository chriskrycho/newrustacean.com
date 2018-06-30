# e024: Traits Deep Dive, Part II

## Intro

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is episode 24: Traits Deep Dive, Part II. In the first part of this deep dive, we talked about the details of defining your own traits and what you can and cannot do in terms of implementing and using traits defined elsewhere. In this episode, we’re going to *start* talking about how you can use traits in place of concrete types. And in the *next* episode, we’ll expand on that to talk about the `impl trait` feature that just landed on stable Rust with 1.26!

And yes, this was originally going to be a two-part deep dive, but as I got deep into this episode I realized that there was no way I could cover everything I needed to cover in the way I needed to cover it in a reasonable amount of time. Which makes sense: after all, traits are Rust’s primary mechanism for abstraction, and they’re *extremely* capable. So… three episodes it is!

## Syntax as sugar

The first thing I want to cover today is how a number of bits of Rust syntax are actually *sugar* for traits. This is something I've mentioned before, but it’s worth digging into a little further. The most important of these are traits related to operators in Rust, like `Add` for addition, `Index` for indexing, `Iterator` for loops, and so on. It looks likely that something similar will be in play with async and await with `Future`s in the future.

The way this works is that there are a set of traits the Rust compiler has special knowledge of. It's not that the traits get special privileges in terms of their capabilities with respect to the language. It's rather that some *syntax* in the language gets turned back into the same old normal traits that everything else is. Made more concrete: these traits are just traits like you could write; what's special-cased is the syntax. Rust *doesn't* let us invent new operators. It *does* express its existing operators largely in terms of the same kinds of things we *can* write: traits!

So, to grab one of those earlier examples: when you write `a + b`, that's equivalent to calling `a.add(b)` or `b.add(a)` or even `Add::add(a, b)`. The *operator* is special syntactical sugar for the *trait*.

This is certainly not unique to Rust, of course, and Rust is actually quite restrained compared to many other languages in terms of how much of this kind of thing you can do. C++ and Swift, for example, let you override *every* mathematical operator; Haskell lets you freely invent your own. There are upsides to those things, but the downside is that when you see a symbol it may mean something quite different than you expect, or you may have no idea what it means at all.

Rust has chosen to go a different direction. First, there is a core set of operator traits, which can be implemented for new types, but you cannot create your own new operators. Second, because they’re *traits*, operators are subject to the “orphan rule” we talked about last time. As such, you can't reimplement traits for core types even if you're feeling clever. There was an [interesting discussion on the internals forum](https://internals.rust-lang.org/t/pre-pre-rfc-for-paths/2420/) a few years ago about whether to add support for using the `/` character as a path-join operator (as you can in Python). While a few people liked it, it definitely didn't fit the way Rust *normally* approaches these things, and so it was dropped. It's not so much a right-or-wrong kind of thing, as what fits with the rest of the language and what people expect.

The upshot to all of this, in any case, is that you can `impl Add` for your own types and have the `+` operator just work. So, for example: let’s say that we have the example of a point in 3D space, with *x*, *y*, and *z* coordinates.

```rust,ignore
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
	fn add(self, other: Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}
```

Now we can add any two point instances together with the `+` operator.

## Using traits

Now, let’s dig in further on the *use* of traits.

The first things we’ll talk about have to do with *generics* and traits. Generics and traits relate in two ways: traits can be generic themselves, and traits can be used to constraint generic types.

### Generic traits

Traits themselves can have generic types. They *don’t* usually, because instead you most often end up using *associated types*. As I’ve mentioned, I have an episode in the pipeline (probably a month or two out yet) to talk all about associated items. It’s worth noting, however, that you *can* write a generic trait. To return to our absurd example of the `Eatable` type from the previous traits deep dive episode, we could do this:

```rust,ignore
trait Eatable<T> {
	fn eat(&self) -> T;
}

impl Eatable<String> for i32 {
	fn eat(&self) -> String {
		self.to_string()
	}
}
```

Then anywhere we had an `i32`, we could call `.eat` on it and get back a string. We could also do `impl Eatable` with some *other* type in the generic position and get something totally different out. But this suggests why we generally prefer to use associated types: because we need to manually write down one of these generic implementations for every distinct value of the type parameter we have. More on that in a future episode *all* about associated items, including associated types!

### Traits in generic constraints and `where` clauses

One of the primary places you’ll see existing traits is as *constraints* for generic types. This lets us write functions which are generic not over *everything* but over *anything which implements a given trait or set of traits*. For example: you might have a special function for displaying a given set of data—perhaps you want to render some HTML with the values in a struct, for example. In that case, you don’t really care what the concrete type is, you just care that it has the `Display` trait implemented. So you’d write your function as *generic* over any type which implements `Display`. There are two ways to write that. The first is with the trait constraint inline, right by the generic type parameter it constrains.

```rust,ignore
fn render_html<T: Display>(displayable: T) -> HTML {
    // ...
}
```

What we’re actually saying here in type terms is that our type `T` must be a *subtype* of `Display` – but other than that, it can be anything. Note that “subtype” here isn’t identical to the idea of a “subclass” in OO languages, even though “subtype” and “subclass” *are* often the same in those languages. In Rust (as in programming language theory more generally), a subtype is merely any type which is *substitutable* for another type. In traditional OO languages, one way you get at this is via the user of interfaces—no surprise then that we see it with traits in Rust.

Now, back to our trait constraint example where we are marking some generic `T` as a subtype of `Display`. Besides writing it inline, we can write this with a *where clause*. Where clauses start with the keyword `where` and go immediately after the return value for a function. If we rewrote our `get_html` function with a where clause, it would read like this:

```rust,ignore
fn render_html<T>(displayable: T) -> HTML
where T: Display
{
    // ...
}
```

This has the same *meaning* as inline constraints, but it becomes really important where the trait constraints on generic parameters gets to be long – *especially* the cases where you have multiple generic type parameters and each one of them has one or more trait constraints.

When there *is* more than one trait constraining a generic, you separate them with a `+`. So here, for example, if our generic type `T` needed to be totally ordered as well as displayable – perhaps so we could sort them – then we would write the constraint as `T: Display + Ord`.

This kind of constraint can be used for items in both argument and return position. A prime example of the way you can use it in return position is the `std::iterator::Iterator::collect` method. This one has some surprising properties when you’re not used to it (and, honestly, some surprising properties even when you *are* used to it)—so much so that I once saw someone [describe it](https://users.rust-lang.org/t/what-topics-would-you-like-to-see-covered-in-a-video-course-about-rust/10500/14) as *feeling* like overload-by-return-type.

#### Explanation: monomorphization

What’s actually happening is indeed something *kind of* like overloading in other languages. I’ve mentioned this before, but this is a good time to trace it out in a bit more detail. With Rust’s generics, the compiler does a process called *monomorphization*. Monomorphization is the process of taking something which is *polymorphic*—i.e. something which represents many forms—and turning it into something which has *just one form*. So, for a concrete, if rather trivial, example:

```rust,ignore
use std::fmt::Display;

fn to_string<T: Display>(t: T) -> String {
  format!("{}", t)
}
```

(We wouldn’t ever really write this function, of course; we’d use the `std::string::ToString` trait for this instead. But it’ll do.)

We have here a function which is generic over the type `T`, where `T` is constrained to “any type which implements `Display`.” It’s *polymorphic*: you can pass many different types—many different shapes!—into it. The only that matters is that those shapes have an implementation of `Display`. But for performance reasons, we don’t want to leave it polymorphic. We don’t want to have to do anything special at runtime for generic functions like looking them up in a table of versions to call, or putting things behind pointers and adding both indirection and allocations as a result. We just want functions we can call normally, as if we had written a concrete type here instead of a generic type!

So when the Rust compiler gets to this in your program, what it actually does is go through and figure out all the different types which get used *as* this generic `T`, and then create a specific version of the function for each of them. For example, say you had called `to_string(42i32)` and `to_string(some_ipv6_address)` and `to_string(some_custom_type)` (where `some_custom_type` is one of your own types where you’ve implemented `Display`). Rust would then create three versions of the `to_string` function; under the hood they each get their own names, which including the crate and module name and a hash to disambiguate them.

The “generic” function is gone entirely; it’s just those three individual normal functions. This is a prime example of Rust’s aim to have “zero-cost abstractions.” You don’t pay any more runtime cost for this than you would have by writing those individual implementation functions by hand (which is how you’d have to do it to avoid the aforementioned pointer and dynamic lookup costs otherwise).

One other note on the compiler: with a function this simple, it’s likely that you actually wouldn’t end up with functions here at all: Rust would probably just inline it all. But the idea is what we’re interested in here.

#### Monomorphization on return types

That first, kind of silly example was dealing with monomorphization of *argument* types, but Rust does exactly the same thing with *return* types. So we can return to our motivation example of `std::iterator::Iterator::collect`. The signature for `collect` is:

```rust,ignore
fn collect<B: FromIterator<Self::Item>>(self) -> B where Self:Sized;
```

Let’s talk through this: `collect` is generic over a type `B`, which has the *constraint* that `B` must implement the trait `FromIterator`. `FromIterator` is a *generic trait*, as we discussed above. The generic parameter for `FromIterator` defines the type returned by its `from_iter` trait method, and the definition of `collect` says that the type returned there will be the `Item` associated time from the `Iterator` implementor on which `collect` is called. (It’s actually slightly more complicated than that; there’s an associated item and *another* trait involved, but I don’t want to get too far from `collect` for right now; you can look at the <abbr>API</abbr> docs for all the details.)

So the function is *returning* a generic type `B` which implements `FromIterator` for the kind of `Item` contained in the iterator we’re dealing with. And the result is that `collect` can take your iterator and wrap it back up into almost anything as long as it has the pieces it needs. You can’t go from a simple list of values to a `HashMap`, for example, because you don’t have anything for the key type. But you also get the compiler telling you that, because `HashMap` doesn't implement `FromIterator` for an iterator over a simple list of values! On the other hand, you *can* go the other way: you can collect from an iterator over a hash map into a `Vec` instance, because it can just collect the values.

### Who’s in control?

One important thing to notice here – and remember this, it’ll be important next time when we talk about `impl Trait` – is who’s in control of the type when we’re dealing with generic arguments or generic return types.

When we have a generic *argument* type, we’re basically saying “for *any* type T which satisfies this trait constraint, I will do the right thing.” That means that the *caller* in control of the type: when I call some generic function with a concrete type, I as the caller have control over the type that goes in.

When we have a generic *return* type, we’re basically saying “I will give you *some* type T which satisfies this trait constraint.” That means the *function itself* is in control of the type you get back.

This starts taking us into some interesting type theory ground – specifically, into *universal* and *existential* types. We’ll talk a bit more about that next time in the `impl Trait` discussion. For now, just try to internalize the notion that for generics with trait bounds in argument position, the function will take *any* matching trait; for generics with trait bounds in return position, the function will return *some* matching trait.

## Closing

So that’s a pretty thorough look at how we can use traits in conjunction with generics. In the next episode, we’ll finish up this deep dive on traits, taking a look at how we can use traits directly as arguments and return values, including the new `impl Trait` syntax from Rust 1.26. We’ll also dig into the extremely important concept of *object safety*.

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- Hans Fjällemark
- Dan Abrams
- Martin Heuschober
- Chip
- Nick Stevens
- Nathan Sculli
- John Rudnick
- Zachary Snyder
- Daniel Collin
- Matt Rudder
- Oluseyi Sonaiya
- Peter Tillemans
- Anthony Deschamps
- Alexander Payne
- Vesa Khailavirta
- Chris Palmer
- Ramon Buckland
- Damien Stanton
- Daniel Mason
- Derek Buckley
- David W. Allen
- Behnam Esfahbod
- Aaron Turon
- Ryan Osial
- Paul Naranja
- Olaf Leidinger
- Marshall Clyburn
- Raph Levien

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for _this_ episode at <newrustacean.com/show_notes/e024/>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!