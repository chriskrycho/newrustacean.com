# Crates You Should Know: Rocket

## Intro

Hello, my name is Chris Krycho, and this is the New Rustacean podcast – a show about learning the Rust programming language. This is Crates You Should Know: Rocket.

## Overview

As someone who works all day every day in the world of web development, I have to admit that as *fascinating* as many of the low-level projects in the Rust space are, and as incredibly *important* as many of the projects are for things like the reliability and safety of our software, this is one of the ones that makes me most *personally excited*.

Rocket is a full-fledged web framework for Rust – probably the most fully-baked of the handful of actual web frameworks in Rust out there right now. One important thing to understand is that I’m distinguishing between a *web framework* like this and the underlying (and essential) tools like Hyper.

Another important thing to understand is where Rocket is in the space – also by way of contrast, with Iron. (I may come back to Iron at some point, as well as some of the other fledgling frameworks.) Iron is a “middleware-oriented server framework” – which requires you to write yourself or reuse sets of middleware to build up your responses to each request. Rocket is much more in the general vein of a Django or a Rails, in the sense that it aims to give you many more of the pieces you need for many web applications right out of the box. There’s room and indeed *need* for both of those approaches to building web applications in this space, and I want to see both approaches thrive. At this particular moment in Rust’s history, though, I think the approachability factor for Rocket is really important.

### The Why

Here’s why: being able to build your own middleware stack is incredibly powerful in one way. It lets you control every part of your stack, and carefully control every detail of your performance. However, it has the downside of, well, requiring you to manage every one of those details. To be sure, I think managing every last detail is probably more tempting for Rustaceans than for the average developer, but one of the things the Rust ecosystem needs is places where it is easy to get *started* – not everyone needs to be concerned with *every* detail everywhere.

Put another way: sometimes you want to drill down that far, but other times, a nice abstraction goes a long way. And if it can be a really low or best of all zero cost abstraction, that’s a huge win. That is, in fact, one of the biggest bets Rust is making: that we can have *both* high-level abstractions and low-level performance. And Rocket is an attempt to provide a high-level abstraction for building the server side of a web application.

### The How

When I say it does this while maintaining low-level guarantees, I mean it. But if you’ve spent time writing a high-level language and framework – like the aforementioned Django and Rails – you’re probably wondering how it can possibly be even in the same ballpark as those while also maintaining the speedy and low-level performance we’ve all come to expect from Rust. The answer, basically, is *magic*—where by "magic" I actually mean "procedural macros”.

Procedural macros are a piece of the Rust story I haven’t covered yet, not least because they are (frankly) still not something I’m comfortable with. And in truth, most normal Rust developers don’t need to be deeply familiar with them, but they’re incredibly powerful and as such they’re profoundly useful tools for building type-safe but ergonomic abstractions like these. They’re also still unstable, which means that Rocket only works with nightly Rust for today.

Procedural macros are a way of doing a full transformation of the syntax made available to you, at a much more sophisticated way than normal Rust macros support—but with the attendant complexity as well. You’re basically writing compiler plugins when you write a procedural macro, and that means that you’re responsible to make sure you are *correct* in the transformations you write. However, that power lets Rocket supply totally custom attributes, and those totally custom attributes are what make for its friendly and accessible API.

I’ll come back to procedural macros in more detail in the future, both to talk about their future in light of RFC #1566 and to talk about how you can use them if you like... but all of that after I’ve written at least one myself!

### The API

Rocket's API is primarily built around a combination of traits to implement, a set of custom annotation-type procedural macros, and a number of convenience macros of the macro-by-example variety.

#### Important traits

There are a number of high-level traits which your types can implement, and some of which they *have* to implement before they can be used with Rocket.

- `Responder` is the basic trait which all types which you use as a *response* to a request must implement.
- `FromData` lets you define how to transform some incoming data into your application's types.
- `FromParam` lets you turn dynamic path segments in your route matchers into well-typed arguments to your route-handling functions.
- `FromSegments` does the same basic thing but with *multiple* path segments, not just one.
- `FromForm` takes HTTP form data and turns it into a type.
- `FromFormValue` does the same thing with individual fields of a form.
- `FromRequest` lets you transform incoming request *metadata*. This lets you do things like require API keys in a header, to borrow just one example.
- `IntoOutcome` lets you define how to convert some type into Rocket's `Outcome` enum type, which is its way of representing the *three* states any given request can have: success, failure, or forward-to-the-next-possible-handler. (The `Outcome` type is kind of like a `Result`, but for HTTP request handlers, where a given handler may be able to *know* that something is a success or failure, but it also may *not* know that and should just pass it on).

Not a trait but incredibly important is the `Request` struct, which defines an inbound request – but which you're not meant to use directly for the most part. Instead, you should usually use one or more of those traits methods to convert *from* `Request`s into your own local types.

There are lots of other concrete struct and enum types you'll care about in building a Rocket application, of course, but those traits form the underpinning for the entire system, when combined with the procedural macros – that is, the custom annotations which allow you to have all the nice type-safe code which gets code-transformed into all the extra handling pieces you need.

#### Attributes

The most important of those procedural macros are the custom routing attributes. These are HTTP verb macros: attributes for defining that a function handles a request of a given HTTP request type at a given path, including any associated data from (for example) query parameters or the request body. The idea of using these kinds of attributes to define how a given function, well, *functions* isn’t an invention of Rocket, of course: with plenty of variation in the implementation details, it’s the basic pattern for everything from Python’s Flask framework to  C♯’s ASP.NET MVC handle routing definitions.

Rocket is smart to adopt that same basic approach – but it doesn’t just adopt it unchanged. Because procedural macros operate at compile time, it can do things and provide guarantees unavailable with decorators in Python or attributes in C♯, both of which are ultimately *runtime* items (though the pieces you use to *define* C♯ attributes are compiled much as Rocket's own code generation is). That means that Rocket can catch all sort of errors which would show up in runtime in other languages at compile time – a non-trivial win, if you ask me. This is one of the huge advantages of having a formal metaprogramming capability built into the language, and this kind of capability is a big part of why stabilizing the procedural macro system is a major and ongoing goal.

The custom attributes Rocket defines are:

- `route`
- `get`
- `put`
- `post`
- `delete`
- `head`
- `patch`
- `options`
- `error`

Most of those are very nearly self-explanatory if you've done any web programming before: `get`, `put`, `post`, `delete`, `head`, `patch`, and `options` all just map directly to the standard HTTP methods, and define a route corresponding to that verb. The `route` attribute is something like a long-hand attribute for each of those: you can define a `route` which takes a specific HTTP verb method as its first argument and get the same result as using the verb shorthand. Each of them takes arguments appropriate to its type, including the ability to handle request body data, query parameters, and so on. Any arguments specified in the attribute are then type-checked – routes won’t match if the types don’t align. You can overload paths by providing a further argument, `rank`, which specifies the order in which to attempt the match.

The last of these attributes, `error`, is a special case: it defines a class of handlers Rocket calls “error catchers.” These are just like the route handlers, but they only deal with error conditions – on a per error code basis.

#### All together

Once you have defined your types, defined the appropriate trait method implementations for them, and defined the routes, you just set up Rocket itself in your main function: you call `rocket::ignite().mount().catch().launch()`, where the `.mount` method takes a path to mount and a list of functions in a `routes!()` macro, and `catch()` does likewise with a list of error handlers supplied to an `errors!()` macro. It’s a very nice to use API: Rustic, just a little bit clever, and very efficient. I hope to be able to *use* it on a production app at some point, because I like it so much.

### Conclusion

Hopefully that gives you some idea of how you’d tackle writing a simple application in a Rocket! I encourage you to take a look at the website for many, many more details on how all of this works—as well as to see a *marvelous* example of technical documentation. In addition to the Crate docs, there is also a great overview section and a really helpful guide section on the website proper. There are many technical details that make Rocket more immediately approachable than Iron – and again, that's not a dig on Iron, just an evaluation of what they're trying to be! – but more important than any of those technical details is the fact that Rocket has one of the best websites I've seen for *any* framework like this. If you're building a framework or library that you want to be adopted, I strongly encourage you to find a way to build documentation around it that is this thorough and well-considered – and to make it *look* good, even if that's just by making heavy use of Bootstrap or something like it. It really does make your tool more attractive. In this case, that's icing on the cake of a really interesting framework in its own right. You should *definitely* check out Rocket if you're looking to build something in the web world in Rust!

## Outro and sponsors

Thanks to:

- Anthony Deschamps
- Behnam Esfahbod
- Christopher Giffard
- Chris Palmer
- Dan Abrams
- Daniel Collin
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

for sponsoring the show this month. If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website, or if you're a company interested in advertising to developers, get in touch!

The other *huge* way people support this show is by helping others find it – by linking it on social media, sharing it directly with a friend, or even just rating or recommending it in a podcast directory. Thank you to everyone who has done that over the past couple years!

### Info

Show notes for this episode are available not only in the podcast player of your choice but also at NewRustacean.com. You can follow me on Twitter @chriskrycho, or follow the show for retweets of Rust news @newrustacean – and if you have Rust news, feel free to tweet it at me and I’ll happily retweet it! That’s also one great place for giving feedback – others include the threads on Reddit, Hacker News, and the Rust user forums, as well as good old-fashioned email at hello@newrustacean.com. I’d love to hear from you!

Until next time, happy coding.
