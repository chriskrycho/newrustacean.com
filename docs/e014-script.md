# Stringing things along

## Intro {>> 15s → 0:15 <<}

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 14: Stringing things along.*

## News and Follow-Up

This week I have some follow-up!

First, a few weeks ago I got a couple very helpful questions from listener Jeremy W. Sherman, who pointed out that in my discussion of the "unit" type in episode twelve, I was a bit ambiguous about whether the type has a value. That's because I myself was a bit unclear about the best way to summarize it! As he pointed out: it's a *low-information* type, but it has a value: that's what makes it usable as an expression (because, remember: all expressions have values). That value *is* that it is an empty tuple. Even though that's a little odd to think about, it's useful. In fact, it's *essential* for Rust to have the type safety and memory safety guarantees that it has!

My thanks to Jeremy for that clarification!

Second, this week I had a really interesting exchange on GitHub with user **Tamir Bahar, @tmr232**, who brought up a point I hadn't fully considered about my discussion of type coercion in C, in episode 11. He pointed out a scenario where adding together a string and an integer doesn't cause a coercion, since it just does pointer arithmetic. It was a close cousin of the example *I* had in mind, where there actually *is* coercion. I'll link to the discussion in the show notes, as I think it's interesting. One of the things that discussion highlighted for me was the fact that the types *are* trivially coerceable in C: depending on what *exactly* you tell the compiler to do, you may or may not get a warning, and you may well get a segfault regardless. Thanks to Tamir Bahar for the thoughtful feedback and conversation!


## Strings in Rust

So today we're going to talk about *strings* in Rust. You might be surprised it took us so long to get to something which seems so ordinary, but as you'll see, there's a reason I picked *now* to talk about them. By waiting this long, we've been able to lay the foundation we need to understand pretty much everything going on with strings in Rust, so I have a lot less hand-waving I need to do than I would have if we'd gotten here sooner.

One thing to note right away: Rust's strings have some pretty substantial differences from many other strong types you might be used to, especially in C-descended languages. For one, *all* Rust strings are UTF-8–encoded Unicode, not ASCII. For another---and this might be especially surprising to C and C++ developers---they are not null-terminated (so you can have null characters in the middle of strings, and finding a null character is *not* how you find the end of a string).

One small benefit of this is that the length of a Rust string is the number you would expect if you hadn't been trained by years of dealing with C-style character arrays. It's the actual number of characters in the string. I think that's excelelnt, but it does require a bit of readjustment.

### Two kinds of strings

Another thing which people often find surprising about Rust strings is that Rust has two kinds of strings available to us. They're closely related, and it's easy enough to convert one to the other, but they are different, and the differences are important.

The two types of string we see in Rust are the `String` type ("string" with a capital 'S'), and the string *slice* type, spelled `str`. (You'll far more often see it as a borrow, `&str`. I've never seen the non-borrowed form outside the docs, in fact.)

The `String` type is an owned, heap-allocated, growable type. Under the hood, it's a vector of UTF-8-encoded Unicode codepoints.

The `str` slice type is a reference to a specific chunk of memory, consisting of some specific number of those codepoints. Those slices can be allocated with static lifetimes, or they can point to some or all of a given `String`.

As you'd expect, you can create new `String` instances by typing `String::new()`. You can also allocate them with a specific size, with `String::with_capacity()`, which you might do if you know ahead of time how big they're going to be, which can be especially handy if you know a given `String` will only ever have a given value. `String` instances are growable, so if you start out with a default size, or specify the size and it ends up being too small, it's not the end of the world---but there is a slight performance penalty if you have to ask the allocator to give it more memory on the heap. The other *very* common way to create `String` instances is from string slices, including string literals. You can write `String::from("some string literal")`, or you can write `"some string literal".to_string()`---both give you a `String` instance back.

Now, a moment ago I said "string slices, including string literals" because in fact a string literal is always just a stack-allocated `&str` slice, with a `'static` lifetime. The same is true of any string literal constant you define in a module. But you can also get string slices---with non-`'static` lifetimes!---from existing `String` instances or as a subset of another `str` slice.

Now, I keep saying "string slice"; what do I mean by "slice"? This is just Rust's word for a view into a given sequence of associated memory. As we'll discuss more below, there are *lots* of kinds of slices. You can think of all of them, and thus, for our purposes right now, `&str` slices specifically, as being references to contiguous chunks of memory. Specifically, they're a pointer to that memory, and the length of memory pointed to.

### Unicode

Throughout this discussion, I've frequently used the term "codepoints". This is because both `String` and `str` represent Unicode strings. So we need to understand a little about Unicode characters to really see what's goin gon. Unicode strings are streams of bytes which make up *graphemes*. A *grapheme* is what looks like a single character when you're reading. So, for example, when you see the any ordinary English letter, an accented á in Shakespearean poetry, or the ñ character used in Spanish, or the accented vowels in both classical and modern Greek, or the logograms used for Chinese characters, or even an individual musical notation from the relatively recent Standard Music Font Layout, you're looking at a *grapheme*. But those graphemes are often made up of more than one *byte* or *codepoint*. So the Greek diacritical markings take two bytes to represent one character. Many Chinese logograms require three bytes. And Rust strings abstract over this so you can interact with text in whatever language (even musical language) without having to worry about it overall.

However, this does mean that you can't index directly into a string like you would a character array in C or a string list in Python. Instead, we can use a specific API provided for this: on any given `String` instance, we can use the `chars()` method to get an `Iterator` over the characters, rather than code points. Then we can use the `nth()` method, which also comes from `Iterator`, to get the character at the visual location we expect.

Another consequence of handling strings as Unicode is that the *length* of a `str` slice or a `String` is the number of *bytes* which make up the total set of graphemes in the String. But that won't necessarily be the number of graphemes. If I type the classical Greek word "ἀγαπάω", that will be *eight* bytes but *six* graphemes.

### Dereferencing

All the usual rules about mutability, borrowing of references, and lifetimes apply here just like everywhere else. However, when you borrow a `String`, what you get is `&str` reference. That... seems a little strange at first, right?

This brings us to the final thing we need to cover in our discussion of `&str` slices vs. `String` instances: dereferencing. When we make a reference to a given type, we can specify, via the `Deref` trait, what it dereferences into. This trait says what happens when you type `*` in front of some name binding, `*foo`, but it also says what happens in this scenario with string types. If you have a type A which defines the deref trait a target type B, and you type `&A`, it will automatically become `&B`. `String` implements `Deref` with `str` as its target, so when you borrow a `String`, you can have a borrowed `str` slice wherever you need it.

Here's a concrete example from the standard library. The `str` slice type has a method called `chars()`, which gives you the characters in the slice. But we can also call it on `String` instances!

`chars()` expects to take `&str` as its argument. And as you may recall from our discussion of struct methods early on, the `.` syntax for methods is just syntactical sugar for calling a given function with the instance as its first argument. So when you call `.chars()` on some `String` instance, it passes that as the first argument to the method, and since `chars()` expects a `&str` rather than a `String`, it automatically dereferences it for you. Neat!

Note two things carefully.

1. This isn't magic. Because it's built on Rust's trait system, it works for any type equally well. You just have to implement `Deref`.
2. It also isn't silent type coercion. The compiler does this for you, but this only happens when you've said explicitly that one type can be transformed into another. If you haven't implemented `Deref` for a given pair of types, *this won't happen*.

### Vectors and Slices

Of course, as I've noted, this doesn't just apply to string slices and String instances. But that's true for *lots* of things with Strings and slices: most of what we've said here---basically everything except the Unicode details---is equally applicable to other vector and slice types.

These issues tend to come up *most often* with strings, because dealing with strings is such a common case in so many different programming tasks. But a `String` is not a special case in Rust: if you look at how it's implemented in the standard library, you'll see that it's just a struct that wraps a `Vec` vector type.

Vectors are a general-purpose, heap-allocated, grow-able memory type in Rust. That's precisely what makes them useful for the `String` type: they're general purpose, so a string can very easily be just one special case with its own additional functionality. But you can just as easily do the same kinds of things with numbers, or enums, or structs, or whatever else. If, for some reason, you need a vector filled with hash-maps which take strings as keys and enums as values... you can do that. The type signature might be a little complicated, but it's not at all a problem as long as the type is well-defined.

So for any given vector, you can take a slice of it, and you can pass that slice as an argument to a function, and you can pass back a slice from a function. (Of course, those slices are references, so you have to manage the lifetimes properly in those cases, as we talked about in episode 13.) You can create a new vector from a given slice. You can dereference a vector to get to an underlying slice, as long as the `Deref` trait is implemented.

Since `String`s are built on `Vec`, they have access to the whole vector API! Similarly, string slices are, well... slices. In general, the things you can do with `&str` are applicable to *any* slice of any vector, apart from the string-specific APIs like `chars()`. But that's exactly what we'd expect when extending one type with another. (Note that `str` is a *little* bit special, in that it's a primitive type in Rust. But even so, a lot of its implementation is just like you'd see for any other slice.)

### The standard library

This is one of the things that I really appreciate about Rust: the way its standard library is built out of the same basic types that we use for our day to day work in the language. There aren't a lot of special cases. If you want to go look at how a String works... you just look at the source and see how it works. And it works pretty much like you'd expect if you already know how vectors work.

This has two really helpful effects. First, it means that if a given type in Rust is perplexing you, you can usually figure out what's going on just by looking at its implementation. If you keep digging, you'll eventually see how the pieces fit together---and you usually *don't* have to understand a bunch of compiler internals for that to be true. You just have to be able to understand the basic building blocks of the language.

Second, it means there's a lot of pressure on the language team to make sure that those basic building blocks are as good as they can be, because *everything else* is built on them. If vectors were a big mess, strings would be, too. And as a nice bonus, it also means that any improvements to the basic building blocks automatically work for everything built on top of those. Vectors are just structs themselves, with a set of associated traits and trait methods they implement.

When you get down to the very bottom of the chain, you'll find some array types and a little bit of actually `unsafe` code that does full-on memory allocation work, as you'd expect, but even that uses ordinary Rust primitives. These aren't really special cases. I'll link to relevant source files from the show notes, because it's actually quite clear how the types are implemented.

We shouldn't take this for granted. This isn't the only way to approach language design, and there are plenty of languages out there which *aren't* built this way, but instead rely on special cases in the compiler, special syntax or keywords, and so on to deal with these kinds of things. By building everything on top of these core primitives, and by choosing *really good* primitives to build on, Rust lets us build almost anything. And I mean that. If you have a reason to---say, you're building a real-time operating system in the language---you can rip out the standard library and supply your own. The basic pieces you need are all there.


## Outro

So that's an overview of strings in Rust! Next time, we'll start looking at some of the "smart pointer" types and when we would need to use them, and how to use them.

As noted in the beginning of the show, I really enjoy feedback. Jeremy's questions and Tamar's comments both made me think harder about Rust, and I learned a lot from both exchanges!

### Sponsors

Thanks to Hamza Sheikh, Chris Palmer, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, or cash.me, or get in touch with me directly.


### Follow/support

You can find links to those as well as to the discussion threads I mentioned at NewRustacean.com. You can also follow the show on Twitter @newrustacean, or follow me there @chriskrycho. You can help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, tweeting about it, or just telling a friend!

So do respond on social media, in the thread for the episode on the Rust user forum at users.rust-lang.org, or via email at hello@newrustacean.com.

Until next time, happy coding!
