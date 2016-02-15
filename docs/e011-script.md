Once Upon a Type
================


Intro {>> 30s → 0:30 <<}
-----

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 11: Once Upon a Type*.

Updates {>> 1m 30s → 2:00 <<}
-------

The last month has been an unusual mix for the show, with a bonus episode about building community and then the two part interview with Sean Griffin, which I hope you all enjoyed! I got several requests for further interviews, and I already have a few in mind. My goal will be to release one every other month or so.

A bit of news from the Rust community: Rust 1.7 came out this week! It included a bunch of standard library improvements, as well as the warnings for some type system soundness changes that were introduced back in 1.4 becoming errors. One of the highlighted standard library changes was a pretty substantial improvement to the way hash maps in the language behave, which can result in a serious speedup in a number of scenarios. The actual change was that you can now drop in a substitute algorithm for hashing, indicated to the compiler simply by specifying a generic.

As it turns out, this leads right into our discussion for today! We're diving back into the more standard fare with an in-depth discussion of one of the things that Sean Griffin and I talked about a fair bit in our interview, and which has come up in the show incidentally from the beginning: Rust's type system, *as a type system*.


Type systems {>> 9m 30s → 11:30 <<}
------------

Before we can talk about Rust's type system in particular, though, we have to answer a much more general question: What *is* a type system? And what do we mean when we talk about the different kinds of type systems programming languages have?

We can come at this from two directions. One is in terms of the mechanics of programming and generating some runnable output form the computer. In that sense, our type system is the way a programming language has for representing data and functions to the compiler. We might distinguish between different type systems, then, in the extent to which they provide more or less information both to the computer and to the programmer. At the most basic level, there are bits---but we need a way to specify what those bits represent when we're running a program, and we need ways to tell the compiler or interpreter how it should think about those bits.

It may actually help to come at it in much more abstract terms. A paper I read, which was a distillation of an opening lecture to Ph.D. students, put it this way: a type system is "a system describing what things can be constructed" ([source](http://www.cs.ru.nl/~herman/PUBS/IntroTT-improved.pdf)).

That's helpful, and it explains what we mean when we start talking to the compiler. We are specifying *what to construct*, and *how to construct it*.

For example, we might be constructing a number or a string, or a linked list, or a hash map. We also might be constructing a complex data structure with other data structures embedded in it. But we might also be constructing operations on those data types. We might be building a function which takes in two numbers and does something with them, or one that takes in any type which is iterable and iterates over all it's elements and prints them. We might build a function which takes an inter able and a function and calls the function argument in every item of that iterable. And so one. We tell the computer what we want to construct, and how to construct it.

So type systems let us specify *intent* about our programs.

### The strong/weak and static/dynamic axes

Now, there is enormous variation in the kinds of type systems out there. We might think about two major axes of type systems: static vs. dynamic, and strong vs. weak. By "static" vs. "dynamic" what we really mean is whether the types are known at the time of some compilation step---"static"---or only at runtime---"static". By "strong" vs. weak, we mean: how easy is to to transmute one type into another? In a strong type system, you have *strong* guarantees that an item with a given type will continue to be handles as that same type. In a *weak* type system, by contrast, the language might perform implicit conversions between types.

On both of these axes, there are tradeoffs, and of course even though these two axes aren't directly connected, the *combinations* make for tradeoffs, too. Let's make this concrete in terms of languages you're probably familiar with at least in principle:

  - PHP is *dynamically* and *weakly* typed. (So is JavaScript.) Types are not known till run-time, and they can *and will* be implicitly converted. If you add an integer to a string, the language will just transform them under the covers so the operation makes sense.
  - Python is *dynamically* and *strongly* typed. Types are not known till run-time, but types are *not* implicitly converted. In the integer and string addition example, Python will throw a `TypeError` and tell you that addition isn't supported for the two different types.
  - C is *statically* and *weakly* typed. It's types are known at compile time, but it's trivial to coerce from one type to another. If you declare an integer and a character array string, and add them together, you'll get nonsense, but no compiler error. Granted: modern compilers will usually warn you about this, but that's no part of C itself.
  - Rust is *statically* and *strongly* typed. It's types are known at compile time, and there is no implicit conversion between types. You can convert between types, but you have to explicitly define the underlying transformation yourself, and explicitly call that transformation in your code, or it simply won't *compile*.

So far so good. Type systems give us the ability to specify our intent to the computer, and have that intent enforced more or less rigorously. Dynamically typed systems are often more convenient, especially for rapid development, as they let you just write the code without worrying about specifying everything about the types.

(In practice, once you're out of the rapid prototyping phase, you usually end up specifying *something* about the types, albeit usually in documentation, because if your function needs an iterable and you hand it something else, you'll get a runtime error. That's the attraction of a static type system, which can give you that information at compile time---but at the cost of making you spell everything out, which can slow you down considerably and can make rapid prototyping much harder.)

Historically, most statically typed languages forced you to declare types fully whenever declaring an instance of the type: `Integer I = 5;` or `MyType someInstance = new MyType();` and so on.

In languages descended from or inspired by ML, including Ocaml, Haskell, Swift, F^♯^, and Rust, a fair bit of the pain involved with specifying types ahead of time is removed because the compiler can often *infer* the types, and in far more sophisticated ways than you get even with C++'s `auto` or C^♯^'s `var` keywords. I'm not going to cover all the details of the Hindley-Milner type inference algorithm used by Haskell, F^♯^, and Rust---I don't understand it well enough, for one; and for another, it would take quite a while even if I did!---but suffice it to say it's enormously capable. This lets us have *strong*, *static* typing with a little bit less of the pain we might expect coming from other languages.


### The expressiveness axis

So that gives us a pretty good idea about the static-vs.-dynamic and weak-vs.-strong axes. There's another axis, though, and that's what we might call "expressiveness": what kinds of concepts is your type system capable of communicating? While Rust is only a little stronger than Java, and probably about the same strength as C^♯^, it's more *expressive* than either. It can communicate things that they can't. Likewise, Haskell's type system is even more expressive than Rust's; it has concepts like *type constructors*, or *higher-kinded types*, which let you do generic things with types themselves. Beyond Haskell, there are yet further things you can express in type systems---things like so-called "dependent types", where the type of something can even include the specific values involved. If you're not tracking with that, it's okay: suffice it to say that there is an enormous range of "expressivity" and that Rust falls toward the *more* expressive end of it.

But of course, there are tradeoffs here, too: the more your type system is capable of expressing, the more possibilities there are for getting things wrong in a variety of ways, and the more work you have to do to get the types you actually do want in place. (This is why type inference is such a big deal, in fact: if you had to fully write out every type in a language like Rust, much less Haskell, it would be incredibly annoying.)


Examples {>> 6m 15s → 18:15 <<}
--------

All fine in principle, but what do we actually mean by this? What's something you can express in Rust that you *can't* express in one of these other languages? Or even which is just much *harder* to express there?

### Enums

Here's one we've covered in substantial detail already: Rust's enumerated types allow you to express, concisely and in one place, something which is perhaps *possible* in C or Python or C^♯^---you can make something that behaves roughly like a discriminated union in any of those languages---but which is both unnatural and nearly or actually impossible to get the guarantees around that Rust gives you. This is why error-handling in those languages ranges from just returning an error code (in C) to exception-based handling in Python and C^♯^. In none of those languages is there integrated, language-level support for returning a type which is *always* either a usable result or an error, with the details of the result or the error immediately available. In none of them can you force the caller to deal with the possibility of an error there, either. This is something that is straightforward to express in Rust, and difficult to express and impossible to enforce in those languages. That doesn't make them *bad* languages; each is very good in its own way. The point is simply that Rust's type system allows you to express a *concept*---the result of some operation---in the form of a *type*---specifically, the `Result<T>` type.

### Higher-order functions

Likewise, in Rust you can tell the compiler not only that you expect a function as an argument to another function, but what *kind* of function you will take as an argument. You can say, "This function accepts functions which operate on all iterable, addable types," for example. Or, further, "This function accepts a function which itself takes a function acting on a given type as an argument." This kind of thing hurt my head at first, but it's incredibly powerful. And unlike in, say, JavaScript (where that kind of things is are also incredibly common), here we can know ahead of time whether we're meeting the required interface. We can express our intent to the compiler through the type system, and the compiler can then respond if we try to pass in a function which doesn't correspond to the relevant type requirements.

### Lifetimes

For one last example, one of the things we haven't talked about on the show yet---lifetimes---is both a part of Rust's type system *and* something that it's really nice that the compiler can usually infer for us, because they're hard. Having them available is an important part of Rust's memory-safety guarantees: knowing how long some piece of data lives lets the compiler, and specifically the borrow checker, make the right decisions about whether a given piece of data is available or not. But if we had to specify the lifetime of every single type, that would make our code much noisier, and much harder to write. Not just because it would involve more typing, but because we'd have to be *thinking* about it constantly. (I'd go so far as to say that if we had to write lifetimes everywhere, the benefit we get from tracking them wouldn't be worth the cost of using them in the vast majority of programs.)

But lifetimes are also something you can express in Rust that you literally cannot express in many other languages. They *exist* in other languages, but that doesn't mean you can inform the compiler of your intent about them. This is directly analogous to the way that arguments to functions *exist* in Python: at run-time, some argument will be a string, or an integer, or some type you've constructed, but you have no way to tell Python that the type *must* be a string, or an integer, or some other type you've constructed. In a language like C++, you even have to reason about lifetimes, because like Rust, it does some deallocation of data behind the scenes for you when it can. But in Rust, you basically never have to call a destructor explicitly in code that *uses* a type, because all of those details are handled by lifetimes---and you can specify the lifetimes yourself as necessary, and the compiler will tell you when there's something wrong with the lifetimes.

Don't get me wrong: as we'll talk about within the next few episodes, reasoning about, and informing the compiler about, lifetimes is still hard. But in Rust, we *can* tell the compiler about it. And any given piece of data in the language has a lifetime associated with it. So we can now *express* in the types themselves something we have to deal with in C++ separate from the type system (by calling destructors, and so on).


Conclusion {>> 2m 30s → 20:45 <<}
----------

Coming back around to our opening, what does this have to do with the `HashMap` changes? Well, simply put, the *type* of a HashMap implementation is generic; so you can specify in the type system itself how a HashMap behaves, as long as you supply it a hashing algorithm that conforms to the specified bounds on that generic. If you write a new hashing algorithm tomorrow, as long as it meets those bounds, you can drop it in and go and *know* that it will work---barring logic bugs, etc.; but you won't get *run-time* errors because of missing items in types, functions not fulfilling contracts, etc. That's powerful!

In the next episode we'll take a look at another interesting dynamic in Rust: that it is an expression-oriented language. That episode will include a fair number of specific code examples, as well, because it's much easier to illustrate in practice than *type systems in general*.

### Sponsors

Thanks to Hamza Sheikh and Chris Palmer for sponsoring the show this month!
You can see a full list of sponsors in the show notes.

If *you'd* like to sponsor the show, you can set up recurring contributions at
Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, or cash.me.

### Follow/support

You can find links to each of those, to other things mentioned on the show, and
notes and detailed code samples illustrating macros at NewRustacean.com. You can
also follow the show on Twitter @newrustacean, or follow me there @chriskrycho.
You can help *others* find the show by rating and reviewing it on iTunes,
recommending it in another podcast directory, tweeting about it, or just telling
a friend!

Thanks to everyone who's added corrections and fixed typos in the show notes;
pull requests are great. I also love hearing from you! Definitely say hi on
social media, add your thoughts in the thread for the episode on the Rust user
forum at users.rust-lang.org, or shoot me an email at hello@newrustacean.com.

Until next time, happy coding!
