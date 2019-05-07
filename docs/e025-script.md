# e025: Traits Deep Dive, Part III

## Sponsor: [Parity Technologies](https://paritytech.io)

Hello, everyone! Over the last few years, I’ve had a number of opportunities for company sponsorships of the podcast; none of them were a good fit. Today, I have one that *is* – and it’s a *great* fit.

[Parity Technologies](https://paritytech.io) is advancing the state of the art in decentralized technology. Their flagship software is the Parity Ethereum client, but they're also building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Their next big project is Polkadot, a platform leveraging blockchain tech for scaling and interop in decentralized systems. And Parity uses Rust for its trifecta of safety, speed, and correctness!

And they’re hiring Rust developers! So if you’d like to work on any of these projects, check out their jobs at [paritytech.io/jobs](https://paritytech.io/jobs).

Thanks to Parity for sponsoring the show!

## Intro

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is episode 24: Traits Deep Dive, Part III. In the first part of this deep dive series, we talked about how to write and implement traits. In the second part, we talked about using traits as bounds on generic types. In this third and hopefully final part, we’re going to look at using traits directly in argument and return position, including the new `impl Trait` syntax and the important concept of object safety. If that sounds like a lot to cover to you, it sounds like it to me too, so let’s jump in!

First, though, a quick correction from the *last* traits deep dive episode: thoughtful commenters on both Reddit and Hacker News correctly pointed out that I was mistaken in my discussion of universal and existential types! I said that in the case of *generics with trait bounds*, arguments are always “universal” and return values are always “existential” – but this isn’t true with generics – the `Iterator::collect` method example I discussed on the show is a prime case where the caller is in control of and specifies a specific return type, after all! Thanks for that correction!

## Plain traits as arguments and return values

There are several ways you’ve been able to use traits *without* generics since Rust 1.0. However, they have historically had some gotchas, many of which just got resolved with Rust 1.26 and 1.27, and one of which *will* be resolved in the future. So let’s dig in!

### Motivation

I’ve spent quite a while thinking about how to explore this particular idea, and had gotten stuck repeatedly. Happily, a couple weeks ago while I was on vacation, a reader of my [Exploring 4 Languages](https://www.chriskrycho.com/exploring-four-languages.html) series had a question from the most recent post there, on [Starting to Model the Domain](https://www.chriskrycho.com/2018/exploring-4-languages-starting-to-model-the-domain.html). In that post, I noted that Rust lets us write out function types as type aliases if we want—though I noted that writing out type aliases for functions isn’t *usually* something we’d see.

The reader who emailed me asked:

> Could you give a small code example of how to use a function type alias like this Rust? It is very hard to find an example of this usage (probably because it is unconventional in Rust).

And as it turns out, this is a perfect setup for the rest of this episode!

One time when it’s convenient to write out an alias for a *function* – not just for “narrowing” a generic or something like that – is when you have some complex or nested transformation you want to apply to a data structure in the context of a [`std::iter::Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) invocation. You might want it to be testable, and you might want it to be passed into some other function for the sake of  abstraction (maybe there are multiple valid kinds of transformations from input type to output type that you’re interested in).

For example, let’s say we want to write a function which lets us get the distances of a set of points from some other point. Let’s assume some basic machinery:

```rust,ignore
use std::{
    f32,
    ops::Sub
};

#[derive(Default)]
struct Point {
    x: f32,
    y: f32,
}

impl<'p> Sub for &'p Point {
	type Output = Point;

	fn sub(self, other: &Point) -> Point {
		Point {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}
```

Now, if we wanted to get the distances between one of those points and another, we could do that inline, using the Pythagorean theorem:

```rust,ignore
fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: -4.5, y: 12.2 };
    let change = a - b;
    let distance = (change.x.powi(2) + change.y.powi(2)).sqrt();
}
```

So far so good, but what if we had a *bunch* of these points – say, a `Vec` of them – and wanted to get their distance from some other point? Well, then it makes sense to iterate over them and use the `std::iter::Iterator::map` method. To do *that* we need a closure:

```rust,ignore
fn main() {
	let points = vec![
		Point { x: 1.0, y: 2.0 },
		Point { x: 12.0, y: 4.3 },
		Point { x: -5.4, y: 18.7 },
	];

    let origin = Point::default();
    let distances: Vec<f32> = points
        .iter()
        .map(|&point| {
            let change = point - origin;
            let distance = (change.x.powi(2) + change.y.powi(2)).sqrt();
        })
        .collect();
}
```

Now, what if we find this “get the distance between two points” function useful and wanted to extract it? Well, we could do that by extracting a function which takes two points by reference and returns the distance between them:

```rust,ignore
fn distance_between(a: &Point, b: &Point) -> f32 {
    let change = a - b;
    (change.x.powi(2) + change.y.powi(2)).sqrt();
}
```

And this is *fine* – when we want to use it in the context we laid about above, we’d just have `|point| distance_between(&point, &origin)` in our `map` invocation. However, we could actually go one step further here and let ourselves create a function which we can “preload” with the desired comparison point: a function `distance_from` which takes a `Point` as its argument and returns a *function* which takes *another* `Point` as its argument and then returns the distance between the two. Then you could just write the invocation like this:

```rust,ignore
	let origin = Point::default();
	let distances: Vec<f32> = points
		.iter()
		.map(distance_from(&origin))
		.collect();
```
 
That’s much clearer to read (at least to me). Unfortunately, though, writing out the signature for that function has been a pretty complicated thing, historically. For one thing, you had to wrap that up behind a heap allocated pointer, like: `Box<Fn(Point) -> f32>`, and it’s actually *really* hard to write this type and get the lifetimes all lining up correctly if you want to use references instead of moving all the `Point` instances:

```rust,ignore
fn distance_from<'a, 'b: 'a>(offset: &'b Point) -> Box<Fn(&'a Point) -> f32 + 'a> {
	Box::new(move |point| {
		let change = point - offset;
		(change.x.powi(2) + change.y.powi(2)).sqrt()
	})
}
```

Now, to get back to my reader’s question, it would be really nice, since `Fn` is a trait, if we could write a type alias for this! Something like:

```rust,ignore
type DistanceFrom<'a> = Fn(&'a Point) -> f32;
```

Then we could use that in our definition:

```rust,ignore
fn distance_from<'a, 'b: 'a>(offset: &'b Point) -> DistanceFrom<'a> {
    // ...
}
```

Unfortunately, that specific definition isn’t quite valid – remember, we had a box around it before. So the correct definition using a `Box` would be to wrap that in a box at the call site:

```rust,ignore
fn distance_from<'a, 'b: 'a>(offset: &'b Point) -> Box<DistanceFrom<'a>> {
    // ...
}
```

Now that’s an improvement in a number of ways. But with Rust 1.26, we got `impl Trait`, so can’t we use *that* here? It would be really nice to write something like this:

```rust,ignore
type DistanceFrom<'a> = Fn(&'a Point) -> f32;
```

and then use `impl Trait` with it:

```rust,ignore
fn distance_from<'a, 'b: a>(offset: &'b Point) -> impl DistanceFrom<'a> {
    // ...
}
```

Unfortunately, we can’t do this… yet. Type aliases can’t be used for traits types like this. However, I’m definitely not the first one to think it’d be useful to be able to write trait aliases! [<abbr title="Request for Comments">RFC</abbr> #1733: Trait aliases](https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md) will give us just that. The syntax would be:

```rust,ignore
trait DistanceFrom<'a> = Fn(&'a Point) -> f32 + 'a;
```

Someday! In the meantime, though, we can still use `impl Trait` – we just can’t use it with the nice convenient name.

### Traits in return position

So… on to a topic we’ve been waiting for for quite a few episodes now. Why *would* we want to use `impl Trait` instead of a box here?

Most obviously, it simplifies our (non-aliased) function signature and implementation a bit. Now it reads like this:

```rust,ignore
fn distance_from<'a, 'b: 'a>(offset: &'b Point) -> impl Fn(&'a Point) -> f32 {
	move |point| {
		let change = point - offset;
		(change.x.powi(2) + change.y.powi(2)).sqrt()
	}
}
```

The major change here is that the `Box` is gone – both in the signature and in the body of the function. Historically, we always had to return any *trait object* – that is, a piece of data where *all we care about* is that it implements a specific trait – behind a pointer. Why? Well, because that trait object could be pointing to all sorts of different sizes of things. A two-variant enum and a 40-field struct can implement the same trait – but the Rust compiler requires that every output from a function be the same size.

The only candidate – at least historically – has been a heap-allocated pointer! A `Box` is always the same size. (A regular reference is, too, but then we’d need a lifetime to tie it to, and... there isn’t one available.) Whatever item we could define in the body of this function actually has to be *moved* out of it, or it’ll get dropped at the end of the function. So our only option here historically was a heap-allocated pointer.

This had a couple important consequences:

1. We had to pay the price of that heap allocation.

2. More importantly, it also requires dynamic dispatch—looking up the specific function to execute at runtime instead of at compile time. One of the main reasons for the monomorphization we talked about earlier is to get *rid* of this runtime overhead.

Those costs are small, and they’re often trivial—the kinds of things you don’t need to worry about. But they are *real* costs, and in the contexts where Rust is *most* useful, it does matter. So those were important limitations.

### Traits in bare argument position

Many of those limitations also applied to the other place you might have been tempted to use a trait by itself, as the type of an argument. For example, you might think you could write

```rust,ignore
fn foo(thing: SomeTrait) {}
```

But the same basic issue is at play as in the return type context. Rust needs an item with a constant size to be able to do this correctly. So we take a reference to the trait type; using a pointer means we have a type with a known size again:

```rust,ignore
fn foo(thing: &SomeTrait) {}
```

We can also accept a heap-allocated pointer here, but in general we don’t actually *have* to: we can just write `&SomeTrait` and both regular references and also any type which implements the `Borrow` trait will work. (For a refresher on `Borrow` you can go back and listen to e018!)

In any case, we have to put the trait type behind a reference, and as such we have dynamic dispatch with its small runtime cost – though in this case we do *not* (necessarily) have a heap-allocated pointer backing the trait object.

This is extremely handy any time we don’t have a single specific type we’re going to be returning from or dealing with in a given function. For example: anywhere that we might be returning different specific iterator subtypes (`Map` and `Filter`, for example), or where we’re operating over a heterogeneous collection, where the only thing we care about in the collection is that every item in it implements some trait.

### `impl Trait` and `dyn Trait`

We know enough now to see that at least in principle, the Rust compiler should often be able to figure out exactly what type we’re talking about when we want to return a trait type or take a trait type as an argument. After all, it already does exactly that with generics! And generic types with trait bounds notably do *not* require a heap allocation or dynamic dispatch. The compiler statically figures out the specific types they’re invoked with and return, and monomorphizes them—makes a version for each. So why can’t it do that for trait types too?

As of Rust 1.26, it can. The new `impl Trait` feature lets us monomorphize the type here. From our perspective writing the code, the type of a closure – that we can’t name concretely, because it’s anonymous and Rust makes up a name at compile time – is just “something which implements the `Fn` trait with these argument and return types. But Rust compiles it down to the actual single function with that anonymous type. And now, it has a specific return type *size*, so we can get rid of the pointer. No more heap allocation, and no more runtime lookup cost. That goes for both argument position and return position.

To return to my correction at the beginning of the episode: *this* is where universal and existential types come into play! (And I got mixed up with them because of how I split this episode into two parts.) `impl Trait` in argument position is a *universal* type, an “any” type specified by the caller; and `impl Trait` in return position is an *existential* type, a “some specific” type specific by the callee.

The limitation of `impl Trait` is all those times we still need dynamic allocation and a heap allocated pointer. For those, we have another slight change landing for the Rust 2018 edition, which just reached stable Rust in 1.27: the `dyn Trait` syntax. Where before, we were allowed to simply write `Box<Trait>` or `Rc<Trait>` or even `&Trait`, now those invocations should be `Box<dyn Trait>` or `Rc<dyn Trait>` or `&dyn Trait`. The reason is to make *explicit* that there’s a trait object in play, not just a regular `struct` or `enum` type. That’s really important for making sure you have the right mental model in place – and, as I noted in my discussion of the feature in the recent Rust 1.27 news episode, it also makes for a nice symmetry between trait objects with `dyn Trait` and existential and universal types with `impl Trait`.

### Trait objects and object safety

I’ve used the phrase “trait object” often throughout the episode – and for good reason; this idea is important. Any time we’re talking about dealing with some item *as a trait behind a pointer* rather than as a generic or concrete type, we’re talking about this idea of “trait objects.” Trait objects let us deal with heterogeneous types dynamically at runtime, but *safely*, much as interfaces do in tradition object oriented languages.

But there is a very important set of rules which govern “trait objects” and allow us to use them safely in Rust, called “object safety.” This set of rules comes up any time you’re trying to use traits for this kind of abstraction in your programming; and since traits are Rust’s primary tool for abstraction, well… it’s going to come up pretty regularly.

A few minutes ago, I noted that the compiler requires a constant size for return values from functions. The compiler captures this with `Sized`: a marker trait which tells the compiler that the item in question has a constant size known at compile time. (For a review on marker traits, you can go back and listen to episode 22, on the `Send` and `Sync` marker traits).

This marker trait has two rules attached to it for object safety.

1. The trait itself *cannot* require that the special `Self` type be `Sized`. Instead, *trait methods* can set that requirement when needed—but only when needed!—with a `where` clause.
2. All of a trait’s methods must themselves be object safe. There are also two rules for object safety  for trait methods:
	1. They cannot have any type parameters, that is, they cannot be generic.
	2. They must not use the `Self` type themselves.

These rules come down to the reality that when you’re dealing with a trait object, Rust essentially “throws away” the concrete type or types you’re dealing with – as it has to, since it needs to treat every different concrete implementation behind the trait object the same way. But if you reference `Self`, Rust needs to be able to get back to the concrete type. These things are clearly at odds! So mostly, if you need a trait to be object safe, just avoid referencing `Self`.

For further reading on object safety, I’ve linked a few important things in the show notes:

- [RFC #255: Object Safety](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md), which is the formal definition of object safety in the language
- [Ch. 17 in the Second Edition of *The Rust Programming Language* ](https://doc.rust-lang.org/book/second-edition/ch17-02-trait-objects.html)
- [A detailed explanation](https://huonw.github.io/blog/2015/05/where-self-meets-sized-revisiting-object-safety/ "Where Self Meets Sized: Revisiting Object Safety") of these ideas by Huon Wilson from around the Rust 1.0 release

## Closing

That’s a wrap on our our deep dive on traits! The next main teaching episode will be a look at `unsafe` and the escape hatches it does and doesn’t allow. Also coming up are a look at functional programming ideas in Rust and a pair of Crates You Should Know episodes focused on Futures and Tokio!

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:


- Behnam Esfahbod
- Anthony Deschamps
- Chris Palmer
- Ramon Buckland
- Alexander Payne
- Daniel Collin
- Paul Naranja
- John Rudnick
- Marshall Clyburn
- Martin Heuschober
- Oluseyi Sonaiya
- Hans Fjällemark
- Vesa Khailavirta
- Ryan Osial
- Daniel Mason
- Chip
- Raph Levien
- Derek Buckley
- Damien Stanton
- Aaron Turon
- Nick Stevens
- Peter Tillemans
- Dan Abrams
- Nathan Sculli
- Zachary Snyder
- Matt Rudder
- Sascha Grunert
- David W. Allen

You can sponsor the show at <https://patreon.com/newrustacean>, or send a one-off at any of a number of other services listed at the website. Even more importantly, please let others know about the show – by telling them about it at a meetup, sharing it around in whatever media you use, or reviewing or recommending it in your podcast directory of choice.

You can find the notes for *this* episode at <newrustacean.com/show_notes/e025/>. The website also has scripts and code samples for most of the teaching episodes and transcripts for many of the interviews. 

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!