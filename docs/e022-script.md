# TODO: something something closures pun

We first talked about closures all the way back in episode 4, almost two years ago in real time! However, I only hit the topic fairly glancingly then, really as an aside in the discussion about functions in general. Recently, though, I was working with a particular, slightly quirky situation in my slow-moving (but moving!) static site generate project, Lightning, and found a spot where closures were *exactly* what I needed and also where I needed to tighten up my own understanding of how they currently work. So I figured I'd take some of that hard-won knowledge and share it with you!

### Review: What is a Closure?

First, let's review. What exactly *is* a closure?