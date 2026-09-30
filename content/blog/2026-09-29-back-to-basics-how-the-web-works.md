+++
title = "Back to Basics: How the Web Works"
date = 2026-09-29
description = "Servers, browsers, and the three technologies every site is built on — and how a browser turns them into a page."
summary = "I got into Rust partly because of WebAssembly, but I never properly learned how the web itself works. This is the first in a series fixing that: what happens when you visit a page, and how HTML, CSS, and JavaScript become what you see."
categories = ["fundamentals"]
tags = ["web", "html", "css", "javascript"]
draft = true
aliases = ["/2026-09-29-back-to-basics-how-the-web-works"]
+++

I'm going to write a few pieces about the basics of how the web works.

Much of my motivation for getting into Rust was WebAssembly, and wanting to take a different path to become a productive web developer. I haven't spent near enough time just understanding how exactly the web functions. I've had a stubborn refusal to really dig into the basic building blocks. I'll never be independent if I don't understand these foundations.

So, let's rectify that.

## The World Wide Web

> "We've learned Earth's languages through the World Wide Web" - Optimus Prime

So what is the world wide web anyway? Does anyone even call it that anymore?

Imagine a city in which there are buildings that offer services. Every building has an address and when you visit there as a client, there are helpful staff who give back information which you can then take and assemble into something meaningful. The buildings are servers and they offer web sites. You, as client act as the web browser. When you visit an internet location, a server answers and gives you back a series of files which your browser takes and renders into something meaningful.

This is overly simplistic, but there's a lot to unpack at each step and having a high level view of what's going on can help.

Every web site, every last one, is built on this foundation of technologies:

- HTML
- CSS
- JavaScript

The methods and techniques may vary, but at the end of the day that's what the browser needs to render information to you as the viewer.

A web server gives back an HTML file (usually called index.html) which has links to styles (CSS) and scripts (JavaScript). When a browser makes requests for these resources, it does that via links available in the HTML file.

## Structure (Hyper-Text Markup Language, HTML)

Every web site needs bones and a skeleton. It's HTML's job to express how a web site, including its content and structure, is to be represented. This is the first thing the browser loads. The browser constructs a model in memory, called the Document Object Model (DOM). I always feel slightly dirty when I say DOM...I digress...

Here's the smallest useful page — save it as `index.html` and open it in a browser:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>My first page</title>
  </head>
  <body>
    <h1>Hello, web!</h1>
    <p>I'm a paragraph. Plain text, wrapped in tags.</p>
  </body>
</html>
```

Each tag — `<html>`, `<head>`, `<h1>` — becomes a node in the DOM tree the browser builds from this file. That tree is what everything else manipulates.

## Styles (Cascading Style Sheets, CSS)

It's the job of CSS to express what a web site looks like. CSS rules allow you to selectively target HTML elements and apply a style to them. This is the second thing loaded by the browser. The browser also constructs a CSS Object Model (CSSOM) which can be targeted and manipulated dynamically with JavaScript. The style and structure are ultimately combined into a "render tree" which the browser uses to "paint" the final web site into view.

Drop a `<style>` block into the `<head>` of that same page:

```html
<head>
  <title>My first page</title>
  <style>
    h1 {
      color: steelblue;
    }
    p {
      font-family: sans-serif;
      font-size: 1.1rem;
    }
  </style>
</head>
```

The selector (`h1`, `p`) finds matching elements in the DOM and applies the properties to them.

## Functionality (JavaScript)

A web site can exist with only HTML and CSS, technically you don't need anything else. However, it's static and relatively boring. You can do a lot with these technologies now, more than 10 years ago, and depending on what you're doing minimal interactivity might be fine. Generally speaking though, you're going to need some JavaScript. It enables interactivity, the ability to manipulate state, and the ability to change the page content, structure, and style dynamically. JavaScript is the very last thing the browser loads.

Finally, a `<script>` at the end of the same page:

```html
<script>
  const heading = document.querySelector("h1");
  heading.addEventListener("click", () => {
    heading.textContent = "You clicked me!";
    heading.style.color = "tomato";
  });
</script>
```

`querySelector` finds the heading in the DOM, the listener waits for a click, and when it fires, JavaScript changes both the content and the style — the same DOM node and CSS the first two snippets set up. Because the browser loads JavaScript last, the heading already exists when this runs.

## Closing

This has been a very basic overview of the fundamental web technologies and how individual web sites work. I'll explore each one further and write about them over the next while. You can also read the resources I've linked below for more background!

## Resources

- [How the web works](https://developer.mozilla.org/en-US/docs/Learn/Getting_started_with_the_web/How_the_Web_works)
- [Constructing the Object Model](https://web.dev/articles/critical-rendering-path/constructing-the-object-model)
- [Render-tree Construction, Layout and Paint](https://web.dev/articles/critical-rendering-path/render-tree-construction)
