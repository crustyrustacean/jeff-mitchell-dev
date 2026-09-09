+++
title = "De-mystifying JSON"
date = 2026-04-25
description = "How to look at a JSON shape and translate it to Rust structs and enums."
categories = ["Field-Notes"]
tags = ["data-modelling"]
draft = false
aliases = ["/2026-04-25-rust-field-notes-de-mystifying-json"]
+++

Looking at a chunk of JSON and figuring out how to model it in Rust used to intimidate me. I'd stare at the nested braces and arrays and freeze. Today something clicked, and I want to write down the trick before I forget it.

The trick is just two rules:

- Curly braces (`{}`) → a `struct`.
- Square brackets (`[]`) → a `Vec`.

That's it. Walk the JSON from the outside in, and every time you hit a `{`, start a new struct. Every time you hit a `[`, you need a `Vec` of whatever's inside it. The fields between the braces become the fields of the struct. Curly braces tell you to _nest_; square brackets tell you to _group_.

Let's apply this to a real example. Here's a sample of JSON output from a local large language model:

```json
{
  "id": "chatcmpl-t8a0zd42kqgp3n2k3nk6wf",
  "object": "chat.completion",
  "created": 1777002236,
  "model": "qwen/qwen2.5-coder-14b",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "",
        "reasoning_content": "",
        "tool_calls": [
          {
            "type": "function",
            "id": "628973129",
            "function": {
              "name": "list_files",
              "arguments": "{\"path\":\"/tmp\"}"
            }
          }
        ]
      },
      "logprobs": null,
      "finish_reason": "tool_calls"
    }
  ],
  "usage": {
    "prompt_tokens": 356,
    "completion_tokens": 26,
    "total_tokens": 382,
    "completion_tokens_details": {
      "reasoning_tokens": 0
    }
  },
  "stats": {},
  "system_fingerprint": "qwen/qwen2.5-coder-14b"
}
```

Walking it from the outside in: the outer `{}` is the top-level struct. Most of its fields are flat scalars — easy. Things get interesting at `choices`, which is an array (`[]`) containing an object (`{}`) — so that's a `Vec` of a new struct. Inside that struct, `message` is another nested object, and `tool_calls` is yet another array of objects. Every `{` opens a new struct; every `[` opens a new `Vec`.

Here's what falls out when you do that walk:

```rust
#[derive(Clone, Debug, Deserialize)]
pub struct ModelResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<ModelChoice>,
    pub usage: ModelUsage,
    pub stats: ModelStats,
    pub system_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelStats {}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelChoice {
    pub index: usize,
    pub message: ModelMessage,
    pub logprobs: Option<String>,
    pub finish_reason: FinishReason,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    ToolCalls,
    Length,
    ContentFilter,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelMessage {
    pub role: Role,
    pub content: String,
    pub reasoning_content: String,
    pub tool_calls: Vec<ModelToolCall>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelToolCall {
    #[serde(rename = "type")]
    pub tool_call_type: String,
    pub id: String,
    #[serde(rename = "function")]
    pub tool_call_function: ToolCallFunction,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub completion_tokens_details: ReasoningTokens,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReasoningTokens {
    pub reasoning_tokens: usize,
}
```

A few things worth pointing out, because they're the small wrinkles that always trip me up.

**`serde` does the heavy lifting.** Bring it into scope via `Cargo.toml` and the `#[derive(Serialize, Deserialize)]` macros handle the actual work of _serializing_ (data going out over the wire) and _deserializing_ (data coming in). The struct definitions are the shape; `serde` is the translator.

**Field renaming for reserved words and clarity.** In `ModelToolCall`, I can't name a field `type` because that's a Rust keyword. The fix is `#[serde(rename = "type")]` on a differently-named field, telling `serde` how the JSON name maps to the Rust name. I did the same for `function` just to give it a more descriptive name on the Rust side.

**Strings of JSON inside JSON.** Note that `"arguments": "{\"path\":\"/tmp\"}"` — those escaped quotes mean the value is a _string that happens to contain JSON_, not a nested object. So `arguments` stays as `String` here. If I need the inner structure, I'd parse it separately.

**Enums for fixed sets of variants.** My first instinct was to make `finish_reason` a `String`, but the API only ever returns one of a known set of values (`"stop"`, `"tool_calls"`, `"length"`, `"content_filter"`). Modelling it as an enum means I get exhaustive `match` checking and a compile error if a new variant shows up I haven't handled. The `#[serde(rename_all = "snake_case")]` attribute is what bridges the JSON strings and the Rust variants — `"tool_calls"` in the JSON maps to `ToolCalls` in the enum.

That's the whole trick. Walk the shape, brace by brace, bracket by bracket. Structs for objects, `Vec`s for arrays, enums for fixed sets of values. I've been working with JSON for a while and I only just saw this clearly today — writing it down so I don't forget.
