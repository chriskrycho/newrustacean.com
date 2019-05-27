# Meta 3: Happy Coding!

## Intro

Hello, I’m Chris Krycho, and this is the *final* episode of New Rustacean: a show about the Rust programming language and the people who use it.

## This is it!

Yes, you heard that right: after three years and nine months of New Rustacean, this is the last episode of the show—or at least, the last *regular* episode of the show. To explain, I will tell you a story, and I will tell you a dream. And they are the same: the reason why I started learning Rust in the first place, and the story of this beautiful almost-four-year detour of a podcast which has come to be such a significant part of my life.

There are two critical things you should know about me. Everything else in this story is a result of these two facts.

First: I am… *particular* about the tools I use. Put another way: I am kind of a snob about software, especially UI. Second: from January 2013 through May 2017, I was enrolled in a Master of Divinity program—studying pastoral and practical theology.

As is the case with most such degree programs, the school had a standard format for paper submissions. Documents had to be submitted in Microsoft Word documents with 12-pt Times New Roman, exactly per the 8th Edition of the Turabian style guide. But I am a snob, and Microsoft Word is, in my opinion, an capable-if-annoying desktop publishing application—and a *horrible* writing environment.

For years before going to seminary, I had been working in plain-text formats for writing on my blog. I had long since become frustrated with What You See Is What You Get editors in tools like WordPress, and so had experimented with Textile, ReStructured Text, Markdown, and so on. There were (and are) a handful of very good plain text editing apps that simply got out of my way and let me *write* and then generate HTML to publish as I liked. I had fallen in love with this flow for a very simple reason: it let me focus on the words I was writing. This is still exactly the approach I use today—and not only for my blog, but also for the scripts of this show!

So I had a writing workflow I liked, and a submission format I hated working in. I began poking around to see if there was a way to write Markdown and generate Word documents. The answer was *yes*, and I was delighted. Over the next few years, I assembled a whole chain of tools for writing academic papers, until I had a flow that let me research, write, and even cite documents in a way that worked much better than doing so in Word… while still handing in papers in the format my professors required. There was friction, but much less friction than working in Word, at least for my (admittedly fussy) aesthetic sense.

A little over halfway through the program, having hit another one of those little friction points, I started wondering what it would take to build something myself that would do *exactly* what I wanted. Some of the pieces I had assembled were good but not optimized for this flow. Others were mediocre at best. And then, noodling on that idea, I realized: if I build a tool that made the research writing experience genuinely *good*, it might be a viable product in the world. *Lots* of people hate their writing workflows. The best options for people unwilling to put up with Word and EndNote involve command-line tools with arcane invocations. There was, I thought, something here worth pursuing. A genuinely great research writing environment *needs* to exist in the world, and I might just be able to make a reasonable living while making researchers’ lives better. It was early summer 2015. That’ll be important.

I knew from the start that I wanted to design the app for eventual cross-platform support. I may be a Mac guy myself, but the audience of research writers is far broader. At the same time—perhaps unsurprising given my aforementioned *particularity* about UI, I have a firm commitment to fully native user interfaces with great performance. That ruled out Electron immediately, despite my background as a web developer. And just as importantly, “cross-platform” also meant targeting iOS, given how I did (and do) a lot of my writing; even if I had wanted to use Electron I couldn’t have. If I wanted to have shared business logic and target more than iOS and macOS, though, I would need to write shared libraries in a language that isn’t Objective-C or Swift. That meant—I thought—C or C++. But I’d also spent the previous six years writing C and C++… and I *really* didn’t want to write a layer of cross-platform business logic in either language.

So I started looking around and asking friends if there was any alternative. One of them linked me to Rust. (Jeremy, if you happen to listen to this, *thank you*. You literally changed my life that day!) I spent a few hours that week working through [Rust by Example]. Right away, I knew three things: first, that Rust was exactly what I had been looking for; second, that *Rust was something really special*; and third, that there was a *lot* of work to do in the space. Remember the timeline! This was the summer of 2015—Rust had hit 1.0 a matter of weeks before I got this idea and started researching my options.

[Rust by Example]: https://doc.rust-lang.org/stable/rust-by-example/

Over the next few months I kept mapping out the basic structure and design for this application. And I also was looking around everywhere for more resources about Rust. In particular, I was looking for a podcast, because then as now I do a lot of *learning* through podcasts, especially when runnning. There was one show about Rust at the time, and it didn’t seem like the hosts were going to keep it up for long. (They didn’t.) At the same time, I was *busy*, and I needed a way to help myself keep learning, and learning deeply and well.

I had been producing my *other* main podcast, Winning Slowly for a year and a half, and so I thought: <i>I bet I could do a podcast talking about *learning* Rust…</i>

The rest, as they say, is history… the history of this show. I did indeed learn Rust along the way. But a funny thing happens when you set out to make a show with the production quality and level of detail I do. As I noted in my “How the Sausage Gets Made” bonus episode a couple months ago: these episodes take a *huge* amount of time to put together. Each of the FFI episodes I just publishe, for example, were 20–25 hours of work. That’s on the high end for teaching episodes… but they weren’t outliers, either. The easiest episodes I produce—the news and bonus episodes—still take 4–6 hours. The shortest teaching episodes and the Crates You Should Know episodes take me a good 8–12 hours to produce.

And I have a family and a church and a life that *isn’t* my day job *or* podcasting. Which has meant that, aside from filling up the pages of a notebook with thoughts and considerations along the way, I never made any progress on that dream of a genuinely great research writing environment. In fact, I barely made any progress even on much *smaller* projects, like the static site generator I started back in 2016.

So: it’s time. I have covered every major topic on my list—though of course there will always be more to cover, because Rust is always growing and changing. (In just a few months, for example, async and await will be stable and things will move forward in a big way again!) Even as the language itself matures and the rate of change in *that* category hopefully slows over the years ahead, the community is exploding and the number of things I *could* cover with it. But the language itself I think I have done justice.

And I still have that dream of building a great research writing environment. I have sunk the better part of four years into *teaching* Rust, but I’m ready to get back to the reason I started learning it in the first place.

## The Promise of Rust

Four years ago, Rust was a dream and a promise: a dream of a world in which safe systems-level programming was possible and accessible to everyone, and a promise of stability as we all tried to make that a reality. There is still a long way to go… but that promise has been kept so far, and in a very real way, the dream is being fulfilled.

People who always found C and C++ too intimidating, too unfamiliar, and at the end of the day too *dangerous* have found in Rust an opportunity to write systems-level software: systems-level both in performance and in the kinds of programs they can write. And the past four years of polishing have sanded off most of the rough edges that were there with 1.0: the many *incidental* points of difficulty. What is left, increasingly, is a language which exposes the real complexity of writing fast, safe software—but guides you through how to handle that complexity. The result is a real joy. Over the past few months at work, I have been working on the Volta project—a Node toolchain manager written in Rust—and it has just been delightful throughout. And other developers who had little or no experience with Rust before coming to the project are enjoying and excelling with it.

The future for Rust is, I think, very bright. The last four years were, in so many ways, just the beginning: most people who will ever use Rust haven’t yet, most programs that will ever be written in Rust haven’t been yet, and the most important changes it brings to the industry haven’t been seen yet! I can’t wait to participate in it in a whole new way in the months and years ahead.

## Thanks

Here at the end, I want to say a heart-felt <i>thank you</i>—to *everyone* who has supported the show in any way along the way. Credit here particularly goes to my wife Jaimie, who encouraged me when I thought about starting the project in the first place, and who has made the space for me to do the work a podcast of this sort requires. (Also, hosting [a video game podcast with her][ma] is far and away the most *fun* I have with any podcasting I do!) Second, my little girls have been enthusiastic fans all along the way. When I started this, they were just 3 and 1; this week, they turn 7 and 5. Third, my friend Stephen Carradini, with whom I host my *other* podcast, [Winning Slowly][ws], has been a steady encouragement as well.

[ma]: https://massaffection.com
[ws]: https://winningslowly.org

Next, I want to say thank you to everyone who has sponsored the show along the way—you have made it far easier for my family to see this as a worthwhile investment of my time! That goes for both Parity and Manning as corporate sponsors, and it goes even more for the many of you who’ve chipped in everything from a single dollar to those of you who have, over the life of your contributions, given hundreds. Your generosity and support genuinely amaze me.

And finally, thank you—thank you *so much*—to all of you who have *listened* to the show over the last years. So far as I can tell, the total number of people who have ever listened to the show is in the tens of thousands; and a good five or six thousand of you have been tuning in to every episode. Hundreds of you have emailed me over the life of the show. That amazes me. I remain profoundly grateful for the time and attention you have given me.

## Conclusion

It’s not an exaggeration to say this show has changed the course of my life in so many ways. I am where I am today professionally in no small part because of this show. And if, as I hope, New Rustacean has also contributed in some small way to Rust’s success, I’m profoundly glad and grateful to have been a part of it.

I’ll still be around on Twitter, both @chriskrycho and @newrustacean; and you can always email me at either hello@newrustacean.com or hello@chriskrycho.com. If you want to follow along with the research writing app project, I set up a mailing list for it at [buttondown.email/rewrite][rewrite]. If you want to *support* me as I do that, including getting *more* updates on than you will in the newsletter, you can do so at [patreon.com/chriskrycho][patreon]. As for New Rustacean itself: I expect to keep the show online indefinitely—my hosting costs are tiny, and so this will be here as long as I think it has value—so feel free to keep pointing people to it as a learning resource. I have of course paused all ongoing contributions, but if you decide to send me a thank-you via one of those *other* services listed on the show website, I will of course not complain. And on a final note, keep the show in your podcast app. I make no promise ever to post another episode here…but I also make no promise not to. If I have something I just *have* to say about Rust, well… you know…

[rewrite]: https://buttondown.email/rewrite
[patreon]: https://patreon.com/chriskrycho

Thanks again for listening, and… happy coding.