+++
title = "Shut-Up and Build: Photography API"
date = 2026-12-21
description = "An on-going series on how to not overthink and just build."
categories = ["Series", "Shut Up and Build"]
tags = ["photography API"]
draft = true
+++

I used to love Instagram.

The ability to share your creative photos with folks, in an easy way, and at the same time see what others were into, was really fun and nice.

Then Instagram became terrible, so I left. I wanted something of my own.

<!-- more -->

For years (literally) I tried different things and different ways to create a "photo blog". The major blocker, always, was:

"Where do I store the photos in a convenient way that scales for the future."

I never was able to answer this question, and I allowed it to be a blocker. Rather than muster the courage to ask questions and find solutions, I never built anything.

Well, Claude helped me see an easy answer, several actually.

You can store photos in a Github repo, then you get links to them.

Mind blown.

Not being entirely willing to give up the overthinking, after reflecting on the easy options I'd been presented with, I decided I still wanted something more robust and made for the future, but at the same time I didn't want to break the bank. I didn't want the complexity of hosting my own server or the cost of that.

What to do?

A few months ago I worked out how to use [Apache OpenDAL](https://opendal.apache.org) with Shuttle. I developed a "blog" API, in which I worked out a lot of things, but never really finished or carried on with the work.

I dusted this code off, and together with another template repo I'd built, Claude and I turned it into this:

[Photography API](https://crustyrustacean-photograph-api-8w6z.shuttle.app)

I now have a simple API and admin console where I can upload photos from computer, phone, whatever, and get them into Cloudflare R2 storage. The API that sits in front will then return all the photos to me so I can consume them in other place.
