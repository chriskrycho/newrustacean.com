Read the episode transcript!

# Colin Dean

**Chris:** Can you tell me your name?

**Colin:** I am Colin Dean.

**Chris:** Hi, Colin. Thanks for sitting down with me. How did you get into Rust?

**Colin:** Uh, so, I used to work with Carol Nichols-Goulding who is one of the - I think she’s now on the Rust core team. She introduced me to Rust a few years ago, I’ve held several meet-ups with Rust, and I, just in the last year or so, started using Rust at work.

**Chris:** Awesome. Can you tell me where you work?

**Colin:** Arcadia Healthcare Solutions, where I’m director of software engineering.

**Chris:** Awesome. What kind of cool things are you getting up to with Rust at this point?

**Colin:** So our first big project is actually a very small Rust program that we’re using to act as a supervisor for a much larger system. We needed something that was really reliable, could do concurrent code, and most importantly it was going to be native binary. So we eliminated a lot of contenders for that, kind of arrived at C or C+ +, and then I was like, “Oh, hell no, we’re gonna use Rust.”

**Chris:** Hahaha! I can understand. What are you excited about in the next - I don’t know - six months, a year...what kind of things do you have on your radar for Rust work?

**Colin:** Probably the two big ones are shipping this product that will soon be responsible for supervising petabytes worth of transfer of health record data, and, um, contributing a whole lot to the Rust ecosystem. I’ve hired somebody who works on crates and she’ll be doing a lot of Rust stuff and kind of owning it. It’s her first big job, well, her first job, period - at least her first programming job - and she’ll be in charge of this little Rust component and, so shipping that’s really important, and then figuring out how we want to do all the continuous integration, how we want to support it when things go wrong...those are skills that I’ll be learning, she’ll be learning, and contributing back to the Rust ecosystem, where we’ll be shipping on Windows, so Rust is not necessarily Windows-super-friendly yet, and where we find gaps, we’re going to be contributing the code back to the community.

**Chris:** That sounds awesome, doubly so because you’re hiring, or you’ve hired, one person and you guys are actually using Rust. What have been big wins, and what are perhaps any pain points so far with Rust?

**Colin:** So, um, we haven’t had any big wins yet; the system is still very new, it’s still less than five hundred lines of code. Our probably biggest pain point right now is how to do CI on Windows. Um, I’m kind of used to a development world where I’ve got containers that have the entire build everything, and I kind of push it up, and it just works. Think, like, Travis, or Jenkins on steroids, and I don’t have that where we are right now. Our Jenkins installation has ten jobs in it, and, uh, they all do stuff with Gradle for a Java app that another team uses, so I’m kind of starting from nothing and trying to figure out how do I do this. There are some other...I think Appveyor does, like, Windows builds, so we’re looking at that, but we have some requirements that might need to keep that in-house, and so we’ll have to sort through that, see what exactly we want to do. Um, but the CI story is probably the hard part. I’m also finding some interesting problems with kind of an immaturity of documentation, like the Rust docs are great, but there’s some parts of it that are - especially on crates, because so much of the library is crates - that they’re just not very well documented. So, I’m finding myself contributing a little bit here and there to documentation. I put up a couple of pull requests just today, fixing some docs. So, I think that’s gonna continue to be a thing, and I know that they - the Rust team - is doing a lot to try to push people to improve how they document their code, because the Rust team knows how much of the Rust ecosystem is external libraries. It’s not, you know, there’s no...there’s a standard library in Rust, but it’s not a standard library in, like, in the sense of like a Java standard library where I’m kind of coming from, where there’s just everything.

**Chris:** Absolutely. Thank you for talking with me. I look forward to seeing and hearing more of what Arcadia gets up to and what you get up to!

**Colin:** Cool, thank you very much!
