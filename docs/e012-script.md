I'm not familiar with that expression
=====================================

Intro {>> 15s → 0:15 <<}
-----

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 12: I'm not familiar with that expression.*


Updates {>> 2m 20s → 2:35 <<}
-------

A few pieces of news from around the Rust community to kick things off:

A project which has been going for a while but that just came to my attention this week is the *Redox* operating system. It's a Unix-like system which aims for broad but not total compatibility with Linux... and it's built entirely in Rust. I'm hoping to give it a whirl sometime this week and see how it goes, and the project is actively looking for contributors, so if that sounds interesting, you might check it out as well!

Also of note, Mozilla announced this week that Servo, their research web renderer built entirely in Rust, will be shipping an alpha of a full web browser using the engine in June. That's *extremely* interesting because of some of the associated performance implications: the engine is written in Rust, and so it's simultaneously safe, fast, and parallelized. I'll link a video showing some pretty incredible GPU-powered web rendering the team has been doing, and which is, wonderfully enough, totally safe and also highly parallelized because it's written in Rust!

Last but not least, Wired had a fascinating look at ongoing changes with Dropbox's cloud infrastructure this week, and one of the revelations was that the Dropbox team, which has made extensive use of Go and Python, now also uses Rust in their stack. As it turns out, Rust's combination of safety, performance, and low memory footprint made it a perfect choice for some of their most absolutely demanding software as they built their own cloud. It's a great read, so I'll link the article. It's fun to see Rust picking up more and more traction as we head on toward the one-year mark from the 1.0 release of the language!

Now, into the meat of the show!


An expression-oriented language
-------------------------------

### Overview {>> 1m 10s → 3:45 <<}

One of the points I've frequently alluded to over the course of the show so far is that Rust is an *expression*-oriented language. Most of the other languages Rust superficially resembles---C, Java, Python, C++, C^♯^, PHP, JavaScript etc.---have in common that they are basically *statement*-oriented languages. Each of those languages, whatever its other strengths and weaknesses, treats *statements* as their base unit, so to speak---even when you add in substantial sophistication around objects, for example, you are still proceeding primarily by way of saying, "Do A. Then do B. Then do C," and so on. An unterminated expression is a compiler error in most cases in those languages---don't forget your semicolons!

Rust, like many functional languages, treats expressions as a more core building block. To be sure, we have statements at our disposal. But as we'll see, in every corner of the language, expressions come to the fore.  Let's look at three major examples of how this plays out in Rust: `if` expressions, `match` expressions, and `fn` expressions.

### `if` expressions {>> 3m 30s → 7m 15s <<}

Many languages you may be familiar with have ternary expressions. In languages with C-like syntax, these are denoted by an expression that evaluates to a boolean, followed by a question mark, an expression whose value used if the boolean is true, a colon, and an expression whose value is used if the boolean is false. In Python, you have the expression for the true scenario, followed by *if* and the condition to evaluate, followed by *then* and the expression for if the condition evaluates to false. These are handy because they let you directly assign the result of the expression to a variable or constant. If you use a normal if statement in these languages, you have to assign the value within the body of that if statement and any alternative else or else-if clauses. With a ternary, you can skip that step, which is especially nice when dealing with languages where you have to declare the variable before assigning to it, as in C, Java, C^♯^, and so on.

Rust doesn't have a special ternary operator or structure. It did, once upon a time, but it was removed. Why? Because it was completely redundant.

*All* `if` blocks in Rust are expressions. An unterminated final expression in the block is treated as the value of the whole block, and may be assigned directly to a name. You might write `let foo = if bar { 42 } else { 3.141592653589793 };` and the value of `foo` would either be the answer to life, the universe, and everything; or a good enough approximation of pi for astronavigation. Again: the final expression of *any* if block is evaluated as the final *value* of that block. This will be a recurring theme.

What about an if-block which concludes not with an expression but with a statement? That is, what if you just have a branch to carry out some varying sets of imperative logic depending on some condition? The if block still has a value: the empty tuple, sometimes called the unit type in various documentation posts.

As it turns out, even statements in Rust have a type; it's just that the type of a statement is the empty tuple, the "unit type". That's basically a value-less type, but it's still a type. So the evaluated result of an if block which is just one or more statements is *also* just the empty tuple type. Of course, you're probably going to just ignore that value, as it's not very useful to you, and that's fine. (It would be kind of weird *not* to ignore it, actually.) The point is simply that even something that in other languages wouldn't be an expression at all, a statement, still has a type in Rust.


### `match` expressions {>> 1m → 8:15 <<}

Essentially everything we've said about `if` blocks is equally true of `match` blocks. They aren't `match` *statements*; they're `match` *expressions*. Thus, you can assign the result of a `match` expression. So if you were matching on a boolean named `bar`, you could write `let foo = match bar { ... }` with the body having arms for the `true` and `false` cases, and simple values or a long complex block resulting in a final expression, or anything in between---but as long as the final element in the body of each match arms block is an expression, it'll be assigned correctly. And course, because of the strong types and good type inference Rust has, if you get mismatching types assigned this way, you'll get a compile error---and a pretty clear and informative one!


### Other block expressions {>> 1m 25s → 9:40 <<}

These same things are true of generic blocks, actually. This means that you can treat all blocks as the expression they evaluate to, with an important caveat I'll get to in a moment. If you create a standalone block within a function, which you can do just by writing opening and closing braces, you can assign or return a final expression from that block just as you can from an if or a match block.

Now for that caveat: this is strictly true, but it doesn't necessarily play out as you'd expect in loop constructs like `loop`, `while`, or `for`. Loop constructs *do* have an evaluated type and value, but it's always the unit type, the empty tuple. I strongly suspect that's because it wouldn't be especially clear or easy to reason about what the final expression value would be in loop constructs (especially with early breaks, which could lead to *very* weird syntax requirements to be able to specify the final value of the block).

Still, this overall pattern---non-loop blocks evaluate to their final expression's value---opens up some very powerful ways of thinking about what our code does.


### `fn` expressions {>> 2m 30s → 12:10 <<}

Before we get to that, though, let's take a minute to look at what is, at least to me, the most interesting consequence of that principle: that is, the way *functions* are evaluated as expressions.

Note that functions are blocks, so they are evaluated as expressions. This explains a few things you've seen if you've looked at Rust code, and it also makes sense out of some things we've skipped over in earlier discussions on the show.

First, the reason that you don't need a `return` statement at the end of a function to return a value explicitly (as you do in C, Java, Python, etc.) is because functions are expressions. Sometimes you'll hear or see people talk about this in terms of "implicit returns" and that may be technically accurate, but in my opinion that's not *really* the best way to think about it.

Instead, just as with other expressions, the function evaluates to a value, and that value is the value of the final expression in the function (or the empty tuple/unit type, if you have a function with no return explicit return type). If you're thinking about it this way, the `return` keyword exists so that you can specify an early exit from a function and specify what the function expression's value should evaluate to at that particular location. Functions are just expressions like all other blocks, and `return` has the same kind of role in them that the `break` keyword and statement does in a `loop` construct.

Second, thinking back to some of the things we looked at last week, this helps us understand how to think about the *type* of a function: it has a given set of inputs and has a specific type as its output. Even when we don't specify the return type, it still exists; it's just---implicitly---the unit type. And this is the bit which really provides a lot of power for *reasoning* about our code.


Conclusion: reasoning about code {>> 4m 45s → 16:55 <<}
--------------------------------

The fallout of all of this is something that our friends over in Lisp land have known for a long time, and it's not the glory of nested layers of parentheses. No, it's that when everything is an expression, you can start thinking about the building blocks of your program in a new way. Loops, conditionals, function calls, and so on are *just like* the expressions were familiar with from other languages. When you look at a program in Python or Java and see the number 2, or a simple strong concatenation, or a mathematical operation, you know how to reason about that. As an expression, you know it has a *value*, and even though the value depends on other things in the program, it's still just a value.

In languages where more things are expressions, you can start thinking of *all* of them in that same way. In principle, a function call is no different than addition or subtraction or string concatenation. Neither is any other kind of block. They're just expressions!

There are caveats to that, of course: if you write a function which modifies a value you pass into it, or a loop or conditional which modifies items declared outside the loop, you can't think of it simply as that expression value. But, first, note that that's true of statements which include expressions in other languages, too: if you have side effects, you have side effects. That's why it's a bad idea to modify a pointer (say, with a pre- or post-increment operator) *and* do something with its contents on one line in C or C++ code, for example: it makes it harder to understand what's going on, because there are side effects.

So all sorts of expressions can have side effects, but wherever you avoid those kinds of side effects, you can think of the terms as straightforward expressions. In Rust, that means that anywhere you're not dealing with data declared as mutable, you can evaluate all blocks as pure, side-effect-less expressions.

That in turn makes it far easier to treat all of these things as composable expressions with types and values---whether those expressions are functions or numbers or strings or loops or whatever else. Then we can glue them together in whatever way is most useful and effective, and if we've made careful choices about mutability (by which I basically mean: avoid it in general and make it obvious where you use it), we can see very clearly and explicitly what kinds of transformations we are applying to the data under consideration.

All of this, remember, just comes out of the fact that Rust is an expression-oriented language, where nearly all the constructs are expressions which have types and potentially values. It's fantastic!

As an aside: I had gotten so used to thinking of functions as expressions whose value is that of *their* final expression that it got me in a little bit of trouble in an interview a while back. I was writing JavaScript for the interview, after a month in which I had been writing a lot of Rust in my free time, and I spent two minutes trying to figure out why a function wasn't behaving as expected; the interviewer finally---and thankfully, he was gentle and sympathetic about it---pointed out that I didn't have a `return` statement. I had become used to thinking of functions as having values, and those values simply being the value of their final expression. It's powerful and useful, and once you get used to it, you miss it when you don't have it.

Next time, we'll make a long-promised foray into the challenge that is dealing with lifetimes explicitly in Rust.


Outro {>> 1m 20s → 18:15 <<}
-----

### Sponsors

Thanks to Hamza Sheikh and Chris Palmer for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If *you'd* like to sponsor the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, or cash.me.


### Follow/support

You can find links to each of those, to other things mentioned on the show, and notes and detailed code samples illustrating macros at NewRustacean.com. You can also follow the show on Twitter @newrustacean, or follow me there @chriskrycho. You can help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, tweeting about it, or just telling a friend!

Thanks to everyone who's added corrections and fixed typos in the show notes; pull requests are great. I also love hearing from you! Definitely say hi on social media, add your thoughts in the thread for the episode on the Rust user forum at users.rust-lang.org, or shoot me an email at hello@newrustacean.com.

Until next time, happy coding!
