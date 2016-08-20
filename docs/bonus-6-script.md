# It doesn't need to be sexy

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a show about learning the Rust programming language. This is *Bonus episode 6: It doesn't need to be sexy.*

## News and follow-up

-   Aaron Turon and Alex Crichton published their new `futures-rs` crate, which provides some low-level primitives for a concept variously called *futures*, *promises*, or *delayed* or *deferred* values. I'll link the crate in the show notes; it has some great documentation as well as being what looks to be a pretty solid implementation. I'll come back around and devote an episode to futures in the future. This is a huge thing for the ecosystem, and part of what makes it great is that it's not part of the language, but built on top of it---the language's core primitves expose everything required to do it and do it *well*. This will open a *lot* of doors for higher-level interfaces in lots of other libraries.

-   Closely related, Carl Lerche, the author of the Mio low-level I/O Rust library, recently published Tokio, a network application framework, which takes advantage of `futures` directly.

-   I just bumped into a project called Shadercat, by Claudia Doppioslash, who's already decently well-known for her work helping teach people about functional programming---but in this case, she's looking at doing graphics program with Unity and Rust. If those are interests of yours, you should definitely subscribe to the blog, which I'll link in the show notes.

-   _The Rust Programming Language_ is now a co-authored work by Steve Klabnik and Carol Nichols, who co-founded Integer32, the first Rust consultancy (which I mentioned a few episodes back).

-   Speaking of *writing* projects, keep your eyes open; I have a Rust-related writing project coming your way later this fall, which I'll discuss in more detail once it's out!

Now, for today's discussion.

## What to build

Over the past few months, as I've watched and participated in the Rust community, I've noticed a pattern (and I include myself in it): we glorify the compiler team and work. A lot of the focus in the community is on the language core itself. And this is a problem.

Now, I get this: like I said, I've been guilty of that myself, right down to the kinds of things I've focused on in my "News" sections. The language is *cool*, and the team working on designing it and tweaking it is very talented. More than that, having a language moving forward at such an active pace is something fairly novel. To the best of my knowledge, there is nothing else quite like the pace of post-1.0 Rust development, where a language is simultaneously stable *and* improving its capabilities rapidly, making it better at solving many problems and capable of solving new ones. It's *exciting*.

It's especially tempting because we can jump in and participate in helping improve the language itself---not an experience most of us have had in other language communities, even really great ones. There are a lot of upsides to just how open the Rust development process is, and this is one of them.

But.

*If* we want Rust to really flourish and succeed, we need to look more broadly and choose things besides the compiler to focus our attention on, and to lavish our praise on. And that's a two-fold change:

1.  We need to keep our eyes open for other significant open-source projects in the community, and chip in on those.

2.  We need to look around and note all the *other* things there are to work on, including the small ones, and buckle down and do those.

Let's take those in turn.

### Filling major gaps

First, on filling big gaps: There are a bunch of projects in a fairly nascent state which could make a *big* difference in how useful Rust is and how easy it is for companies to adopt it. And it's worth understanding that one of the biggest things that helps languages be more than an exciting flash in the pan is getting them used in "industry" settings. You're starting to see some of this with Rust already---companies from Dropbox to Academia.edu and many others.
    
But for Rust to go from *niche* to *everywhere*, there are a lot of pieces of "infrastructure" that need to be built outside the language itself. *Big* infrastructure pieces. We need both the low-level infrastructure pieces like mio and the new futures library I mentioned at the top of the show, and some higher-level abstractions sitting on top of those to make it easier for people to build e.g. web services easily. {>> TODO: other gaps <<}
    
### Filling smaller gaps

What about all the little pieces? The community actually has a good start on this. Lots of us have built little exploratory projects---another Rust binding to some C library, a part of an XML parser, etc.---and those are *great* for little learning projects, but we also need to take a lot of those and turn them into production-level tools that are at a level that can be used in larger projects.
    
Now, don't be discouraged: if you've written that kind of small project, and you're thinking, "No way is this ready for some big company to depend on; I'm still just learning!" that's okay. No one should be sad about learning and learning in public, and no one should feel ashamed of their projects. *All* I mean is that as we move from the "just trying this out" phase to the phase where we feel reasonably competent with the language, we can start looking around and thinking about how we can turn "my first XML parser" into a *great* XML parser---perhaps by collaborating with several others who've done the same, and looking at some of the state-of-the-art options out there.
    
Those kinds of small projects aren't that *sexy*, but they're incredibly important. In many ways, given a sufficiently decent lanaguage *foundation*, they're actually *more* important than anything else that the language might do. If you don't believe me, look at JavaScript: the language has a lot of issues in its design, and there are well-known problems with the Node.js ecosystem and tooling. I *like* JavaScript, but the amount of time I've spent chasing down something which broke five layers deep in an npm dependency is infuriating.

But none of that matters that much, because there is also a vibrant community trying to solve every problem imaginable---sometimes several times over in different ways, which causes its own set of problems! And the result is that, whatever the language's warts and whatever the weaknesses of the tools, the Node.js ecosystem is flourishing. And a huge part of that is simply people's willingness to build small things which *aren't* glorious, which aren't at the very center of the community.
    
A willingness to embrace that idea---that it doesn't have to be sexy for it to matter and be a valuable contribution---is huge. We need that in spades for Rust to become all it can. So: go write an incredible XML parser (or, better, help make one of the two that already exists more robust and capable). And then maintain it: fix bugs when they're found, keep on top of pull requests as they come in, and generally try to make it useful to people.

There isn't a lot of glory in that. But it's *extremely* valuable.

And you know what? A lot of Rustaceans are out there doing just that. I saw news come by this week about everything from cryptography to building Alexa skills for the Amazon Echo to a library for getting input from gamepads and controllers. This is fantastic. So, number one: keep it up. And number two: let's work to celebrate those kinds of things just as loudly as the core language itself. They're not in competition. It's the combination of the language and the things we build with it that makes this all worth doing.

## Outro -- Sponsors and Following

Thanks as always to the sponsors who help make the show possible! This month, the sponsors included Chris Palmer, Daniel Collin, Eric Fulmer, Jared Smith, Raph Levien, Stephen Murawksi, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes, and the top tier of sponsors on a dedicated page on the website as well.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at a variety of other services listed at the show website: newrustacean.com.

There you will also find links to the news items from the top of the show, and code samples illustrating referencing and dereferencing data.

You can also follow the show on Twitter \@newrustacean, or follow me there \@chriskrycho. If you enjoyed the show, please help *others* find the show by rating and reviewing it on iTunes, recommending it in another podcast directory, sharing it online, or just telling a friend!

Also, please do respond on social media, in the threads for the episode on the Rust user forum or on Reddit, or via email at hello@newrustacean.com. I love hearing from you!

Until next time, happy coding!