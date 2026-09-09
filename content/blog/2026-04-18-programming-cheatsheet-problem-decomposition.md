+++
title = "Problem Decomposition: A Cheatsheet"
date = 2026-04-18
description = "Struggling to decompose problems into smaller steps? This is the cheatsheet for you."
categories = ["Cheatsheets"]
tags = ["problem-solving"]
draft = false
aliases = ["/2026-04-18-programming-cheatsheet-problem-decomposition"]
+++

This is the distillation of my latest conversation with Claude about problem solving.

## The Framework

Every problem, every time, same steps:

1. **What do I have?** Look at what's in front of you. Not in your head — on the screen, on paper. List it.
2. **What do I want?** Describe the end result in plain words, not code.
3. **What's the gap?** Compare the two. Don't solve the gap — just describe it.
4. **What do I know that might work?** Scan your shapes list. Look backward before you look forward.
5. **One small question about the gap.** If the question feels too big, split it. Keep splitting until it feels almost trivially answerable.
6. **Answer that question.** Try something. If it's wrong, that's data. Go back to step 5.

## Pseudocode Rules

- Pseudocode is comments in plain English describing what needs to happen. That's it. No special format.
- Pseudocode and coding are **two separate activities**. The pseudocode describes the *what*. The code describes the *how*. Don't mix them.
- Each step should feel almost too simple. If a step feels hard, it's actually two or more steps compressed together.
- A step is valid even if you don't know the syntax to implement it. Not knowing the syntax means you have a googleable question, not a dead end.
- Write the steps first. Put `// ???` next to any step where you don't know the syntax. Don't stop to look anything up. Write all the *whats* first, then go back and figure out the *hows* one at a time.
- Validate pseudocode by running a small example through it mentally. Pick specific inputs and trace them through each step.

## The Shapes Are Enough

Most "new" problems are familiar shapes with different names. Routes are just folders. Nav links are just a transform. Frontmatter parsing is just "find the delimiter, slice around it." Tera is just a fancy `.replace()` chain.

When a problem feels new, it probably isn't. When it truly is new, decompose it with the framework, solve it, name the shape, and add it to the list.

## Habits to Watch For

- **The freeze.** The feeling of being stuck comes before you've actually tried. It's the fear of being wrong, not the absence of ability. When you notice it, ask one small question — any question.
- **Discounting evidence.** Successes don't register. Failures stick. When something works, it "doesn't count" because it wasn't natural, or it was too easy, or you needed help. Write it down anyway.
- **The hidden trick.** The belief that there's a better, more correct approach you can't see. There almost never is. The brute force way — loop, compare, check — is the real solution. The fancy approaches are wrappers around it.
- **Negative absolutes.** "I can't." "I don't." "I never." Add "yet" or "right now" for accuracy. "I can't decompose this right now on my own" points toward what's missing instead of slamming the door.
- **All-or-nothing understanding.** The belief that you need to understand something completely before you can use it. You use `println!`, `.find()`, and database connections without understanding the internals. A HashMap is the same kind of leap.
- **Comparing your inside to others' outside.** What looks like effortless reasoning in experienced people is pattern matching against a library you can't see. They have more reps, not a different kind of brain.
- **The grading mindset.** Nobody is grading you. Being wrong costs a compiler error and thirty seconds. The only rush is in your head.

## Key Insights

- The problem is never the whole problem. It's always a stack of small problems.
- Small steps should feel almost stupidly easy. That's how you know the decomposition worked.
- When something feels abstract and blocked, make it concrete. Receipts on a table, cards face-down, a notebook with names in it. The logic is identical — you're just removing the part that triggers the freeze.
- Nothing is lost by thinking in plain language first. The English steps translate directly to code.
- The brute force solution is a real solution. The fancy approach is an optimization you earn after the brute force version works.
- Wrong answers are information. Wrong questions still move you forward. The only thing that doesn't move you forward is not trying.
- You don't need to do this in your head. Externalize — screen, paper, conversation. That's not a weakness. That's just how your brain works best.
- You learned clarinet from nothing to improvisation. You know how to learn this way. Give yourself the same patience with code.
