# Point me where I need to go

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 17: Point me where I need to go.*

## News and follow-up

- Raph Levien posted an interesting post on Medium looking at the font renderer he mentioned briefly in my interview with him. It's crazy fast, and the post is pretty interesting.
- Aaron Turon, one of the Rust core team members, wrote up a proposal for a "Rust Platform" to help make it easier for new Rustaceans to get up and going---trying to get some of the benefits of the "batteries included" standard library of something like Python, while still keeping the lean, light *actual* standard library. There's been a lot of interesting discussion about that, so I'll link it in the show notes!
- Nightly Cargo builds now support the ability to "vendor" dependencies, that is, specify a source for them besides their source repository---which is helpful for e.g. builds in environments where going out and hitting GitHub isn't possible, or where the dependencies all need to be verified ahead of time.
- Nightly Rust now has MIR turned on by default! It was previously available, but only behind a feature flag. We're quickly headed toward a lot of compiler improvements, as it looks like it should land sometime in the next few major releases. (For more details on what MIR is, you can go back and listen to news episode 1!)

## Referencing and Dereferencing

One listener, \@gazhaze on Twitter, asked if I'd covered the dereference operator or the general behavior of dereferencing, and I realized I hadn't. In all the code samples I've given, I've basically assumed the syntax there, but it's worth slowing down and thinking about what we mean when we use a given piece of syntax, and how that fits into the language's model for safety. It's also important for understanding a lot of performance considerations! And given that the original plan was to spend this episode talking about some traits and types which make heavy use of the semantics around referencing and dereferencing things in Rust---but to do *that*, we need to talk about what we mean by "references" and "values" and "pointers" and "dereferencing" in the first place.

Most modern languages (from C to F^♯^) have some idea of dealing with data either by *value* or by *reference*. And sometimes it *matters*. I once ran into a problem in a piece of Python I'd written where something was going wrong precisely because a given bit of data was being passed around by reference, not by value, and until I realized that, I couldn't solve the problem---even though Python does all of that "behind the scenes," so to speak. As such, I hope this helps you specifically with Rust, but understanding these ideas should help you whatever language you're writing in.

### References and pointers

So when we talk about references and pointers, what *exactly* are we talking about? If you're coming to Rust from a language like Python or Ruby or JavaScript, and you don't have a background with lower-level languages, you've probably heard the words batted around, and you may even have gotten to the point where you know how to use the relevant operators (which we'll discuss further in a minute). But if you're still wondering what they are, or if you just need a refresher, hopefully this will help.

When you're dealing with any piece of data in a program you're writing, it has a particular location in memory. That location is just an *address*, usually written in hexadecimal, but ultimately just a number, that ultimately maps down to a specific location in the RAM where the computer is currently storing that data. That's true whether you're talking about something as simple as an integer or something as complex as a vector of structs with refcells pointing to strings in them. *Everything* has an address. (One thing to note as we go: the program doesn't actually have a *direct* address to the RAM, but rather an address relative to its own block of memory as allocated by the operating system. The operating system maps that address back to the RAM itself; that's an important distinction for security purposes.)

When you call a function with some data, there are two ways you could give it that data. Since every function gets its own memory on the *stack*, we could just allocate the required amount of memory in the stack and just copy the data into that new space. Then we could just pass the data directly to the function and be done. This is simple and straightforward, and it's also *always* perfectly safe, because the original memory location is untouched. The new function can't accidentally smash it. (We'll come back to that concept a *lot* more in the next episode.)

But there are downsides to doing it that way. Copying all the data for every argument every time we call a function will get expensive. What's more, if we wanted to *change* the value in the original, we'd need to copy it back (by returning it) when we were done, and then overwrite the original memory with the new value. That's a *lot* of work. We can run into performance problems pretty quickly.

So, remembering that every item has an address in memory, we can do something which is a lot faster *and* which uses a lot less memory: we can just hand around that address instead. When we do that, we're handing around a *reference* to the data. If we know the address and the kind of thing we're talking about---a struct, or an enum, or whatever else---then we know how big it is, and we know how it's shaped, and we don't actually have to copy the data to be able to use it.

We do have some concerns about safety doing that, though. If we have two functions running in parallel, and they both have the address of a given data structure, what happens if they try to write to it at the same time? Or even, what if one of them tries to read from it while the other one is writing to it? Or what if you just make a mistake and write the wrong size object to a given address---one that's too big for the space, and so goes past the end of it and crashes into some *other* object's memory? Because, remember: at the end of the day our data is all just values stored at specific addresses in memory. So you can easily end up with bad data, and have no way of knowing. We'll come back to those safety issues in just a moment.

First, though, let's talk about what we mean by "pointers" now that we have a good idea what references are. A pointer, strictly speaking, is the content of a *variable* whose type is a *reference*. Instead of the content of the type being an integer or string or whatever else, the content is the *address* of an integer or string or whatever else.

So, when you pass an argument *by reference* to a function in Rust, the type of the argument is a *reference* and its content is a *pointer*, a number pointing to a specific address in memory. However, the vast majority of the time (and *all* of the time in safe Rust), you don't have to think about the value of that address---just the kind of thing it's pointing to, and the fact that you have a reference instead of a value. When you perform operations with a reference, the value of the memory address (that is, the pointer value) is both irrelevant to you and, in general, inaccessible to you.

This is very different from C or C++, and it's precisely where Rust's idea of *borrowing* and *ownership* comes from. Rust keeps us from making the kinds of mistakes we talked about above in a couple ways. The first, and most interesting, is by making sure there is only one mutable reference to data at a time. What that *really* means, in light of everything we've just said, is that Rust makes a strict compile-time guarantee that only one scope is allowed to have *write* access to given memory address at any given time. But an unlimited number of scopes can have simultaneous *read* access to an address as long as nothing has *write* access to it: we know the data at that address won't change.

When we talk about *lending* or *borrowing* a reference, then, we're really talking about handing around pointers to data, with strict rules about whether the scope getting a reference is allowed to write to that address or not.

As for smashing the memory by writing the wrong kind of data to a given address, this is where Rust's type system and its rules about unsafe code come in. C and C++ won't let you write the wrong kind of object to a given location... unless you just coerce it to the "correct" type, or cast it to arbitrary memory `void *`, both of which are bad practice but totally allowed by the compiler. Both of those are actually impossible in safe Rust, which requires you to define safe, explicit transformations from one type to another---and, just as importantly, Rust doesn't actually expose the *pointer value* in the reference. Instead, it just lets you read and write to the data behind the reference by *dereferencing* it---but without ever needing to know the memory address yourself. (You can get the memory address, and manipulate it directly, in unsafe Rust, and there are times you do need to do that---but it's not *most* of the time, even for high-performance code, because the primitives Rust supplies for things like iteration, for example, are both safe and very performant.)

One other important thing to understand: I've been using a simplified version of how pass-by-value works, to keep it easier to understand. In reality, modern compilers (for a variety of languages) are generally smart enough *not* to actively I copy the memory every time you pass by value if it isn't necessary. Thus, when you "move" an object's ownership, we actually *don't* have to copy the memory every time. If the LLVM internals of the Rust compiler can see instead that the data ownership has changed in ways it understands, it can often build out the final program so that those bits of data *aren't* copied. There's a lot more we could say there, but for now I just wanted to make clear that moving data from one owner to another in Rust isn't always as costly as it might seem if you think it's always doing full copies. (As is usually the case, when you're optimizing, you should *check* for what uses large amounts of memory or takes a lot of time, rather than assuming!)

## Using References

Now that we have a decent idea what's going on with references and pointers, let's look at the syntax for using them in Rust. There are 3 basic operations we're interested in:

- declaring that a type is a reference
- making a reference from a given piece of data
- getting at the data behind a reference

The first two of these operations use the *reference* operator, and the last one uses the *dereference* operator. We'll tackle them in that order.

### The reference operator (`&`)

First: we denote a reference with the ampersand character. When we declare  a reference, we use the `&` in front of the name of the type it's referencing. So when you have a type like `DataStore` and you see `&DataStore`, you can read that *type signature* as "a reference to a `DataStore`". Likewise, you read `&mut DataStore` as "a mutable reference to a `DataStore`".

Second, we specify to pass a given value *as* a reference by using the `&` operator on the name of the given piece of data. So if we had written `let some_store = DataStore::new()`, we might think about a function `use_data_store()` which takes a reference to a `DataStore`, defined just like we said above. To pass a reference, we would write `read_from_data_store(&some_store)`. You can read that as "call `read_from_data_store()` with a reference to `some_store`". If we were passing a mutable reference, for example if we had an `update_data_store()` function, we would call it like `update_data_store(&mut some_store)`. You would read that like "call `update_data_store()` with a mutable reference to `some_store`". And of course, remember that you can only pass mutable references to mutable data!

### The dereference operator (`*`)

Now, once we *have* a reference to some data, because the content of that reference is the pointer, rather than the data itself, we need a way to get back to the data. That way is the *dereference* operator, which is called "dereference" because it takes a reference type and, well.. *de*-referencifies it, if you'll allow me to make up a silly word. We denote the *dereference* operation with the star character. Writing `*` in front of some piece of data gives you access to the original data, and then we have something with type of the original kind of thing being referenced.

Let's go back to our example with the `DataStore` type, and imagine that it was a struct with a field called `contents`. So in our example with a data store, inside `read_data_store()` we might do something like check a value `some_data` by writing `if some_value == *some_store.contents`. If we tried to do this comparison *without* the `*` operator, it would fail to compile and we would see a type error, because we would be trying to act on the *reference* to this `DataStore`, and the reference (whose content is just a pointer) doesn't actually *have* a `contents` field. But when we dereference it, we get back the actual `DataStore` type, which does have a `contents` field, and then we can make the comparison.

For a somewhat more complicated example of using both with an enum and pattern matching, see the show notes.

## Outro -- Sponsors and Following

And that's it! Once we understand the concepts behind them, the reference and dereference operators themselves are pretty simple. And that gives us the *background* we'll need to tackle the `Borrow` and `AsRef` traits, which will let us generalize these ideas to our smart pointer types---and that in turn will let us look at the `Cow` smart pointer type.

Thanks as always to the sponsors who help make the show possible! This month, the sponsors included Chris Palmer, Daniel Collin, Eric Fulmer, Jared Smith, Raph Levien, Stephen Murawksi, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes, and the top tier of sponsors on a dedicated page on the website as well.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at a variety of other services listed at the show website: newrustacean.com.

There you will also find links to the news items from the top of the show, and code samples illustrating referencing and dereferencing data.

You can also follow the show on Twitter \@newrustacean, or follow me there \@chriskrycho. If you enjoyed the show, please help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, sharing it online, or just telling a friend!

Also, please do respond on social media, in the threads for the episode on the Rust user forum or on Reddit, or via email at hello@newrustacean.com. I love hearing from you!

Until next time, happy coding!
