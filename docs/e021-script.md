# Keeping your types under cover

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast: a show about learning the Rust programming language. This is Episode 21: "Keeping your types under cover".

## Overview

Today we're going to take a step back and look at a couple relatively easy tools in Rust. These are some of my favorite kinds of things in programming languages: tools which are not at all complicated or arcane, but which make the experience of programming in the language *much* better in some way or another. Today, we're going to talk about two different kinds of type declarations: *type aliases* and *new types* -- with a couple variations on how we can build the latter.

### `type` declarations

Let's start with `type` declarations. The simplest kind of type declaration is something like `type Email = String;`. That defines a type you can use throughout your program, `Email`. This is a really convenient way of making your types more descriptive. After all, there are *lots* of kinds of `String`s out there, and giving things good names is a really helpful way of clarifying exactly what you mean. Instead of being left wondering, "Is this an email or a name?" the type itself can express some of that intent.

Now, one important thing to understand is that type aliases are tools of convenience, *not* of type safety. When you write `type Email = String;` you're not creating a special type. *All* you are doing is creating a nickname for the longer type. If you're coming from Haskell, this is *not* a newtype, in other words. I have often wished there were a native facility for doing newtypes in Rust, because usually when I create this kind of thing in my own code it's *not* for a shorthand version, but because I actually want some type safety around something. More on that in a moment.

Still, these kinds of things can be extremely helpful: even where you're perfectly fine accepting the type being aliased, you have now expressed to whoever is reading your code the actual *point* of that particular type. For example, we might have a function that sends an email; but let's say that in this particular context we have a good reason to name the function `send_message`. (This is a slightly silly example, as `send_email` would normally be a better name. But I'm using it to illustrate the point more clearly.) In that case, we might have a function definition something like:

```rust
fn send_message(toAddress: Email, content: String): Result<(), ErrorCode> { ... }
```

It's really obvious even just looking at that signature that there's supposed to be a difference between the `toAddress` parameter and the `content` parameter, because their types are named differently. Now, of course, a caller could still get that wrong (and we have a solution for cases where we want to make sure the caller *can't* get this wrong, which we'll come back to in a few minutes). But at a minimum, the type signature makes it *even more* explicit what kind of thing we have here. It even clarifies that sending this message will send it specifically to an *email* address, not a physical address. So it's a real win even in this simple and rather silly example.

Of course, precisely because this example *is* rather silly, it may not be obvious where you'd really want to use something like this: aliases for `String` might clarify your intent slightly, but good function and parameter names are actually probably *more* helpful here. The biggest places in Rust where these kinds of things are useful are type signatures you have to repeat a lot; and *complex* type signatures.

Let's start with often-repeated type signatures, and with a type alias you're almost certainly familiar with if you've spent much time with Rust: `std::io::Result`. Normally, the `Result` types have two type parameters, `T` for the type returned in the success case, and `E` for the type returned in the error case. However, `std::io::Result<T>` has only *one* type parameter: `T`, for the expected type. That's because `std::io::Result<T>` is a type alias for `Result<T, std::io::Error>`. Since all errors in this case will be of the type `std::io::Error`, we don't need to make users (or the standard library team, for that matter!) type that out anywhere they want to return a `Result` within `std::io`. They can just write that it returns a `Result<String>`, for example, instead of a `Result<String, Error>`. To make this work, we just write a generic type alias: `type Result<T> = Result<T, std::io::Error>`. If you look in the Rust docs or source, in fact, that's almost exactly what you'll see. (The only difference is that the full namespace isn't written out because the `std::io` namespace is already in context.)

Another way to think about this is that we're making a generic type more specific. `std::result::Result<T, E>` can take *anything* as its type parameters. `std::io::Result<T>` can take anything for that `<T>` parameter, but it's much more specific than the `std::result` type, because it always and only returns `std::io::Error`s for its error type.

The other major scenario we care about is when we have a complicated type that we just don't want to write out over and over again. It's not hard to bump into these in Rust, *especially* when dealing with smart pointers. Imagine you have an `Arc<Vec<Result<String, ErrorCode>>`. Even though it's a totally concrete type (no generics in sight), you *really* don't want to have to write out that whole type every time. Or at least, *I* don't. So instead you could give that a good, local name. The type would still be exactly the same, but you could always just use the type alias instead. You could name it `ThreadSafeCollection` and just say *that* anywhere you need it, and it simplifies your codebase enormously.

Now, I want to pause for a moment before moving on to "new types" to trace out an implication of the fact that these really are just convenient names for longer types. Because that is the case, *any* function that works on the full type name also works on the type alias. That ends up being particularly handy when dealing with things which implement traits like `Iterator`: if you can `map` over the type being aliased, you can `map` over the alias too.

### New types

All well and good: those use cases really are important. But what about the times when we *don't* want the original type to be interchangeable with the new type we're defining? I used the example of an email address initially, and we might well want to distinguish between an email address and a more general string. This is where Haskell's newtypes come in handy, but it's the only language I know of that has them. Most other languages in the same broad lineage as Rust have a nice, low- or no-cost way to accomplish the same thing, though, and Rust is no exception!

We have two closely related ways to create types which are *distinct* from a type-checking perspective this way. One is a *tuple struct*; the other is a *single-case enum*. They have the same result: they just wrap another type in a distinct type declaration.

#### Tuple structs

For a tuple struct, you'd write that like:

```rust
pub struct EmailStruct(String);
```

Then you can declare an instance by writing:

```rust
let address = EmailStruct("hello@newrustacean.com".into());
```

(Recall that `.into()` takes the string literal, which is a string slice reference, and makes it an owned `String`.) When you *used* the address, you can destructure it. Say we were using an external email library which had a `send` function which just took a string. To unwrap it, we could either just use the `.0` accessor, or (my preference), explicitly destructure it.

#### Aside: destructuring assignment

We haven't talked about destructuring assignment before, though, so before I *do* it, let's pause and talk about what it *is*.

Destructuring assignment uses the same kind of syntax as pattern-matching, but outside the body of a `match` expression. It lets you bind directly to any public member in the interior of a struct or enum variant. (Note, though, that you *can't* do this enums outside the single-variant case, because you have to match against all its variants.) It's incredibly handy for getting at internals of a given type without writing something like `let the_value = the_instance_of_the_type.the_value`. Going back to our `EmailStruct` example, that would look like this, recalling that we'd created an `address` variable with the value attached:

```rust
let EmailStruct(emailString) = address;
send(emailString);
```

Delightfully, I wasn't exaggerating when I said destructuring assignment has the same syntax as pattern-matching arms; I was being exact. That means that any kind of destructuring you can do in a pattern-match, you can do here. So let's say we had a more complicated `struct`, one with a couple fields:

```rust
struct ThingToDestructure {
    a_field: String,
    another: i32,
}
```

We could declare an instance like usual:

```rust
let thing = ThingToDestructure {
   a_field: "Neat!".into(),
   another: 42,
};
```

Then we could destructure it – and, handily enough, we can rename things in pattern matches. So, for example, if we didn't want to use `another` locally, we could call it `can_rename` instead:

```
let ThingToDestructure { a_field, another: can_rename } = thing;
```

Destructuring assignment isn't necessarily something you need *all the time*, but when it's handy, it's *very* handy.

#### Single-variant enum

Now, back to the main topic – what about the other way of building special types? Well, for a single-variant enum, you'd declare it by writing:

```rust
pub enum EmailEnum {
    Address(String),
}
```

Creating an instance of the num is just like creating any other enum instance:

```rust
let email = EmailEnum::Address("hello@newrustacean.com".into());
```

Notably, however, you can use destructuring assignment here just like in the struct tuple case, *because* it is a single-case enum:

```rust
let EmailEnum::Address(emailString) = email;
send(emailString);
```

You can probably tell even just in listening to me walk through those verbally that you're basically always better just using a tuple struct in these cases. There's a lot of extra keyboard mashing involved for no actual gain by using an enum in this case. I brought it up for two reasons, though: first, simply so you know what you're looking at if you happen to bump into it; and second, because if you're coming from another language with tagged unions (F♯, Elm, etc.) that might be your first inclination, since that's how you usually do it there. And it does *work* in Rust. There's just no reason to do it here, since we have tuple structs available.

Now, there are a couple other extra challenges we face when defining a custom type like this. One is that, unlike a type alias, you don't get all the various implementations which apply to the inner type for free. You can implement them *fairly* easily, of course -- I provided a simple example of implementing `Iterator` in the code samples for this episode so you can see how it might work -- but you *do* still have to implement them yourself. And that does dramatically decrease the convenience. So you'll have to evaluate whether the work of implementing those APIs yourself is worth the extra type safety you get from defining the type as a tuple struct rather than just a type alias. And it won't always be obvious.

Now, one thing you *can* do when implementing a newtype is implement the  that sometimes you want all this type safety *and* want to be able to use the underlying type in some places. You can implement the `Deref` trait, so that calls can take advantage of deref coercions. In our email address example, then, we might implement `Deref` for the `EmailStruct` with a target type of `String`. Because `String` in turn implements `Deref` for `str`, you could then pass your `EmailStruct` *by reference* to anything which expects a `&String` or (more likely) `&str`. Similarly, if you wanted to be able to move the contents out of the struct into an email, you could implement `Into<String>` for `EmailStruct`, and then anywhere you were ready to move ownership into something that required an owned `String` you could just do `address.into()`.

Of course, you should be careful with these kinds of tools: it's easy to actually just end up throwing away a lot of the type safety you're trying to buy yourself with these kinds of "new types" in the first place. But you at least have the option. My rule of thumb for these kinds of things (across languages) is that the more critical to the business rules something is, the more likely I am to write a real type and whatever custom implementations I need. By contrast, where it's just a convenience issue -- I don't want to write that same long type signature with multiple layers of generics over and over and over again -- I'm happy to use type aliases instead.

## Outro

That's about all there is to say about type aliases! They're a really handy feature, but they're not an especially *complicated* feature. Hopefully you'll get some good mileage out of them!

Thanks to this month's $10-or-more sponsors:

- Anthony Deschamps
- Chris Palmer
- Benham Esfabod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Matt Rudder
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

As always, thanks as well to everyone who helps other people find the show---whether by just telling a friend, or by rating or reviewing it in iTunes or recommending it in another podcast directory, or by sharing it on social media!

You can find show notes for this episode, including code samples for everything I discussed, at newrustacean.com/show_notes/e02. The show is on Twitter @newrustacean, and I am @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or – and this is my favorite – just send me an email at hello@newrustacean.com.

Until next time, happy coding.