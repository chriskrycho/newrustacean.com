# Meet My Associates

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is Episode 28: Meet My Associates.

## Sponsor: 

First up, I'm really happy to have Manning back sponsoring the show this episode! They're here with a discount on some *video content* they’ve put together. Even better because that video content is by Carol Nichols || Goulding and Jake Goulding—we talked about this content just a few episodes back! I’ve been saying for years that we need more video content: it’s a huge way people learn. Manning’s Rust in Motion is one of the first major video courses for Rust I know of. You can get it for 40% off at [deals.manning.com/new-rustacean](http://bit.ly/2OXnlEb) – there’s a link in the show notes. That link actually gives you 40% off of *anything* at Manning, including their book *Rust in Action*, which is in early access preview. Thanks so much to Manning for sponsoring the show and building some great video content with Carol and Jake!

## Associated Items

Okay, so let’s jump in and talk about *associated items*. You can think of this as effectively being Traits Deep Dive, Part IV, because associated items are one of the last pieces of the story around Rust’s trait system that we haven’t covered. We’ve actually already talked a *lot* about a number of things which go under the label of “associated items”, because “associated items” actually covers basically *all* trait items! However, there are some nuances to that summary which are worth digging into, and there are some important kinds of associated items which we haven’t talked about *at all* yet on the show.

Associated items are—and here I’m quoting the Reference—

> the items declared in traits or defined in implementations. They are called this because they are defined on an associate type — the type in the implementation. They are a subset of the kinds of items you can declare in a module. Specifically, there are associated functions (including methods), associated types, and associated constants.

We should notice here that associated items are a limited set of all kinds of items in Rust. You cannot define just *anything* on a trait. Most notably, you cannot in general define *type members* on a trait. That is: you could define a `trait Nameable` that has a method on it like `get_name(&self) -> String`, but you can’t include a property on it like `name: String`. That distinction is quite important, and it’s one of the ways Rust’s trait system is quite *unlike* most interface types in other languages. It’s possible to imagine a system where Rust's traits *did* define data, but that would end up undercutting one of the most important elements of Rust's core design: the essential distinction between <i>data</i> and <i>behavior</i>. Traits are <i>behavior</i>: that’s why you *implement* a trait for a data type with an `impl` block, just the same way you implement the behavior of the data type itself.

So with that little clarifying note out of the way, let's look at all the things we *can* define on traits: associated functions, constants, and types.

### Associated functions

We’ve talked about functions and methods before, during [Episode 23: Traits Deep Dive, Part I][e023]. There are just two things I want to highlight in this context:

[e023]: https://newrustacean.com/show_notes/e023/

First, trait functions and methods are indeed a kind of associated type, because they’re a *type* that is associated with the thing implementing the trait! Simple as that.

Second, and more importantly, when I talked about these back in episode 24, I skipped over the difference between associated functions and associated methods. Both are functions defined on a trait; the difference is whether the function takes `self` as a parameter in some way. If the function *does* take `self` as a parameter—whether it takes it by reference, moves it, or even takes it as `Box<Self>`—it is a trait <i>method</i>. If it does *not* take `self` in any way, it is just an associated function.

Remember that the general syntax for calling a function attached to a type—whether that type is a struct, an enum, or a trait—is simply `TheType::some_function(with, any, args)`. And methods are just special cases of functions which take `self` as their first parameter. These rules work out exactly the same way for *associated functions*—functions defined on traits—as they do on concrete types.

Notice, then, that methods (which I focused on in my previous discussion) are simply a special case of associated functions in general: they're just associated functions which take `self` as their first parameter. (And this is true of functions vs. methods in general in Rust! You can `impl` a function which doesn't take `self` on *any* type.) For functions which are *not* methods, and which are located on a type, you have to call them with the name of the type, a pair of colons, and then the arguments: `TheType::some_function(args)`. And since trait methods are just a special case of associated functions, you *can* call them the same way! But, as with all methods in Rust, you can also call them like `some_instance.some_method(args)`.

And that's basically all we need to add about associated functions—most of it, I covered back in [the first Traits deep dive episode][e023], so check those out if you need a refresher.

### Associated constants

Next up: associated consts. As I noted a minute ago, you cannot generally define members on a trait. There is one kind of member you can define on a trait, though: *constants*. A trait cannot have any *dynamic* members, but constants are allowed. Those of you who listened to my news episodes for Rust 1.31 and the 2018 Edition may recall that one of the features stabilized with 1.31 were `const fn` types useful for compile time programming. Well, associated constants—designed with the rest of the associated items system all the way back in [RFC #195], and stabilized back in [Rust 1.20]—are another of the tools available for that kind of work.

[RFC #195]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
[Rust 1.20]: https://newrustacean.com/show_notes/news/rust_1_20/

There aren't a *ton* of places where associated constants come up, but they're quite handy when you have a known value associated with given trait that makes more sense with the *trait* than with its containing *module*. Numerics are one of the go-to examples for the obvious reason that there are lots of kinds of numeric constants that we care about, so if you were designing a library for fast numeric computation, you'd presumably want a lot of those values hard-coded as constants—but as constants on *types*, not just free-floating in a module. So you might have something like `Float::PI`, for example, where `Float` is the trait representing floating point numbers, and `PI` is the value 3.1415926… and so on, to whatever appropriate level of precision.

To define this in the trait, you simply write a `const` type declaration, just as you would *outside* a trait – except that, just like with an associated function, you only have to *define* the constant, not *declare it with a value*. So for our `Float` example, in the body of the trait declaration, we'd simply have `const PI: f32`.

I actually ran into a place where this was useful recently: I was building a small tool and needed to define some string types with different maximum lengths. I could have implemented this distinctly on each and every one of those types, but it was easier and clearer to write a new trait called `ConstrainedString`, which had two associated constants: `MIN_LEN` and `MAX_LEN`. Then I could build around it a default implementation for creating a new instance of the type with an associated function *given* those constraints. (There’s an example in the show notes, of course.)

Just as associated functions and methods may or may not have default *implementations*, associated constants may or may not have default *values*. In the case of my example a minute ago, `Float::PI`, you would presumably want this to have a predefined value—it would be really odd and indeed *wrong* for pi to have different values for different implementations! But in the case of my `ConstrainedString` type, I did *not* want to pre-specify the constant values: the whole point is that those differ for each instance of the type. Instead, I define only the types the minimum and maximum lengths (`Option<usize>`) and then allow the implementor for the trait to specify their actual values!

One other thing worth saying: associated consts are an example of a <i>const context</i> – a place where the compiler will perform constant evaluation including on const function types, as I covered in some detail in the second of my two episodes on Rust 1.31 and the 2018 Edition!

### Associated Types

Okay, so much for the easy bits. Now let's talk about associated *types*. This is where things get a lot more interesting – both in the sense of increased power and also, as you'd expect, in the sense of increased complexity! In fact, associated *types* are one of the things that took me the longest to wrap my head around with Rust. (That’s one of the reasons it has taken me more than three years to get to them—along with everything *else* I’ve had to cover, of course.)

Let's start with how you *write* an associated type. It turns out it's exactly as you would expect, given what you already know! Just as you simply write a `fn` signature (with or without a body) or a `const` definition (with or without a value) in the body of a trait definition to define a function or a constant respectively, you write a type annotation just like you would write a standalone type alias declaration: `type AssociatedThing;`.

The canonical standard library example you've used over and over again (though quite possibly without knowing it!) is `std::iter::Iterator`'s associated `Item` type. Check out [the source][iterator-src] if you like—it's just:

[iterator-src]: https://doc.rust-lang.org/1.32.0/src/core/iter/iterator.rs.html&version=1.32.0

```rs
pub trait Iterator {
    type Item;
    
    // and then the various associated functions
}
```

"Just," I say, but this is one of Rust's most powerful features for keeping traits for exploding into a mess of generics upon generics. If you have an iterator that is generic, you can create an `impl` for an enormous number of different concrete types. So, reaching back to the ridiculous example I was using in the first two traits deep dive episodes, `Eatable`: if `Eatable` was a *generic* type, it might look like:

```rust
trait Eatable<B> {
    fn eat(&self) -> B;
}
```

Then we could:

```rust
# trait Eatable<B> {
#     fn eat(&self) -> B;
# }
impl Eatable<String> for i32 {
    fn eat(&self) -> String {
        if *self < 0 {
            format!("{} is gross", self)
        } else if *self < 10 {
            format!("{} is just fine", self)
        } else {
            format!("{} is delicious!", self)
        }
    }
}
```

But we could *also*

```rust
# trait Eatable<B> {
#     fn eat(&self) -> B;
# }
impl Eatable<f64> for i32 {
    fn eat(&self) -> f64 {
        f64::from(*self)
    }
}
```

Which, besides being a lot less interesting, shows us where the problem here is: if we actually try to use this, say by writing:

```rust,ignore
let a = 13;
a.eat();
```

…well, now we have a problem. The compiler is going to tell us "multiple applicable items in scope -- multiple `eat` found" and then (as of Rust 1.32) the nice suggestion, "help: to disambiguate the method call, write `GenericEatable::eat(a)` instead". But if we try *that*, we get another error: "type annotations needed -- cannot infer type for `B`". The only way we can actually make it resolve is by writing `GenericEatable<String>::eat(a)`. EWWWWW. This is gross!

Now, gross or not it is *occasionally* necessary… but the truth is that most of the time we don't want arbitrary and unbounded genericism for our traits like this. Leaving aside the fact that we can't eat an integer in the first place (I mean, unless you decided to make an integer-shaped birthday cake, which I would be fine with), we usually want a *single* implementation of a given trait for a given type: eating an `i32` should give us a *string* describing how tasty it is, not an `f64` or some random struct. In other words, we want the ability to specify *one* other type here when we implement a trait for some specific type.

An example of this rom the ecosystem which I think should help clarify why this is so helpful is Serde's `Visitor` trait. Serde uses `Visitor` implementations for defining how to walk through a deserializer structure – *visiting* each of the items in the data being deserialized. `Visitor` has a single associated type, `Value`: the type of thing produced by a given implementation. You `impl Visitor` for specific types you want to support custom deserialization for. If `Visitor` were a *generic* trait, then you'd end up with no way to say "always produce a type that's actually appropriate for using with `ValidatedEmail` type – but that's what you want. You don't want `visit_str` to produce a different output type when Serde is trying to deserialize into a `ValidatedEmail` than `visit_string` or `visit_borrowed_string` do. But with a generic, that would be quite possible. What you want instead is to have a single implementation of `Visitor` for `ValidatedEmail`, where the `Value` produced by that visitor is always a `String` – but with a *deserialization error* if it is not a valid email.

And that's the beauty of associated types: when you implement a trait for a given type, it will only have exactly as many associated types as the *trait* defines. Two associated types on the trait, two concrete types for an implementation of the trait for some other type – vs. generics, where the number of generics is *multiplicative*: two generic types for a trait means *every* implementation for a type adds two more.

## Things to come

There are also two features in this general bucket which I'm *not* going to cover, but which fit into this story and you should know about; I expect to cover them in a news episode as appropriate when they're stabilized.

The first is *associated lifetimes*. These were defined in [RFC #195] with the other associated items, but (at least so far as I can see) were never implemented, and in fact [the Reference currently says][associated lifetimes] "Only functions, constants, and type aliases can be associated." The point here, per the RFC, would be to provide the same kind of constrained generic programming ergonomics for lifetimes as associated types provide for types. (This makes sense: lifetimes and types are both parts of Rust's *type system*, and they're closely related. That's why they both go in the same rough places syntactically in the language!)

[associated lifetimes]: https://doc.rust-lang.org/reference/glossary.html#associated-item

The idea of associated lifetimes leads us directly into the other thing, which I *do* expect to land sometime in 2019 or 2020: *generic associated types*, sometimes labeled with the initializism GATs, from [RFC #1598], where the feature was originally called "associated type constructors." I am not going to dig *especially* deep on this today, as I expect I will cover it in considerable detail when it eventually stabilizes. The gist is that an important limitation of today’s associated types will be lifted: they will be allowed to be generic themselves. Let’s flip that around: today, an associated type cannot be a *generic* type. So… why would we want it to be? After all, we just discussed a minute ago how the whole point of associated types is to avoid the problem that comes from the explosion of types with generics.

Here I’ll use an example from the original RFC here: let’s say you wanted to write a trait that handled both the `Rc` and the `Arc` reference counted types, a “family” of types as it were. (For those of you with lots of type theory, yes, this is a *step* toward one way of capturing higher-kindedness, by extending Rust’s existing type system. For those of you without arcane type theory knowledge, don’t worry about it!) Ideally, you’d like to be able to have a `PointerFamily` trait with an associated type, `Pointer`, which you could then fill in for `Arc` or `Rc`… but `Arc` and `Rc` are `Arc<T>` and `Rc<T>`, so you can’t do that. We need to be able to say `type Pointer<T> = Arc<T>`, which means we need to be able to write the trait as something like `type Pointer<T>: Deref<Target = T>`. The same kind of thing goes for lifetimes, which are their own kind of generics. So, some time hopefully this year, that will land (and I’ll give it an appropriate deep dive when it does).

[RFC #1598]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md

## Outro

And that does it for associated items! Hopefully you have a better feel for associated *types* in particular; I know digging into them this way was quite helpful for me.

Thanks as always to this month’s $10-or-more sponsors:

- Alexander Payne
- Andrew Dirksen
- Anthony Deschamps
- beaorn
- Behnam Esfahbod
- Brian McCallister
- Bryan Stitt
- Chip
- Chris Palmer
- Dan Abrams
- Daniel Collin
- Daniel Mason
- Embark Studios
- Graham Wihlidal
- Jako Danar
- James Hagans II
- Jerome Froelich
- Johan Andersson
- John Rudnick
- Jonathan Knapp
- Joseph Marhee
- Martin Heuschober
- Matt Rudder
- Michael Mc Donnell
- Nathan Sculli
- Nick Gideo
- Nick Stevens
- Nicolas Pochet
- Oluseyi Sonaiya
- Paul Naranja
- Peter Tillemans
- Ramon Buckland
- Raph Levien
- Rob Tsuk
- Ryan Osial
- Scott Moeller

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/e028/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!
