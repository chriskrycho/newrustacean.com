# `Borrow`, `AsRef`, `Deref`: my head hurts now

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 18: `Borrow`, `AsRef`, `Deref`: my head hurts now.*

## The `Borrow` and `AsRef` traits

### Set the Stage

Back in episode 17 (a long time ago for those following along with the show in real time!), we talked about references in Rust, and looked at the referencing and dereferencing operations. And we've also talked about a variety of data structures and heap-allocated objects like the `Box` and `Vec` and `String` types. In this episode, we put a bunch of those pieces together. We're going to talk about the `Borrow`, `AsRef`, and `Deref` traits. If you listened to my interview with Raph Levien, you'll recall that we mentioned (and neither of us could remember initially which was which!) the first two of those traits. All three are really important for designing easy to-use APIs, *especially* when you're working with those heap-allocated pointers.

Borrowing and taking things by reference seem like pretty fundamental building blocks in Rust, right? And indeed they are. The ampersand borrow/reference operator isn't overloadable. But how it works is something you can define and customize for each type you create: using the `Borrow` and `AsRef` traits. Now, if you're like me, you have two questions:

1. Why would I *need* to customize this?
2. Why are there three different traits for borrowing and references?

That latter question is precisely why---in a section which would have been ridiculous to put on air---Raph and I had to stop in the middle of the interview and go look up the difference between `Borrow` and `AsRef`. For that matter, it's the reason there's a note about it in the standard library docs and a dedicated section in the Rust book about it! But we'll come back to that in a minute. And that's not even mentioning `Deref`!

In order to get to those questions, we need to have a handle on what each one of these types actually *does*.

### `Borrow`

Say you have a basic struct type, and that it's allocated on the stack as usual. You get your normal borrowing and move semantics when you're dealing with the type, as we've discussed before. So if you *borrow a reference* to it with the `&` operator, you have a *pointer* to the type, and depending on whether the original instance was mutable and whether you passed a mutable reference, you may be able to change the object where you borrowed that reference. All well and good.

But---and here, let's thing about this with the `String` and `str` types in mind---what if you wanted to build something like a `HashMap`? Remember that `String` is a `Vec` of `u8` data validated to be valid UTF-8, and `str` is a slice of `u8` data validated to be valid UTF-8. Presumably you would want those two things to hash to the same thing---they're both just UTF-8 strings!---and so far as possible you wouldn't want the consumers of your `HashMap`'s API to have to think about what they were handing in.

So the `Borrow` trait lets us abstract over this. We can `impl Borrow` for a type by implementing a `borrow` method on the type. Then anywhere we need something which can be borrowed as a UTF-8 slice, we can explicitly call the `borrow` method, and any type which implements it can be used. This is just mashing together this specific trait with generics, but it becomes really helpful when you're implementing something like a `HashMap`. And indeed, the `HashMap` in Rust's standard library does *exactly* this: its `search`, `get`, `contains_key`, and `remove` methods (and the mutable variants where applicable) all explicitly require one of their arguments to have a type the main key of the `HashMap` can be borrowed as. So if the `HashMap` has a `String` key, and you pass it a `str` argument, that's fine: it can call `borrow()` on the `String` instance and get a `str`.

We could do the same thing with our own custom types, too. Say you had a function which you needed to be able to borrow as a slice of unsigned-eight-bit integers. You would write the function as `fn takes_a_borrowable<B: Borrow<[u8]>>(slice: B) { ... }`. In the body of the function, wherever you actually needed at the borrowed contents, you would just type `slice.borrow()` and do what you needed with it. Then you could pass in an actual slice of `u8`, or you could pass in a `Vec<u8>`, or a `Box`, `Rc`, or `Arc` around a `u8` slice, or your own custom type.

As for your own custom type, say you had a chunk of data which made sense to represent as a slice of `u8` values, and some associated metadata you were going to carry along with it. (I have no idea what this would be that isn't a unicode string off the top of my head; just roll with me here.) When comparing this to other things, though, the metadata is irrelevant and the `u8` slice is all that matters. You would just implement `Borrow` for the `struct`, where the `borrow()` method returned a reference to the slice contents.

So that's how `Borrow` works, and the example I gave at the outset is the main time you'd want to use it. `HashMap`s and things like that---types where there is some kind of underlying equivalence you need to be able to get at---are prime places to use `Borrow`. And note that the *equivalence* relationship is such that you don't care whether you own the data or not: you're just *comparing* it. As such, you're always fine *borrowing* it.

If we wanted it to dereference into a different type, we'd need to implement the `Deref` *trait* instead. More on that one in a moment.

### `AsRef`

First, though, let's talk about the other trait discussed in my interview with Raph Levien: `AsRef`. More explicitly, we're talking about `std::conversions::AsRef`: and the module namespace tells us something about the semanatics of this trait. It's a way of converting one kind of reference to another kind of reference. Remember that: it's important and we'll come back to it in just a minute. First, though, let's talk through the signature.

Just as `Borrow` only has the `borrow()` method, `AsRef` just has `as_ref()`. And the signatures are nearly identical. Both of them are generic over a type with a compile-time-known-size, and the signatures of both functions are the same: they borrow `self` and return a reference to the type over which they're generic.

Above, we talked about using `Borrow` for using related keys for `HashMap`. What about `AsRef`? We use it for things like converting between operating system string types, or from a `str` to a `u8` slice. Think of it this way: every pointer to a `str` can be a pointer to a `u8` slice, because all `str` instances are composed of `u8` data---they just have *more* guarantees than a plain `u8` slice does. Those guarantees mean you can't implement `as_ref` from `u8` to `str`, but you can readily implement `as_ref` from `String` to `u8`, too. Note that in each of these cases, we *do* care that we are getting a reference to something. The whole point here is to have something we *know* is a reference, and then to do something with that reference---unlike `Borrow`, where the point is that we want to treat both owned and borrowed data identically.

### `Borrow` and `AsRef`

As we noted from the outset: `Borrow` and `AsRef` are pretty similar. The difference in their signatures... is the name of the trait method. This close relationship shouldn't be a surprise if you've followed along with the show, or if you're familiar with Rust's core ideas more generally. *Borrowing* is Rust's way of reasoning about ownership when we make *references* to a given piece of data. So what separates `AsRef` from `Borrow`? Basically, it comes down to what you're actually using the conversion for. If it's for doing some kind of comparison for two types, you'll generally want `Borrow`. If it's for *converting* one type to another, use `AsRef`.

You can see where the name for `AsRef` comes from: you can think of it as being a trait for *converting* a reference to one type *as a reference*---`AsRef`---to another type. `Borrow` is a little hazier. Yes, we're *borrowing* a reference from one type to another, but, well... that's kind of what `AsRef` is too. I'll freely admit: `Borrow` isn't a particularly descriptive name in terms of distinguishing it from `AsRef`. But in the Rust standard library's defense, it's hard to come up with a better name here: `AsRefButWithInterchangableEquivalenceOrOrderingToo` doesn't exactly roll off the tongue. So `Borrow` it is. Just remember: use `AsRef` for cheap conversions between types which are identical under the covers. Use `Borrow` for getting at relationships of *equality* or *ordering*. Use `AsRef` when you *need a reference*. Use `Borrow` when you don't care what the ownership state is.

### `Deref`

Note we're *not* here changing the behavior of the dereferencing `*` operator with either `Borrow` or `AsRef`. You can see this if you implement either `Borrow` trait for a simple struct which wraps an integer, and try to pass a reference to that struct to a function which takes a reference to an integer. It won't work; the compiler will complain that you passed it a reference to the wrapper, not a reference to an integer.

If you're already fairly familiar with Rust, you either already know where I'm going, *or* you're wondering how something like passing a reference to a `String` can work in functions which take a reference to a `str` as an argument.

For this, we need the `Deref` trait. This is one of the only places in Rust where an implicit conversion between types can happen. And I say implicit because it's implicit from the point of view of the *caller*, but it's still explicit in the sense that you have to define *exactly* how a dereference operation is going to behave. It's still explicit *somewhere*. But if you have a good reason to---the `String` and `str` example is one such *very* good reason, in terms of the "ergonomics" of using the language. If you had to write `some_string.as_str()` every single time you needed to get a string reference from a `String`, it would get old in a hurry. The same is true for `Vec` and slice types. We have a nice out: `std::ops::Deref`, combined with the language feature called *`Deref` coercion*.

As its module name indicates, this is an *operator* overload, unlike `Borrow` and `AsRef`. When you use the `*` operator, it uses the `Deref` trait to define how to dereference the data in question. In many cases, that'll be incredibly simple. In others---say, smart pointer types---there may be a layer of indirection to remove. You might have a wrapper type which you're just using to provide extra data *around* some underlying value, but where you want to be able to get at the contents of without doing something like `*MyStruct.its_internal_contents`; it would be nice to just write `*MyStruct` instead.

The real power here comes when we combine the trait with the the `Deref` coercion rule. This rule says that where you have a dereference defined from some type T to some other type U, anywhere which takes a reference to T can automatically be coerced to act like references to T. So, going back to our little type which just wrapped around an array slice: if we implemented `Deref` for that slice type, any place which takes a reference to a slice can also take a reference to our type.

This isn't dangerous (though it can be surprising the first time you see it), because we've defined exactly what the coercion relationship is---and all in `safe` Rust code. If you define a way to dereference your type as another type, the compiler can safely use that way---the way *you defined*---where it needs to. As such, this is very different from pointer type coercions in C. There's no unpredictability about what you'll end up with, and you can't do *arbitrary* coercions. And of course, this is what makes it ergonomic to use `String` and `str` side by side: `String` implements `Deref` for `str`, so anything which expects an `&str` can also take `&String` because of this rule.

### The `Mut` variants

One thing I've glossed over so far in the discussion is that each of these traits has a `Mut` version: `BorrowMut`, `AsRefMut`, and `DerefMut`. These do exactly what you'd expect: let you implement the *mutable* equivalents to the above. Separating these from the immutable variants might seem like extra work, but actually it's extra control: it means you *can* let consumers have mutable access to your data, but you don't have to. This is particularly helpful where you're defining something with an immutable external API and a mutable internal API. I haven't looked around for an example of this pattern, but I imagine this would be a sensible and fairly Rustic way to implement so-called persistent data structures in Rust.

### An Example

For an interesting example of a case where someone is using `Borrow` and `AsRef` in practice, I'll link a blog post by Arnin Roacher from mid-2016, in which he describes building a command line tool for the Sentry crash reporting framework. In that blog post, he describes using a `RefCell` to deal with a C API that expected a single, long-lived pointer for managing a connection. As an aside, if you're looking for a relatively small but solid Rust code base to read through, the **sentry-cli** repository is a great place to start, so I'll link that in the show notes as well.

## Closing

I hope that gives you a pretty solid idea of how you can use, and when you should use, these three traits around borrowing and referencing things in Rust. These are a little more arcane than some of the pieces we've talked about before, but they're also incredibly useful---when you want them, you *really* want them.

### Sponsors

Thanks to 

- Chris Palmer
- Christopher Giffard
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Philipp Keller
- Steven Murawski
- Raph Levien
- and Vesa Khailavirta

for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website, or if you're a company interested in advertising to developers, please email me!

### Info

You can also find links to the docs, repository, and crate page for `quick-xml` at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it! You can also help others discover the show by rating and reviewing it on iTunes, recommending it in other podcast directories, or sharing it around on whatever social media you use.

I'd love to hear your feedback, as well as suggestions for topics, interviewees, and so on, in the threads for the episode on the Rust user forum or Hacker News. And hearing from you via email is one of my favorite things---feel free to send me a note at hello@newrustacean.com. Another Rustacean in my vicinity recently sent me a note and we're going to see if we can meet up sometime soon. And hey, speaking of meetups: if there isn't a meetup in your area, you should think about starting one! Jump in the #rust-community channel and they will help you get going.

Until next time, happy coding!
