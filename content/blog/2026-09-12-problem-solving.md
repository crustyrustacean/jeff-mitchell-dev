+++
title = "The Card"
date = 2026-09-12
description = "A problem solving methodolgy"
categories = ["fundamentals"]
tags = ["problem-solving", "tools"]
draft = false
aliases = ["/2026-09-12-the-card"]
+++

I struggle *mightily* with problem solving. Embarrasingly, I've spent countless hours and burned many tokens with LLMs acting as wannabe coaches, mentors...psychologists...yes, sadly psychologists.

Anyway, the latest round produced, "The Card".

Here it is.

**The three questions**
1. **What do I have?** A fact. Data, a variable, a socket, nothing. Can't be wrong.
2. **What's the one next thing that has to be true?** Not the goal. One step closer.
3. **Can I do it by hand on a tiny input?** Yes → do it, translate. No → too big, split it, back to 2.

**When the card comes back blank — shrink**
- Go one step earlier: what has to be true right before this?
- Hard-code it: fake the value, get the shape working, replace the fake.
- One instead of many: handle a single case first.
Stop shrinking at the first size you can act on. Small steps feel irrelevant from the artifact's view; that's the sign you've found one.

**Two kinds of stuck**
- *I don't know what* → shrink.
- *I don't know how* → look it up. Docs, examples first. Name the gap precisely enough to search it.

**Run it, don't judge it**
Print, compiler, curl, paper. Any cheap judge beats the one in your head.

**Look back — the rep isn't done until:**
One line in the notes: which shape was it (accumulate, filter, transform, search, sentinel loop, lookup table, state machine, extract), and what was the shrink.
