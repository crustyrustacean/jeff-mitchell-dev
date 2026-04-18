+++
title = "Programming Shapes: A Pattern Library"
date = 2026-04-17
description = "Four repeating patterns that cover most programming problems, no matter how different they look on the surface."
categories = ["Fundamentals"]
tags = ["patterns", "problem-solving", "decomposition"]
draft = false
+++

Most programming problems, no matter how different they look on the surface, are built from a small number of repeating shapes. The names change, the data changes, but the structure is identical. Here are four that cover a surprising amount of ground.

## Find the Biggest

Loop through a collection, compare each item to the current best, update if it's better, return the winner at the end.

**Examples:** highest test score, most popular blog post, most recent date, longest build time, most expensive item in a cart.

**Iterator shortcut:** `.max_by_key()`

## Filter

Loop through a collection, check a condition, keep the ones that match.

**Examples:** published posts, posts with a certain tag, files that are markdown, library members at their borrowing limit, expenses above a threshold.

**Iterator shortcut:** `.filter()`

## Transform

Loop through a collection, change each item into something new, collect the results. Same input count, same output count, different form.

**Examples:** title to URL slug, markdown to HTML, file path to file name, page name to nav link, raw score to letter grade.

**Iterator shortcut:** `.map()`

## Accumulate by Key

Loop through a collection, use a HashMap (the phone book pattern — look things up by name, not position), group things by some key, and combine the values.

**Examples:** count word frequency, total sales per product, group blog posts by tag, track borrow counts per library member.

**The counting version:** key exists → add 1, key doesn't exist → insert 1.

**The summing version:** same thing, but add the amount instead of 1.

## Using the Shapes

Most problems are one of these four, or a combination. "Get the published posts, transform them into HTML, find the most recent one" is filter → transform → find the biggest. Three shapes snapped together.

When a problem feels new, it probably isn't. The surface just looks different. Ask: **which shape on this list does it look like?**