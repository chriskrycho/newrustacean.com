# Not Dumb Pointers

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 15: Not Dumb Pointers.*

## News and Follow-Up

- First, note that in short order after the release of this episode, the tickets for RustConf will go on sale. RustConf is September 10th in Portland. Speakers should be announced fairly soon!
- Second, if you're on the *other* coast of the United States, the Rust Belt Rust Conference is October 27th and 28th in Pittsburg, Pennsylvania. The speakers and topics have already been announced, and it looks like a pretty great conference.

I'll let you know of more conferences as they come up. You should also keep an eye out for local meetups---or think about starting one!

I mentioned in the last episode that I was curious about other podcasts. There *is* one other one out there: Rusty Radio. I knew about Rusty Radio, but I thought they'd gone off the air. I'm happy to find that they haven't, and you can find them on SoundCloud (and of course I'll link them in my show notes).

Also, I want to remind you about Exercism. I mentioned it a long time ago but just started playing with it myself: if you're just starting out with Rust, it has a series of problems ranging from "Hello, world!" to "Write a parser for part of the Forth programming language"---with built-in test suites to help you through it. It's *really* good stuff.

One other big thing: since the last time I recorded, I actually wrote an RFC! As you may recall, RFCs are the official way changes make it into Rust. In this case, I made an argument for requiring documentation to accompany all new changes to the language. You can check it out at rust-lang.org/rfcs/pull/1636.

## Smart Pointer types

Today we're going to talk about a number of kinds of *pointers* in Rust. This will pull together things we've talked about going all the way back to episode 2 and our summary of ownership and borrowing, as well as pieces like the `Box` and `Vec` types. We'll also look at some closely related types like `Rc` and `Arc` and see when and how we might use them, and what some of the constraints on using them might be. This is the kind of episode where there's a lot going on, and it depends heavily on other things in Rust---but it also means we get to see how the pieces *come together* to give us some powerful tools.

### Review: ownership, the stack and the heap, and lifetimes

Let's start by briefly reviewing three topics: ownership, the stack vs. heap distinction, and lifetimes. If you start to feel in over your head, don't worry; there's a lot here. It may help to go back and listen to episodes 2, 5, and 13, which covered ownership, the stack and the heap, and lifetimes.

First, think back to our discussion of Rust's approach to *owning* data. The basic idea in Rust is that we're going to keep our code safe from things like dangling pointers or bad memory accesses by tracking the *ownership* of every item in the program. One function can *lend* another function a *reference* to a given piece of data, while holding onto the *ownership* itself. Or it can *move* the ownership of that piece of data over to the other function. No piece of data is *destroyed* via a destructor until the function which *owns* it goes out of scope. Once the owning function *does* go out of scope, the data gets cleaned up automatically.

Second, Rust (like all low-level languages and many high-level languages) can allocate memory in two places: the *stack* or the *heap*. The stack is where all data which is purely internal to functions normally goes. When you write `let x = 10;` inside a function, it's on the stack. However, sometimes we need data allocated in different ways---for example, because it needs to live beyond the scope of that function, or because we need to be able to resize it arbitrarily (like a `Vec`). For those scenarios, we can allocate things on the *heap*. We've talked about a number of heap-allocated types so far: `Box`, `Vec`, and `String` are all heap-allocated types. In a few minutes, we'll also look at two more: `Rc` and `Arc`.

Third, *lifetimes* are Rust's way of making it clear how long a *reference*---when one function *borrows* a piece of data from somewhere else---has to be valid. This becomes very important when talking about these heap-allocated, smart pointer types, because, well... they're all pointers. Now, you *don't* have to write a lifetime on every `Box` reference. But things like `Box` or `Rc` can *contain* references, and when they do those internal references need to use the same lifetime specifies that you would need in any other context where a lifetime would apply, like function signatures or type definitions.

### `Box`

Okay, so we've gotten back up to speed on the pieces we need. Let's start by talking again about `Box`, because we'll build on it with a bunch of the other types.

Imagine you're writing a program and have an open reference to a file, and you want to be able to read from it any number of places. (Note that I'm not suggesting this particular approach is a great idea; I'm just giving a relatively easy-to-understand example.) You might have a type called `struct FileData`, with some metadata about the file as well as its contents. But if you included its contents, it could end up being a large file, and then you might not want to be passing around the whole type all the time. In that case, it might make sense to create it and return it on the heap, by creating a `Box<FileData>`. Now you have a *smart pointer* to the connection instance.

So why do we call these *smart* pointers? Well, because the pointers we're used to---pointers from C, specifically---are *dumb*. *All* they do is point to things. The semantics of a dumb pointer in C are very, very simple: they point at things. But that means that any and all memory management involving pointers is *of necessity* a job for the developer. The compiler doesn't handle *any* of it for you with heap-allocated objects. Better watch your `malloc` and `free` calls. If you mess them up, you're quite possibly in trouble.

Smart pointers, by contrast, have "smarts" to them. When you heap-allocate using `Box<T>`, it has all the same compiler-level cleanup intelligence as any other type in Rust. (And to be clear, other languages have smart pointers, like C++'s `unique_ptr` and `shared_ptr` types. But Rust's have better safety guarantees.) When a smart pointer type goes out of scope in Rust, it automatically gets cleaned up, and so does what it points to. So if you have a `Box` pointing to some data, and it goes out of scope, the pointer itself gets destroyed, and then any cleanup code associated with the type it was pointing to runs for *that* data.

Going back to our example where we wrapped our `struct FileData ` in a pointer so we could hand it around. If you've dealt with SQL connections before, you know there's often some cleanup involved: you need to release the connection cleanly so it gets returned to a connection pool, so that other parts of the program or other programs can use it. In Rust, we do this by implementing the `Drop` trait on a given struct and defining a `drop` method which runs when something goes out of scope. (If you're feeling *Rusty* on traits, you can go back and listen to episodes 8 and 9.) So in this case, our `Box<FileData>` would get rid of the heap-allocated data, but first it would run whatever code was in the struct's `drop` method. The pointer is *smart*.

This is the same basic principle that makes types like `Vec` or `String` work: internally, they just employ their own smart pointers (though in those cases, if you get all the way down, it's the still-unstable `Unique` type, not `Box`).

### Reference-counting

So that's a nice combination of increased convenience and safety over C-style dumb pointers on its own. But where this gets really good is when dealing with, say, types that need to cross threads.

If you have some data you need to pass from one thread to another, and you know that the data is immutable, you're fine: you can just "move" the data to the other thread. And in many cases in Rust, that's actually exactly what you'd do. Passing things around behind smart pointers all the time is actually an anti-pattern: this isn't Python or C#, and we don't have a garbage collector cleaning up heap-allocated memory all the time. But there *are* times you need smart pointers, and times when you need more than even a `Box` gives you.

For example, if you needed to *share* the data between two threads---that `FileData ` instance we talked about before, maybe---how would you know when it was safe to get rid of it? If it went out of scope in one thread, and the `drop` ran, but another thread was still pointing to it, you'd be in the same familiar mess everyone who's tried to write thread-safe code in C has found themselves in. And in that case, it wouldn't matter whether the data was mutable or immutable. Like I said: a mess. And of course, if you think about it, if you start sharing around references to objects, you can run into that same problem even just within one thread if you have *any* asynchrony at all.

But gladly, smart people have come up with some other *smart* pointer types that can help us with this problem. For thread-local, immutable data, we have the `Rc`, or *reference-counted* type. This lets us have more than one reference to the same data when we need it. The `Rc` type does just what the name says: it *counts* the number of references to a given item. You can't mutate the item, but you can pass around references to it as much as you like within a single thread. Whenever you need another reference to it, you get it by calling the `clone()` method on the `Rc`. So, back to our example of a `FileData`, if we had an instance named `ffd`, we'd wrap it in a reference counting pointer by writing `let wrapped_data = Rc::new(fd);`. Then when we wanted to get a copy of the pointer to let other pieces of the program use it, we'd write `wrapped_date.clone()`.

Under the covers, the `Rc` has a count of *strong* pointers to the data it wraps. Any time any pointer reference is created, the count goes up. Any time one of those pointers goes out of scope, the count goes back down. As long as there is at least one *strong* pointer, the underlying data *cannot* be deallocated. In addition to strong pointers, there can also be any number of *weak* pointers. Weak pointers don't contribute to the count. They're a way of expressing, in the type system, that the thing they point to doesn't *have* to exist. You can think of them like `Option<T>`, but for reference counts---and in fact, when you try to access them, an `Option` is exactly what you get back. So how would you *make* a weak pointer? Normally, by "downgrading" an existing `Rc` instance. In our ongoing example, where `wrapped_data` was the `Rc` we made a minute ago, you might write `let weak_fd_ref = Rc::downgrade(wrapped_data)`. Then to get back at the data, you can call `weak_fd_ref.upgrade()`, which will give you the `Option` I promised a minute ago. If you're wondering when exactly you'd need this, well, suffice it to say there are a lot of circumstances they'd apply in, and we'll come back to that at some point in the future. For today it's just enough to know they exist, and how to make one.

Now, everything we just talked about is grand and all... but `Rc` instances are only allowed within a single thread. But of course there are times we need to share data *between* threads. For that very purpose, we also have the `Arc`, or *atomically reference-counted*, type. *Atomic* types in general are types that (and here I'm quoting the Rust API reference), "when used correctly, synchronize updates between threads." That means that when we use them correctly, they're a *safe* way to share data between threads. Like `Rc`, `Arc` lets you pass around *references* instead of *copies*, while maintaining safety guarantees. The API is basically the same, too: you make them with `Arc::new()`, downgrade them with `Arc::downgrade()` and upgrade their corresponding `Weak` types with `upgrade()`. What's different is that, because you need stronger guarantees about behavior, there is additional machinery for making sure that the operations are *atomic* so the threads stay in sync.

With both `Rc` and `Arc`, you can use the `make_mut()` method to *always* get a mutable reference to the data as long as the wrapper is mutable---even if there are outstanding references. How? By *copying* the data if necessary, an approach called *copy-on-write*, where you don't make copies of data unless you're *changing it*. If you don't need to change it, you can use their `get_mut()` methods instead---they just return `Option`, so if there are other outstanding references, you'll get back `None`, because you're not allowed to mutate it in that case.

## Outro

This behavior, and in fact everything we've just talked through, is a result of two of Rust's core philosophies: *zero-cost abstractions* and *only pay for what you use*. It's not that doing something like reference counting is *free*; it's that you have to pay that cost no matter what if you're doing atomic operations, and Rust doesn't make you pay any *extra* cost beyond what you would write by hand to get the same safety and performance.

Anyway, hopefully you have a better idea of how the smart pointer types work in Rust now, and hopefully you'll feel more comfortable using them when you need to (and understanding what's going on when you see them in other people's code). I know my own understanding got a lot clearer as I prepared!

### Sponsors

Today in particular, I'd like to highlight the kind people sponsoring the show. If you noticed an increase in the quality of the audio today, it's because of the generosity of the listeners who've steadily contributed to the show over the past many months. I was able to buy a top-notch recording setup through that funding, and it's now paying off for all of you listeners as my audio quality should be much improved from here on out.

So: thanks *very* much to Chris Palmer, Daniel Collin, Raph Levien, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, Flattr, PayPal.me, or cash.me, or get in touch with me directly.

### Follow/support

You can find links to those as well as some examples of using smart pointers at NewRustacean.com. You can also follow the show on Twitter @newrustacean, or follow me there @chriskrycho. You can help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, tweeting about it, or just telling a friend!

So do respond on social media, in the thread for the episode on the Rust user forum at users.rust-lang.org, or via email at hello@newrustacean.com.

Until next time, happy coding!
