# e023: Traits Deep Dive, Part I

## Intro

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is episode 23:  – the first of two deep-dive episodes on traits. Much earlier in the show (all the way back in episodes 8 and 9), I talked about interfaces at a high level: what they’re for, and how you can implement existing traits. Today, we’re going to dig into some of the mechanics of actually using traits – the nitty-gritty of building your own traits and the concrete limitations on where and how you can implement them, and what you have to do to use traits defined elsewhere. In the next episode, we’ll take a look at the use of traits in place of either concrete types or generics, including a look at the `impl trait` feature that will be landing in Rust 1.26!

## Traits: Deep Dive, Part I

For a quick refresher, let’s review what traits are *for*. Traits are Rust’s primary mechanism for *shared behavior* between objects. Some languages solve this problem with inheritance, or with interfaces that must be applied to an object at the time of definition. Rust has no conventional notion of inheritance, and its traits are quite unlike, say, interfaces in C♯.

A Rust trait defines *shared behavior* only; it cannot define the required *shape* of an object which implements it (e.g. required class fields in traditional OOP language’s interfaces). And a trait can be applied to a type totally independent of the normal definition of that type – you can apply anyone else’s traits to your own types, and your types to anyone else’s traits.

And the only place that Rust *does* get something kind of like inheritance in the normal sense is in traits themselves: a trait can declare that implementing it requires that you implement some *other* trait as well.

So if you’re coming from the perspective of a Java or C♯ or even TypeScript, traits are sort of like method-only interfaces which can be implemented *whenever*, not just at class definition time. Importantly, as we’ll cover in a minute, they can also be applied to any old type – an `enum` is equally valid for a trait implementation as a `struct`.

Traits are *everywhere* in Rust programming, as you can imagine, and the things you can do with them are pretty amazing. So… how do we use them?

### Defining and implementing traits

Let’s start by looking at how you actually define and use your own traits. A trait is always defined using the trait keyword and a block that defines the *items* associated with that trait. (This idea of “associated items” is actually pretty large, and it’s also very important, so we’ll come back to it in a later episode. For now, we’ll leave aside some of those details and just focus on creating and implementing traits.)

#### The basics

Let’s define a slightly silly trait as the basic example we’ll build on for the rest of this episode: `Eatable`, to represent things we can eat. (Apparently I’m writing this episode too close to dinner time!) Anything which implements `Eatable` must have an `eat` method, which returns a description of the act of eating that particular thing. We’d write that like this:

	trait Eatable {
	    fn eat(&self) -> String;
	}

A couple things to notice about this:

1. The function type isn’t explicitly declared as `pub` or any other privacy modifier. And it’s not allowed to. The trait itself can be declared with whatever privacy you like, but items *on* the trait are always public to whatever implements them. This makes sense when you consider that the whole point of a trait is to define what behavior is available for things *outside* a struct or enum to do. Private details of a type don’t belong on a `trait`; they belong in the `impl` block for the type itself, with whatever privacy is appropriate.

2. We have not defined any implementation. We’ve simply written out the type of the function which any implementor has to provide. However, we could have written a basic implementation – albeit, one that doesn’t know anything at all about the type where it’s implemented, and so it can’t say anything that depends on the internals of those types.

```rust,ignore
trait Eatable {
	fn eat(&self) -> String {
	String::from("Open mouth, insert food!")
	}
}
```

Now, implementors can just write `impl Eatable for MyFood {}` and then they can call `MyFood.eat()`, and we’ll get back the string `"Open mouth, insert food!"`. However, it’s often preferable *not* to use a default implementation for a trait, even when it’s defined. In that case, we can supply our own implementation.

Implementing a trait simply means doing `impl TheTrait for TheType` and supplying all required (and any desired optional) elements of the trait. Let’s say we wanted to make a `Breakfast` type – that sounds nice and “eatable.” One thing that’s worth noting, and which I did not realize immediately when I first started learning Rust a few years ago is that you can implement a trait for *any* kind of type – in other words, for enums as well as for structs. So let’s make a `Breakfast` enum with only the sugariest foods:

```rust,ignore
enum Breakfast {
	Waffles,
	Cereal,
	Pancakes,
}
```

Then we can implement `Eatable` with some special behavior for this type:

```rust,ignore
impl Eatable for Breakfast {
	fn eat(&self) -> String {
		match *self {
			Breakfast::Waffles | Breakfast::Pancakes => String::from("Pour syrup, consume"),
			Breakfast::Cereal => String::from("Add milk, then enjoy the crunch"),
		}
	}
}
```

This is what is called *specialization*: instead of just accepting the default implementation of a trait, we’re specializing it for just this one specific object. And we can see even from this silly example why we would want to do that. Remember: the default implementation of the trait didn’t know anything about the actual types that would implement it. And even though we can supply *some* more information about the implementing type with further details in a trait’s definition—we’ll see that in a minute—it’s often the case that a specific concrete implementation will know more about the best way to handle a given trait than the trait itself ever could.

A common example of this that you’ll see often is with the `Iterator` trait: although there are default implementations for most of the methods on the trait, you will sometimes see more specific implementations associated with particular data types. In that case, it’s often for *performance* reasons. Because you implemented the data structure, you may have more insight than `Iterator` does about the best way to implement some of the methods. For example, the `skip` method might naïvely be implemented as just calling the `next` method the number of times you pass to it. However, if you have a data structure where every item is of a known size, and laid out in contiguous blocks of memory—like a C-style array, for example—you could write a `skip` implementation that simply jumps to the appropriate memory offset from the current position in the structure.

#### Other methods

Now, I just implied something very important there: any trait method can call any *other* method defined on the same trait. So let’s say our `Eatable` trait should also describe how the food needs to be prepared. And this one we *cannot* have a default implementation for – some foods are meant to be eaten raw, for example, while others might actually be poisonous if you don’t cook them! So we’ll add a corresponding function to the trait, `prepare`, and we’ll also go ahead and update our default implementation to include preparing.

```rust,ignore
trait Eatable {
	fn prepare(&self) -> String;
	fn eat(&self) -> String {
		self.prepare();
		String::from("Open mouth, insert food!")
	}
}
```

Now we’ve required that every implementor supply an implementation of `prepare`. It’s totally fine for the trait to do this, because it *knows* that no implementor can exist which *doesn’t* have the `prepare` method defined.

This is how a the default implementations for the `Iterator` example can work: they know there is a `next` method defined, and they can define their own default behavior in terms of that `next` method (and one other piece of information we’ll talk about in just a second) – and that’s all they actually *need*.

#### Other items on the trait

Now, about other pieces of information… I’m not going to take a *lot* of time on this, because it really deserves its own whole episode, but traits can also define *associated items*. In the case of the `Iterator` trait, besides all its methods, the trait defines the existence of an `Item` type, which implementors have to define – an `Item` is the kind of thing an `Iterator` iterates over, and which the various methods handle and return. It’s not a struct field or anything like that; it’s *purely* a type-level detail. You can think of it as kind of being like a generic, except that these associated items can themselves be generic. And having them as these (quote-unquote) “associated items” means that we don’t have to specify the type of the generic for every place we call the trait method; we can just say at the place we implement `Iterator` that its `Item` is a `String`, or a `u32` or a `MyStruct`, and then whenever we call it it will *always* be that type, rather than needing its generic parameter specified.

If that flew past you, don’t worry. As I said: we’ll spend a whole episode on associated types in the future: there’s a lot there! For now, it’s sufficient to know that you can require implementors of a trait to say “Hey, the things you’ll operate on in this method are these specific types.” That’s what associated types gives you!

#### Trait inheritance

As I suggested a few minutes ago, traits can also specify that they depend on other traits. So if you’re writing a trait that requires a type also implement `Iterator` – perhaps so you can `map` over it – you write that with your trait name, followed by a colon, followed by the “super-trait” name:

```rust,ignore
trait AwesomeMappable : Iterator {
	awesome_map(&self) {
	// do something awesome
	}
}
```

Anything which implements `AwesomeMappable` now also has to implement `Iterator`. This is handy both for extending the behavior of existing traits, and also for *relying* on the behavior of existing traits, as you define your own.

### Implementing other crates’ traits on your own types

You can also implement other crates’ traits on your own types. So, thinking back to a recent Crates You Should Know episode, you often do just this with the `Serialize` and `Deserialize` traits from `Serde`. As you may recall, in that specific case, you *usually* implement those traits for your types using the custom derive macro: `#[derive(Serialize)]`. But of course you don’t have to: you can write the implementation yourself: `impl Serialize for MyStruct { ... }`. And this goes for *any* trait defined outside your crate and types you want to define inside your crate: `Iterator` is another common example, from the standard library.

### Implementing your own traits for other crates’ types

Now, as I mentioned at the top of the show, one of the things that makes Rust’s traits fairly unique among programming languages in wide use is that you can implement a trait on a type that already exists. The only other examples *I’m* familiar with for doing this are *extension methods* in C♯, extensions in Objective-C and Swift, and type classes in Haskell. Perhaps the biggest difference with C♯ extension methods is that it’s actually extremely *common* to do this in Rust. You *can* do it in C♯, you almost certainly *will* do it in Rust.

Let’s take our `Eatable` trait again, but let’s do something truly absurd and apply it to the regex crate’s `Regex` struct. “But Chris,” you say, “You can’t eat regular expressions.” To which I say: Rust gives us super powers! It even lets us eat regular expressions! We just have to write the `Eatable` implementation!

```rust,ignore
impl Eatable for regex::Regex {
	fn prepare(&self) -> String {
		String::from("This is truly absurd.")
	}

	fn eat(&self) -> String {
		format!("{} But we can do it anyway!", self.prepare())
	}
}
```

Then we can take *any* instance of `Regex` in our crate and call `that_regex.eat()` and we’ll get back “This is truly absurd. But we can do it anyway!” And it is indeed absurd in this case. However, you can imagine many cases where this kind of thing would be handy: anywhere you want to be able to use the behavior you’ve defined on a struct defined outside your own crate. This is handy even just when using crates as strong API boundaries within your *own* codebase, but it’s also handy for dealing with types from *outside* your code.

### The orphan rule

Now, there’s one thing you *cannot* do here: you cannot define an implementation of a trait from outside your crate for a type from outside your crate. The reason is simple: if *you* define an implementation for a trait from crate A for a type from crate B in your crate, and then I define a different implementation for the same trait from crate A on the same type from crate B that you did, and then someone uses both your crate and my crate… which implementation of the trait wins? Rust doesn’t have any good way to resolve it!

The rule is: you can implement a trait for a type as long as *either* the trait *or* the type is local to your crate; you cannot implement external traits on external types. This is sometimes called the *orphan* rule – from the idea that one of the *parent types* (either the concrete type or the trait type) is missing – so hopefully you won’t be confused if you hear that in the future.

### Using trait methods

One other thing that’s worth note here is a little detail that often trips up newcomers to the language: that when you want to use a *trait method* on a given item which implements that trait, you have to import the trait itself. So, for example, when you want to call a Serde serialization or deserialization method on a type which implements those, you *must* reference the appropriate Serde trait with a `use` keyword. The reason for this is that you need to tell Rust *which method* of that name to use. There’s no reason a given `struct` or `enum` cannot have multiple traits which define the same method name, and accordingly Rust needs a way to disambiguate between the options. Requiring the desired trait to be in scope is Rust’s normal way of solving this problem. You don’t *usually* import multiple traits that have the same method on them. (We’ll talk in a minute about how to solve it if you do.)

So, for an easy-to-understand example: quite a few different traits out there in the world may define a `from_str`. And you might want implementations of more than one of them for any given type. You can imagine a `Printable` trait (somewhat analogous to the real `Display` trait in the standard library), and a `Convertable` trait that takes strings and turns them into an appropriate type (somewhat analogous to deserialization) – both of which define `from_str` definitions as appropriate to what they do. If you had a `struct MyThing` which had implementations of both, and both traits were in scope, how would Rust know which one you meant?

This leads to the last detail we need to understand, which is how to *disambiguate manually*. After all, perhaps in some function you end up wanting to use *both* the `Printable` and the `Convertable` trait’s versions of the method. How can we do that?

Here it’s worth remembering that a *method* is syntactical sugar for calling the function with the first argument explicitly. This is true for *all* methods, not just trait methods. (This will be familiar to listeners who’ve spent time working deeply with Python.) In this case, we can call the trait method we want to use directly: `Convertable::from_str("some string literal")`. There’s an example in the show notes showing this exactly with our `Eatable` trait and an even sillier trait called `Nomable`, as in “nom nom nom.”

Your other option is to change the scope for importing a given trait. *Any* block is a possible location for a `use` statement. So you can import the traits you need to use which happen to have the same method name in different blocks – whether that’s different function bodies, or just standalone blocks within a function body –instead of at the top level of the module like you normally would.

## Closing

And that particular detail does it for the *first* half of our deep dive on traits! In the next episode, we’ll dig deep on *trait objects*, including the `impl trait` and `dyn trait` features which are about to land on stable Rust!

Thanks to this month’s $10-or-more sponsors:

- Aaron Turon
- Alexander Payne
- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Derek Buckley
- Fábio Botelho
- Hans Fjällemark
- John Rudnick
- Matt Rudder
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Paul Naranja
- Olaf Leidinger
- Oluseyi Sonaiya
- Ramon Buckland
- Raph Levien
- Vesa Khailavirta
- and Zachary Snyder

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for _this_ episode at <newrustacean.com/show_notes/e023/>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!

