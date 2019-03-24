# Bonus: How the Sausage Gets Made

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is a bonus episode: How the Sausage Gets Made!

## Sponsor: [Parity](https://parity.io/jobs)

First of all, I’m really pleased and grateful that Parity continues to sponsor the show, and it’s because they want to hire *you* to come work in Rust with them!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## Conferences

Second up, it’s time for me to mention some Rust conferences!

First, and most urgent is RustLab, to be held in Florence, Italy, this June 28–29. I call it the most urgent because the CFP closes tomorrow, March 24! (I meant to have this episode out a week ago! Whoops!)

Second is RustConf, being held August 22–23 in Portland, Oregon. Once again I’m on the program committee, and once again we’d love to have you submit. It was a great conference last year, and I hope to have an equally great program this year! And hey, I’m looking forward to being in Portland again, as I was this week for EmberConf—it’s such a fun city!

Third, near and dear to me (I seriously could not be more excited) is Colorado Gold Rust: a brand new regional Rust conference being held in Denver, Colorado, on September 20–21. I *will* be there, and I will almost certainly be there in a very official New Rustacean way! Stay tuned for details on that as we hammer them down.

Finally, Rust Belt Rust is being held this year in Dayton, Ohio, on October 18–19.

The calls for proposals for Colorado Gold Rust and Rust Belt Rust are not yet open, but I will certainly mention on the show when they are.

## How I Make the Show

Okay, so today’s episode is a little unusual for the show, but I get asked questions about how I make the show and about a number of small but important decisions I’ve made along the way often enough that I thought it would be worth setting this all down in one place, so that I can have an easy place to reference when those questions inevitably come up!

I want to note before I dive in, though, that what follows is *my approach and process*, as it has evolved over the course of the show and as a relatively financially successful show with a solid listener base. It is *not* my recommendation of how *you* should produce a show if you decided to start one! If you're interested in my thoughts in *that* direction, you should check out the two guest lectures I delivered on the subject at North Caroline State University back in the fall of 2016, both linked in the show notes ([A], [B]).

[A]: https://winningslowly.org/bonus.03/
[B]: https://winningslowly.org/bonus.04/

### Prep

I also have a confession: I drafted this episode back in January, because I *knew* that I’d be super busy right now in March because I was teaching a workshop on TypeScript again at EmberConf, and trying to get out *any* episodes in March last year just about did me in! But that leads me into the first bit of how the show gets made: these episodes are all scripted top to bottom.

That wasn’t always the case. The first few episodes of the show, I just wrote outlines and winged it. That was… not a good idea. I managed it okay, but if you go back and listen to those episodes, you can *tell* I’m winging it, and not in a good way. There were two big problems with doing it that way for me.

1.  The first problem was that I had to record basically as soon as I finished learning something, because there was no way that an outline was going to do the job even a few days or a week later. My goal from the start has been to make this a top-notch resource that would have years of value to people, and the deeper into the language I got, the less comfortable I was with just saying stuff off the top of my head, instead of writing down *exactly* the right thing to make sure I got it right.

    So, a few episodes in, I started scripting the episode top to bottom. And, again: you can tell! It's obvious that I'm reading a script, both in the sense that it's obviously not just off-the-cuff (and it's a little bit less natural), but also in the sense that I can be much clearer and more precise as a result, and hopefully that tradeoff is worth it. Of course, it doesn't protect me from making mistakes! You can check out even a few recent episodes and note that there are corrections in the show notes! It does mean those mistakes are much less likely to happen, and they're typically smaller!

2.  The second problem in doing it off the cuff was the editing job. So, here's the thing: when I edit a podcast, I *really* edit a podcast. I cut out umms and uhhs, and long pauses, and coughs, and so on. If I stumble over something, I nearly always back up and say it again. If, when I'm speaking from the script, something just *sounds* wrong, I back up and say it a different way. But those kinds of stumbles and things which made me need to edit happened a *lot* more when I was recording everything off the cuff, unscripted. That in turn meant that I had to spend more time editing the show, rather than doing, well, literally anything else! And as much as I enjoy editing podcasts—it's actually quite pleasant work for me!—it's not something I want to spend more time doing than actually *necessary*.

    Scripting lets me make a tradeoff: I spend a lot more time in up-front preparation, in exchange for a lot *less* time spent editing… and, as a bonus, I get scripts that I can make available with the episodes for people who prefer reading over listening, or for people who just want to go back and review the material in a different medium later.

### Recording

Given that kind of prep work, producing the show is pretty straightforward most of the time! I'm just recording off of a script! Even when I do have guests, they have all been pretty easy to coordinate with. We just set up a Skype call (or something like that) and I split apart the guests' audio from mine so I can do that same kind of careful editing on both sides of the recording. I'm on a Mac, which means there are a bunch of *really great* tools for these kinds of things—in particular, [Audio Hijack] and [Loopback] from [Rogue Amoeba] have been game-changers for me when dealing with recording. Audio Hijack also supports *streaming*, so I hook it up to an [Icecast] server and broadcast my live recording sessions to the internet sometimes (like right now!).

[Audio Hijack]: https://www.rogueamoeba.com/audiohijack/
[Loopback]: https://www.rogueamoeba.com/loopback/
[Rogue Amoeba]: https://www.rogueamoeba.com/
[Icecast]: https://icecast.org

When I'm recording, whether alone or with guests, I take note of any obvious moments where I need to make an edit—I just write down the time stamp of my recording from Audio Hijack in a notes app, as a Markdown-style to-do list item. If I'm recording with guests, I also write down anything they mention that seems interesting and worth linking, so that I can use it for the show notes later. For solo episodes, I actually build everything I need for the show notes during the scripting process: I add links to the script wherever I mention a particular topic, and I built [a small tool (in Rust, of course!)][extract] to extract those links from the script to drop into the show notes when I'm done. For teaching episodes, I also spend the time building out the example code—both as a teaching resource and also just to make sure I understand things correctly!

[extract]: https://github.com/chriskrycho/extract-md-links

### Editing

Now, once I have all the audio in hand, I do the editing work I described a minute ago. I start by preprocessing the audio with a *suuuper* fancy tool called iZotope RX, which is worth its weight in gold for dealing with things like noise removal and cleaning up weird artifacts… but also you will pay your own weight in gold for it.

Once I have preprocessed the audio, I switch into editing mode. Because I've usually written down most of the major edits, it doesn't normally take me very long. I'm using Logic Pro, and it gets the job done *just fine*, though it's pretty obvious that it's not really designed for podcast production. (When traveling I sometimes also use Ferrite, an iOS app, which is *incredible* value for money: full purchase is something like $20 and it's an absolutely fantastic podcast editing studio.)

My workflow is basically: cut out and rearrange as necessary for any of the flubs I made along the way and took notes on, then use Logic's handy strip silence tool to remove any particularly long gaps. I then listen through to the whole episode, pulling together all the places where there was silence, and adding in the musical cues for the intro, an initial sponsor read if there is one, and the outro. This is also the phase where I sometimes turn mistakes into bloopers—only if they're actually good or funny, though. I also add chapters marks into the audio, usually just corresponding to the section headings in the script, which get exported with the WAV file when I'm done. I use a free tool called Forecast to convert that into an mp3 file with chapters in it, and upload that to my hosting!

## Publishing and Hosting

So, speaking of hosting: how do I *publish* the show? Well, you can see that the website just rustdoc, of course, and I talked about *why* I did that all the way back in [the very first proper episode of the show][e001]. But if you know how podcasts work, you know that I need an RSS feed, and one set up to include downloads. And rustdoc… doesn't even know what RSS is, much less how to generate one for a podcast! I *could* write the RSS feed by hand, but happily there's an app for that. I use a tool called Feeder which generates RSS feeds with just about every option you could imagine, including podcasting—I basically just copy the show notes material into a standalone Markdown file and pipe that into the "notes" section for each episode in Feeder, and then my build process for the show pulls the generated feed output into the right location.

[e001]: https://newrustacean.com/show_notes/e001/

The show HTML (including that RSS feed) is all just hosted via GitHub pages. I currently use Backblaze's B2 service—which is effectively an Amazon S3 replacement—to host the audio files, because they're quite reliable and extremely low cost. That setup is one of my favorite things things about podcasting—simple file hosting and an RSS feed is all you need. It's all just built on open web tech!

### Google, Spotify, and Stitcher

And that leads me into the last part of this discussion—an answer to the question I get fairly often: <i>Hey, can you add the show to Google Play Podcasts or Spotify or Stitcher?</i> And the answer is: <i>I could, but I won't.</i> I'm not particularly ideological about many things in software—I save that for the theological side of my life!—but one thing I'm deeply committed to is the open web. The open web is an amazing thing: for all its messiness and all the nasty things that exist on it, the fact that I can put up a blog or a podcast using free, open technologies available to everyone is *amazing*. The fact that people from all over the world can listen to this because of that still astounds me, and I still think it's wonderful. And podcasting, like blogging, is an incredibly simple technology built on nothing but file hosting and RSS.

Unfortunately, Stitcher, Google Play Podcasts, and Spotify's podcast service all aim to make something *different* out of podcasting in their own ways. They're not the only ones, but they're the ones with the most influence. From a technical perspective, all of those services re-host—and, just as importantly, re-compress—my audio. That annoys me first and foremost because I've made extremely careful and considered decisions about my audio; I don't want anyone else changing that. It frustrates me second because they do it primarily because it gives them more insight into their users' behaviors, and it turns out there's money to be made there. Although there are technical benefits to them—controlling that means they have command over streaming, the source won’t go down, etc.—the deeper reasons are those economic reasons.

And that leads me to the final reason I consistently refuse to put *any* of my podcasts on those services or any others like them which may appear in the future: they're interested in monetizing my content and your listening habits for themselves—not for you and not for me. In particular, they want to do that by analyzing everyone's listening habits and trying to deliver more targeted ads. I'm simply not on board with that model. Finally, in each of those cases they clearly want to be *the* source—the sole or primary gatekeepers and middlemen—for podcasting. And I have no interest whatsoever in contributing to the centralization of yet another open web standard. We don’t need a Facebook for podcasting.

Now, to be clear, I don't think it's wrong for anyone to use those services, and I don't think there's anything wrong with other podcasters making different choices about these particular tradeoffs! Depending on what your podcasting needs look like, it may be worth dealing with those downsides as part of a larger play to be financially viable, or because you’re part of a podcast network that is on those platforms, or simply because those tradeoffs don’t bother you as they do me. That’s legitimately fine: this is a complicated space when it comes to the moral reasoning! But this is a place *I* draw the line. If Google, Spotify, and Stitcher decide to work the same way everyone else does—*with* the open web—they're welcome to my shows. Because, after all: it's just RSS! But so long as they're trying to do an end-run around the open web for better ad monetization, I'm not interested.

## Outro

Anyway, that's it for this bonus episode. I hope it was interesting, and if it piqued your curiosity or encouraged a budding interest in podcasting, I do encourage you to check out the podcasting guest lectures I mentioned a few minutes ago—you can check them out in the list of bonus episodes at <winningslowly.org/season-bonus.html>, and I've linked them both in the show notes! Likewise, if the ethics concerns of this particular episode are interesting to you, Winning Slowly is likely to be of interest in general. This season, my cohost and I are basically arguing it out every episode in the interest of finding a third way that is neither techno-utopianism nor radical techno-skepticism.

### Patreon Sponsors

Thanks as always to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- Oladapo Fadeyi
- Nick Stevens
- Rob Tsuk
- Embark Studios
- Alexander Payne
- Ryan Osial
- Matt Rudder
- Bryan Stitt
- Graham Wihlidal
- Daniel Collin
- Oluseyi Sonaiya
- Daniel Mason
- Jako Danar
- Raph Levien
- Peter Tillemans
- Soren Bramer Schmidt
- Evan Stoll
- Adam Green
- Anthony Deschamps
- Brian McCallister
- Joseph Schrag
- Jonathan Knapp
- John Rudnick
- Benjamin Manns
- David Carroll
- Behnam Esfahbod
- Jason Bowen
- Paul Naranja
- Dan Abrams
- Chip
- Nick Gideo
- Martin Heuschober
- Scott Moeller
- Jerome Froelich
- James Hagans II
- Nathan Sculli
- Ramon Buckland
- Arun Kulshreshtha
- Michael Mc Donnell
- Andrew Dirksen
- Johan Andersson
- Nicolas Pochet

You can sponsor the show at <patreon.com/newrustacean> or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/bonus/_14/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!