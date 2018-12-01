# e027: I Promise; Trust Me

Hello, I’m Chris Krycho, and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is Episode 27: I Promise; Trust Me.

## Sponsor: Parity Technologies

Parity Technologies is here sponsoring another episode! Parity is advancing the state of the art in decentralized technology. Their flagship software is the Parity Ethereum client, but they're also building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Their next big project is Polkadot, a platform leveraging blockchain tech for scaling and interop in decentralized systems. Parity uses Rust for its trifecta of safety, speed, and correctness—and they’re still hiring Rust developers! Check out their jobs at paritytech.io/jobs.

Thanks again to Parity for sponsoring the show!

## What is safety?

One of the most interesting claims Rust makes is that it can help us write *safer* code. I call this interesting because there’s a lot bundled up in that word: “safe”. We’re digging in, this episode, to the idea of safety through the lens of Rust’s keyword `unsafe`. But if we want to understand what we’re saying with `unsafe`, we need to understand what Rust means by “safe” in the rest of the code. Code marked `unsafe` is unsafe by *contrast* with the rest of Rust.

So: what is safety?

Safety is a specific set of guarantees about specific kinds of undesired behavior around memory. It is *not* a guarantee that your program will never crash, or that you won’t have logic bugs, or that you won’t get into deadlocks. It is a claim about *memory access* and *threads*, not about logic! The list of things Rust protects us from (and here I’m quoting from the Nomicon, the official Rust guide to writing `unsafe` Rust):

> - Dereferencing null, dangling, or unaligned pointers
> - Reading uninitialized memory
> - Breaking the pointer aliasing rules
> - Producing invalid primitive values:
> - dangling/null references
> - null `fn` pointers
> - a `bool` that isn't 0 or 1
> - an undefined `enum` discriminant
> - a `char` outside the ranges `[0x0, 0xD7FF]` and `[0xE000, 0x10FFFF]`
> - A non-utf8 `str`
> - Unwinding into another language
> - Causing a data race

That’s not a short list… but it’s not that long of a list, either! There are still plenty of things that can go wrong. For example: data races are prevented… but you can still have race conditions in a more general sense: those races just can’t cause memory unsafety.

However, that list *does* eliminate a lot of the worst kinds of unsafety in the world of programming – and in particular, it eliminates the kinds of bugs that cause segmentation faults or corruption in the data of your program. This is what makes Rust such a big deal: those kinds of guarantees have only ever been available (in mainstream languages at least) in languages with managed memory. So safe Rust is pretty safe!

## `unsafe`

So when we talk about “unsafe” Rust, does that mean all the rules go out the window? Happily, *no*

We’ll dig into details around what it means to mark blocks, functions, or traits with the `unsafe` keyword in a minute. First, though, it’s important to understand that writing `unsafe` Rust does *not* mean you’re now in a free-for-all zone. If you’re *not* using one of the very specific additional abilities that `unsafe` unlocks, all of Rust’s other normal compiler rules and safety checking behaviors are still in play. You heard that right: the compiler checks you just as much inside an `unsafe` block as outside, other than some very specific abilities.

So, for example, if you’re using a normal Rust reference instead of a raw pointer, the borrow checker will still check your access, just like normal! I’ve linked to [a Rust playground example](https://play.rust-lang.org/?version=beta&mode=release&edition=2018&gist=38d1089cdc3a4148609e9e3bbbfd002c) in the show notes so you can see exactly this (along with a warning that the `unsafe` block in the example is unnecessary).

The takeaway here, before we dig into what each of those special `unsafe` abilities gains us, is that they don’t remove Rust’s normal safety checks in a general sense – they just let you do these specific *additional* (unsafe) things. As the Nomicon (the official Rust guide to `unsafe` Rust) [puts it](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html):

> The unsafe keyword has two uses: to declare the existence of contracts the compiler can't check, and to declare that a programmer has checked that these contracts have been upheld.

One other thing to notice: just because we’re writing code in an `unsafe` block does *not* mean that what we’re doing is actually not a safe thing to do. In fact, the *point* is the opposite! We intend to write *safe* code in `unsafe` blocks. The difference is whether the compiler can know they’re safe or not. For this reason, people have suggested—with various degrees of seriousness, and none of them serious enough to write an RFC—that `unsafe` should have a different name: something like `vouchsafe` or `trustme`. Because code in an `unsafe` block is code where the normal trust paradigm with Rust is inverted: instead of *my* trusting the compiler to keep the code from ending up in conditions that would, say, segfault, the compiler has to trust *me* to keep the code from ending up in those conditions.

So: what are the special abilities unlocked by the `unsafe` keyword? Just these ([quoting from](https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html) the most official source: <cite>The Rust Programming Language</cite> book):

> - Dereference a raw pointer
> - Call an unsafe function or method
> - Access or modify a mutable static variable
> - Implement an unsafe trait

We’ll talk about each of those in turn.

### Dereferencing raw pointers

The first thing you can do with `unsafe` is dereference a raw pointer. Let’s start by talking a little about the difference between a raw pointer and a normal “reference” in Rust, since “pointers” and “references” often get conflated in imprecise discussions. The difference between pointers and references is less one of *mechanics* and more one of *semantics*: what they mean and therefore what the compiler will let you do with them.

Under the hood, both a raw pointer and a reference contain a memory address. However, semantically speaking, the *value* of a pointer is that memory address, while the *value* of a reference is the thing it’s pointing to.

When you interact with a reference in Rust, you’re never concerned with the specific memory address involved: it is *only* a way of getting access to some item where it already exists, rather than having to copy all of its data around over and over.

When you interact with a pointer in Rust, you are interacting with a specific `isize` value, which contains a memory location. It’s an `isize` because `isize` is the name of the type which is large enough to hold a memory address on the architecture you’re running on – so for a 32-bit architecture, `isize` is 4 bytes, and for a 64-bit architecture, it’s 8 bytes. If you somehow managed to get Rust running on an 8-bit architecture, it’d be 1 byte! But the point here is that the pointer is not just an indirection to get at some piece of data somewhere in memory: it is the address itself.

That difference goes right down through the division between safe and unsafe Rust. Pointers and references are still pointers and references. And pointers are things you can do things with in safe Rust! You can get pointer values in safe Rust, e.g. with `Box::into_raw`. You can even mutate them, so that they point at something else, in safe Rust. (There’s an example in the show notes, as well as [a link to a Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=3a7a9facd0f67d4a590afc3a3ecef95b) showing exactly this.) The huge thing you cannot do in safe Rust is dereference a raw pointer. That is: you cannot go “through” the pointer to get at the data at that memory location. In unsafe Rust, you can.

The reason for this is simple: let’s say I get a pointer by calling `Box::into_raw` with a valid `Box<SomeStruct>`. That pointer is valid. But  now, as I noted a second ago, I can change it. I could, say, just put the hex value `0x10` in it. And behind the address `0x10` is… who knows what? It could be uninitialized. It could be empty. It could be a different piece of data – maybe one being used by another thread! And when I call `Box::into_raw`, I get back a *mutable* pointer, so if I could dereference it, I could mess with the values on the other side of it. All of which means that I can create just about any kind of mess you can imagine if I’m allowed to dereference a raw pointer!

Many of the data structure implementations in the standard library use `unsafe` heavily to do things that can be *verified* but are not doable with safe code. For example: the implementation of `Clone` for `Box` is `unsafe`, because it does a direct memory copy from one pointer location to another, and *that* is `unsafe` because the caller has to make sure that the source and destination locations (a) are both legitimate to access and (b) don’t overlap!

### Calling `unsafe` functions or methods

So that covers raw pointers. What about `unsafe` functions and methods? The obvious reason we might need unsafe at the level of a function or a method is that the function cannot guarantee the safety of a given operation with just the information it needs to *execute*. That is: the function or method has guarantees *you* need to uphold – guarantees that should hopefully be very well-documented.

A prime example of this is any kind of FFI interaction! If you’re dealing with memory from outside Rust – say, because you’re calling out to C, or because you’re using Rust to speed up an Elixir computation, or so on – by definition the compiler can’t check that you’re keeping things straight. It doesn’t know about everything going on in the other language! So calling a function that is in another language is inherently unsafe!

### Interacting with mutable `static` bindings

Okay, now let’s talk about mutable `static` variables. `static`s are *global* items in Rust, and in safe Rust, they can’t be changed, because as a lot of people have said over the years, “Shared mutable state is the root of all evil” and mutable global variables are the definition of shared mutable state. A `static` variable in Rust has a specific, unchanging location in memory, and you can of course have references to `static` values in normal Rust. In fact, you *often* do: that’s what any static string is, for example! So if you could *change* them… you could have data races if multiple threads had references to that particular `static` at the same time and tried to read or write to it. Bad times would ensue.

There are times when having a single piece of global, mutable state is useful, rare though they may be. But, as usual, we need to very carefully constrain how that is handled so we don’t shoot ourselves in the foot. Thus, we can only change *or* read mutable `static` variables in `unsafe` blocks.

### Implementing `unsafe` traits

<b>Note to readers:</b> this section is incorrect; see the comments [in the show notes](/show_notes/e027/index.html#errata).

Finally, we have `unsafe` traits. These are basically just one abstraction layer up from the things I just covered with `unsafe` functions and methods and with raw pointers. If a trait has a *method* which is `unsafe`, then the trait itself also has to be `unsafe`. The prime examples for `unsafe` traits are `Send` and `Sync`, which I covered back in episode 22. If you build a custom type that uses raw pointers, the Rust compiler by definition can’t tell if you’re using it in a way that is safe to share across threads (either as a value or a reference), so you have to write the `Send` and `Sync` implementations yourself… and that is unsafe, just as you would expect.

## Safe abstractions

So those are the four things Rust lets us do with `unsafe`. But there’s more to say here, because the *most important thing* about `unsafe` in Rust is that you can constrain the unsafety. It isn’t a virus that infects everything it touches. You can (and basically always do!) wrap `unsafe` code with code that *is* safe. In fact, this is how large swaths of the Rust standard library are implemented. This ability to provide a safe abstraction around an unsafe implementation is at the core of what makes Rust viable.

As we just talked about, there are things you *cannot* do in safe Rust code, but which you *have* to be able to do to get the kinds of performance we’re looking for. The trick is making it so that the other people using your code aren’t exposed to those unsafe things. This is of course not specific to Rust! Well-written C and C++ does the same kind of thing, and of course this same basic idea is fundamental to the notion of abstraction in programming.

The difference with Rust as compared to C or C++ is that we have a tool for making the boundaries between the unsafe and the safe code explicit. Code inside an `unsafe` block is allowed to make those four moves we discussed, with all their ramifications. Code outside an `unsafe` block… isn’t. So when you write a chunk of `unsafe` code, you’re responsible to uphold the invariants required to make the safe wrapper, well, *safe*.

Here’s an analogy. One of the fundamental data structures we use all the time is a stack, where the first thing in is the last thing out. From the perspective of someone using a stack, the only thing we care about is that we have the tools we need to put more items into the stack or to get items back out of the stack.

However, there are lots of ways you can *implement* a stack while keeping those constraints for users. You could use a simple linked list. In Rust we could just have something like `Option<Box<Node>>`, where the `Box<Node>` is our dynamic pointer to the next item. Every item in the stack would have `Some(Box<Node>)`, except the first one, which would have `None`. Or we could use a `Vec` and keep track of the latest item with an index pointing to the most recently inserted location in the vector.

From the perspective of someone *using* the list, none of those details matter. We can choose the underlying data structure that gives us the best performance tradeoffs for our particular stack’s needs.

However, we have to make sure that the functions we supply for interacting with our stack do what we say that they do! In the list-based variant, we need to make sure that we only ever append to the end of the list. Otherwise we’ll end up with a tree! (Happily, Rust would actually stop us here unless we switched from `Box` to something like `Rc`, because otherwise there would be two owning pointers.) Likewise, in the `Vec`-based version, we would need to be careful to set the index correctly when adding or removing items from the end of the `Vec`. We are responsible for upholding the invariants for our abstract data structure with the concrete implementations.

This is exactly the same thing going on with `unsafe`. The abstraction around the unsafe code and the unsafe block itself are responsible to ensure that the function *does the right thing* with the data outside callers hand it. But now we’ve isolated it, at the programming language level, and we can know with confidence exactly where any bugs in the behavior of that unsafe abstraction come from. Or, to put a more positive spin on it, we can know where we need to focus our design and testing and verification efforts!

I really liked how Nerijus Arlauskas put it in a blog post (which I have of course linked in the show notes), [Rust and OpenGL from Scratch](http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html):

> It may seem strange that “unsafe” exists at all. The reason for it is quite simple: it allows us to deal with complicated stuff once, inside a function with a safe API, and then completely forget about it when we become the users of that API. In other words, it moves the responsibility of correct API usage to API implementer.

One last note here: in some languages, the default is that all items in a given module are public, and you have to go out of your way to hide them. That’s not the case in Rust – and it’s absolutely essential that it not be the case in Rust! We can (and indeed basically *must*) use privacy to make it so that we can uphold the invariants we need in a given context. Have a function you need that isn’t itself `unsafe` but which can break invariants that `unsafe` code needs upheld to make sure it works correctly? Leave it private to the module and write good comments explaining exactly how it has to be used to uphold those invariants: then the only possible places where things could go amiss are, again, scoped!

## Closing

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- James Hagans II
- Paul Naranja
- Ryan Osial
- Chip
- Bryce Johnston
- Dan Abrams
- Alexander Payne
- Bryan Stitt
- Ramon Buckland
- Jako Danar
- Nathan Sculli
- Scott Moeller
- John Rudnick
- Steffen Loan Sunde
- Matt Rudder
- Raph Levien
- Michael Mc Donnell
- Chris Palmer
- Oluseyi Sonaiya
- Daniel Collin
- Joseph Marhee
- Brian McCallister
- Nick Gideo
- Graham Wihlidal
- Nicolas Pochet
- Behnam Esfahbod
- Jonathan Knapp
- Nick Stevens
- Jerome Froelick
- Rob Tsuk
- Daniel Mason
- Adam Green
- Anthony Deschamps
- Peter Tillemans
- Martin Heuschober

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for \_this\_ episode at \<newrustacean.com/show\_notes/e025/\>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy (maybe unsafe!) coding!
