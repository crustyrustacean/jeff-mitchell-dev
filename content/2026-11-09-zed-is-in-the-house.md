+++
title = "Zed is in the House"
date = 2026-11-09
description = "A quick take on getting Zed up and running on Windows."
categories = ["Code Editors"]
tags = ["rust", "zed"]
draft = true
+++

Thanks to a kind person in the 2025 RustConf discord, I'm now *easily* able to get Zed on Windows.

<!-- more -->

The key is this repo: [zed-windows-builds](https://github.com/deevus/zed-windows-builds)

Zed is not officially available on Windows yet, it's still a work in progress. If you're brave, you *can* build it yourself from source, but the process is not for the light hearted.

With the repo I linked, you just install [Scoop](https://scoop.sh/) and follow the instructions in the `README`.

Honestly couldn't be easier.

Bear in mind, you're on the bleeding edge here, and any issues you hit should be submitted through the official Zed channels.

Now that I'm Zed-ified, there is one other thing I wanted to make a note of, not only for myself but for others. Zed can be configured locally, per project by adding a `.zed` folder and adding `settings.json`. I always want word wrap, so add this to your `settings.json`:

```json
{
  "soft_wrap": "editor_width"
}
```

Thanks to [this post](https://www.potatofi.com/posts/word-wrap-in-zed) for showing the way.

You can do this globally as well, but I haven't learned how yet. :)
