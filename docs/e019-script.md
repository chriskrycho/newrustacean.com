# Let's `Clone` a `Cow`!

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 19: Let's `Clone` a `Cow`!*

### News-y thing

One quick note before we jump into the episode: if you hear before March 2017, you should take a look at underhanded.rs, where the Rust Community Team is running a contest to create deliberate vulnerabilities in Rust software, to help improve security all around---which is hugely important, given one of Rust's goals is making more secure software! If that's something your interested, check it out.

Now, into the show proper!

### Setup

Over the past several episodes, we've been working through some of the different patterns Rust offers us for dealing with managing memory.

- In episode 13, we looked at lifetimes---what they are, how to write them, and when you need to write them.
- In episode 14, we zoomed in on the details of managing strings specifically.
- In episode 15, we looked at *smart pointers*---especially `Box` and the `Rc`, `Arc`, and their corresponding `Weak` types.
- In episode 16, we followed up by looking at how we can use the different *cell* types to 
- In episode 17, we took a step back and just talked about the general syntax and semantics of referencing and dereferencing and the corresponding operators.
- In episode 18, we talked about the `Borrow`, `AsRef`, and `Deref` traits.

Lots and *lots* of discussion about memory management. That makes a lot of sense: in some sense, Rust's whole reason for existing is that we need a way to manage memory *safely*. But this episode is going to pretty much wrap that up, at least at a first pass, because we'll largely have covered things where memory itself is front and center. To be sure, the topic will continue coming up, e.g. when we talk about the traits for thread safety---but it'll be a little less front and center.

In today's episode, we'll tie this up with a bow, talking about the `Copy` and `Clone` traits and the `Cow` type!

## Precursus: more on traits

Before we start, though, we need to pause and talk about a few things we skipped over when we talked about traits back in episodes 8 and 9: default implementations, marker traits, and super-traits.

### Default implementations

A trait can supply a *default implementation* when the normal or default behavior can be known ahead of time. Most traits don't have these, because in most cases you don't have any way to know ahead of time how a given type will need to implement the interface. For example: equality and ordering, expressed with the `Eq` and `Ord` traits, simply couldn't have default implementations. But on the other hand, there *are* cases---one of which we'll see below---where we can know what a good default implementation is. The current version of the Rust book uses the example of a trait which supplies `is_valid()` and `is_invalid()` methods. You might not know what `is_valid()` should return, but regardless, you can probably assume that `is_invalid()` can just return `!is_valid()`.

Of course, implementors need a way to change the implementation if there's a better way to do it. In that validity-checking scenario, for example, it might be the case that you need to do an exhaustive check, but could more cheaply do a sequence of checks and therefore have better best-case performance for calling `is_invalid()` than negating `is_valid()`. In that case, you just add the method definition in the `impl` block like with any other trait method implementation.

### Marker traits

When we talked about traits originally, we described them as a way of describing common structure or behavior. However, there is a set of traits which have no functionality of their own, but which simply *mark* the implementing type as having certain properties. There are a number of these traits in `std::marker`, including `Send`, `Sized`, `Sync`, and---important for our purposes today---`Copy`. There are also others elsewhere in the standard library, including the `UnwindSafe` and its buddy, `RefUnwindSafe`, in the `std::panic` module.

The idea of a *marker trait* is that it's totally empty, and simply used to serve as a *marker* for some kind of guarantee about a type. You can even implement your own if you have a good use case for it; the aforementioned `UnwindSafe` types aren't built into the compiler but are simply defined in `std`.

### Supertraits

Now, what are *supertraits*? Traits in Rust can indicate that they are "subtraits" of another trait. If you've coming from an object oriented background, your first instinct on hearing that might be to think of them as analogous to a *subclass*, but that's not it. Rather, it's like an interface *which extends another interface*. In today's discussion, for example,, we'll see that `Copy` declares `Clone` as a supertrait. By doing that, the `Copy` declaration is actually saying it is *more specific* than `Clone`. That is: all `Copy`-implementors are `Clone`-implementors, but not all `Clone`-implementors are `Copy`-implementors. We'll talk about the reason for that below.

You write a sub-trait relationship by using a colon and the name of the super-trait after declaring the trait. So in the case of `Copy`, it's written `pub trait Copy : Clone` and then the (empty) body. There are *many* such sub- and super-trait relationships in the standard library. They're really much the same as any generic type constraint, except that they apply to traits rather than to generics. For example, `Eq` requires `PartialEq`, `Ord` requires `PartialOrd`, and so on.

## `Copy`, `Clone`, and `Cow`

### `Copy`

The `Copy` trait applies to types---and here I'm quoting the standard library docs---"whose values can be duplicated simply by copying bits." That's distinct from types whose values *can't* be duplicated simply by copying bits, usually because of some degree of indirection behind pointers. Put another way, these are *simple* types---there are no references to worry about here, because if there were we couldn't just copy the bits. Why that constraint matters, you'll see in a minute when we talk about `Clone`---because `Clone` is a necessary precondition for `Copy`.

However, in the normal case, Rust *moves* the ownership of the data whenever it can. Here's an extremely simple example of that (which I've written out in the show notes). Let's say we declared a `struct Point` with `f64` members `x`, `y`, and `z` and an `origin` method `origin` in the `impl`. Then we could declare a point: `let a_point = Point::origin()`. Now, if on the very next line, we write `let moved_point = a_point`, and then try to `println!("{:?}", a_point)`---that is, try to print the original point---the compiler will give us a "use of moved value" error. Rust *moved ownership of the data* from the original `a_point` binding to the new `moved_point` binding. As such, we can't get back to the original binding. This would violate Rust's core rules about ownership---remember, only one binding gets to *own* the data at a time.

There are times, however, when we just want to make a copy so that we *can* still access the original value. You can imagine plenty of times when dealing with integers, for example, where you might want to set up the initial value of one integer with the current value of another, but still be able to use the original. This is what `Copy` is for: the case where we want to get the same *values* in a new, owned binding, while leaving the ownership of the original data with the original binding. The big upside is that we can make an assignment and then continue using the original binding. The big downside is that copies can be extremely expensive for very large data structures.

Note that this distinction between moving and copying data is a *semantic* rather than a *mechanical* distinction: sometimes, though not always, with move semantics Rust can re-use the same memory space for the next "owner" of the data. That kind of space reuse is a small win for something like an integer, but for some large enum or struct type, it can be a big win, especially over the life of the whole program. That's a nice optimization when it can happen. But most of the time, Rust *does* copy the data over even if when using move semantics. The point of the `Copy` trait is *ownership*, not instructing the compiler how to handle the memory.

And that makes sense. After all, if you specifically *need* to reuse the same memory, you can just take a reference!

So that's `Copy`. But of course, I brought up marker traits for a reason: namely, that `Copy` is a *marker trait*. All it does is tell the compiler to use copy semantics instead of move semantics with types which implement it.

### `Clone`

But this raises an important question: *how* does the Rust compiler perform the copies? After all, if you want to copy the values for anything but extremely simple value types, you have to know *how* to copy them. For example, if there are references involved, making a true duplicate involves following all those references and creating copies of the values behind those references, and then creating new references to those new copies. You may have heard this described as "deep copying" in contexts like JavaScript.

The `Clone` trait is the trait which tells Rust how to perform those copies, and so all `Copy` types must also be `Clone` types. Put another way: `Clone` is a supertrait for `Copy`. 

But `Clone` is also much more generally useful. After all, there are plenty of places you want to `clone()` something while retaining normal move semantics instead of switching over to copy semantics. When dealing with `String`s, for example, it would be extremely expensive to make them `Copy` types: that would mean you *always* made a full copy of a `String` any time you assigned it. That would destroy a lot of the performance wins we all appreciate in Rust. But you do need a way to specify how to get a copy of a `String` when you actually *need* a copy. And of course, it's extremely valuable for there to be *one* way to do that, consistent across a variety of types (even if the implementation details differ on a per-type basis). *That's* where `Clone` comes in.

The `Clone` trait specifies two methods: `clone` and `clone_from`. Implementors *must* implement the `clone` method, and *may* implement `clone_from`. I say *may* because a default implementation of `clone_from` is supplied by the trait. The default implementation is just to do a bit-wise copy of all the elements of the type. For many types, that's exactly what you want. But as with all default method implementations, you can override it and implement it yourself if you need to.

If you're wondering about the deep copy problem: well, this is why you can only implement `Clone` for simple types *or* types which contain only *other* `Clone` types. There's no way to *automatically* deep-copy reference types unless those reference types *also* define how to clone themselves.

However, once you *do* have a type whose members are all `Clone`, you can just use the `#[derive]` attribute (which we talked about in more detail back in episode 7) with the `Clone` trait. And this makes sense: if you know how to `Clone` every member, then `Clone`-ing the type is just recursively cloning its members. In other words, `Clone` is just a normal deep copy---but with Rust's strong type guarantees wrapped around it, and the nice ergonomics of the `[#derive]` syntax sugar.

And because `Clone` is a supertrait for the marker trait `Copy`, any type which is `Clone` can be `Copy` for free. And just as any type whose members are all `Clone`, any type whose members are all `Copy` can also be `Copy`.

### `Cow`

Now let's pivot slightly, put some of these pieces together, and look at the `Cow` type. In this case a "cow" is not a large, possibly smelly animal, but a "clone on write," or "C-o-w" smart pointer. The `Cow` type pulls together the `Borrow` and `Deref` traits (discussed in episode 18) and the `Clone` trait (which we just talked through). With `Cow`, we have a type that gives you *immutable access* to data you are only *borrowing* access to (using the `Borrow` trait), but lets you get a mutable *copy* of the data if you need to mutate it (using the `Clone` trait). That way, when you have a piece of data where you don't know until runtime whether you'll need to mutate it or not, you can just wrap it in a `Cow`. You can wrap up any other type in a `Cow`, just like you can in a `Box` or an `Arc`. Since it implements `Deref`, it will do that one nice little bit of automatic type coercion the Rust compiler does, and you can transparently use it with functions which expect the wrapped type. Of course, just like with other smart pointers, you have the *small* overhead of allocating a smart pointer on the heap. And you have the potentially very large performance costs of doing a deep copy on a large data structure.

But This can make our APIs a lot friendlier. If you're not in a performance critical section of the code, you may not care that much about whether a copy happens when you write something, and accordingly you may not want to have to write `.clone()` every time you need to make a copy and do something with it while leaving the original ownership in place.

Under the covers, `Cow` is actually an `enum` with two variants: `Borrowed` and `Owned`. The `Owned` type explicitly wraps around a trait which is *also* called `Owned`, and which is itself (and here I'm quoting the docs again) "a generalization of `Clone`": it not only lets you go from a *reference* to some type `T` to an owned copy of the value of `T`, but also from *any* borrow, including those created with the `Borrow` trait. The `Borrowed` variant of `Cow`, meanwhile, lets the type wrap around borrowed data as well. So `Cow` is general purpose and can be used equally with, for example, a `&str` borrowed type, or with a `String` owned type.

Here's the neat thing: we're not working around or dodging the borrow checker somehow with this. We're actually working *with* it. We only ever borrow references to data we need to access immutably. When we get to a point where we find that we actually need to write a change, the `Cow` will automatically run its implementation of `clone` and hand us a *copy* of the data. But the thing that's nice about this is: you don't pay that cost of copying the data *all the time*. You only copy the data when you *change* it. And you're also not paying the ergonomic costs of having to think explicitly about ownership all the time.

As long as you consider the performance tradeoffs carefully, though, it can be a win: `Cow` makes for a really nice abstraction. I should also note in closing: `Cow` is pretty straightforward to implement---you can go look at the source (which I'll link in the show notes) and see for yourself. There's nothing magical happening there, and less than 100 lines of *very* readable Rust code to implement the whole thing.

## Closing

So that wraps up our journey through single-threaded memory management at a high level in Rust. In the months ahead, I'm expecting to do another interview. I also plan to tackle a question a high-tier sponsor posed about structuring and organizing larger codebases, to talk in some detail about Cargo, and then to dive back down into more nitty-gritty details!

### Sponsors

Thanks again---as always!---to 

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

You can also find links to the docs, repository, and crate page for `clap` at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it! You can also help others discover the show by rating and reviewing it on iTunes, recommending it in other podcast directories, or sharing it around on whatever social media you use.

I'd love to hear your feedback, as well as suggestions for topics, interviewees, and so on, in the threads for the episode on the Rust user forum or Hacker News. I also love getting email---you can send me a note at hello@newrustacean.com.

Until next time, happy coding!
