+++
title = "Enter MXC: What Process Containers Even Are"
date = 2026-10-13
description = "Part one of a field report on Microsoft's MXC: what process containers actually are — AppContainer identities, declarative policy, learn-versus-enforce — explained from the underserved platform, doubts included."
categories = ["Field Notes"]
tags = ["windows", "mxc", "security", "agents", "sandboxing", "rust"]
draft = true
aliases = ["/2026-10-13-enter-mxc-what-process-containers-even-are"]
+++

Every week brings another story about an agent harness treating your stuff as theirs — training on your files, shipping your code to somebody's endpoint, or getting exploited by someone who read the docs more carefully than the vendor did.

I got tired of reading those stories. So I made a vow: either I write my own harness, or I support the people building in the open.

I ended up doing both, too soon, as is typical for me. [rho](https://rho-code.dev) is mine: almost entirely LLM-coded, with tiny corners edited by me directly. It works, and it is forever a pat-pat good-effort project that I regard as learning territory. I even baked a sandbox into it — and I could never shake the feeling that it was precisely what it was: an amateur sandbox with holes. [jinn](https://github.com/jayson-lennon/jinn) is the serious one, a coding agent built by someone whose skills FAR outrun mine, and I support that effort because the alternative was using `pi`, which...sorry but TypeScript. But here is the thing nobody tells you about the vow: **both roads end at the same place.** If the harness is mine, the security is mine — and on `rho` that means an LLM-written harness running LLM-written commands behind a fence its own author doesn't entirely understand let alone trust. If I support someone else's, I'd better understand the answer they picked — because when an agent runs a shell command on my behalf, that command executes with *my full authority*. My files, my credentials, my network. And who wrote the command? A language model. Usually right, occasionally wrong, and in the worst case steered by something hostile that leaked into the prompt.

On Linux, this problem has a deep bench: namespaces, seccomp, landlock, bubblewrap — decades of overflow. Windows has… enthusiasm. It's the underserved platform, which is exactly why I like spending time there, and exactly why I had no idea what I'd do about containment. I mean yes, there is Windows Sandbox, but it's one of those non-option options.

Then, Microsoft shipped MXC after several months of beta-ness: real process containers, a derived sandbox identity for your workload, declarative policy — and no civilian writing about driving it from a coding agent. Underserved territory...f**k yeah!

This post is part one: my understanding of what MXC even is. The part where I actually put it in front of an agent — and find out what fights back — comes after. This is a field report, not a tutorial. I will flag what I don't know, and corrections are welcome; I'd rather be corrected than confident and wrong.

## The problem, stated plainly

An agent harness runs shell commands on your behalf. Those commands are written by a language model, which makes them *semi-trusted by construction*: usually correct, occasionally broken, and in the worst case malicious, if something hostile leaked into the context that produced them.

Uncontained, every one of those commands runs as you. It can read your Documents folder, phone home, delete things, exfiltrate credentials. The traditional defenses don't really apply: an antivirus signature database has no opinion on `rm -rf`, and a permission dialog per command would train you to click "yes" within a week.

Containment inverts the default. Instead of "trusted unless proven bad," the command gets **nothing unless granted**: a declarative policy lists what this particular workload may touch, and the operating system enforces the list. Not a suggestion. Not a best effort. The kernel itself does the refusing.

## Windows has always sandboxed by identity

Here's the part that surprised me most, because Windows' reputation made me expect something baroque: the mechanism underneath MXC is the same one Windows has used forever.

Every file on an NTFS volume carries an Access Control List (ACL) — a list of who may read, write, or execute it. Every process carries an identity — a SID. Access control is just the kernel comparing the two: if you're not on the list, the answer is "Access is denied." Your normal processes carry *your* identity, so they pass every check against everything you own.

What MXC's **process container** does is launch a workload with a *derived* identity instead — an **AppContainer SID**, the same mechanism Windows uses to isolate Universal Windows Platform (UWP) apps and Edge browser tabs. The workload still runs:

- on your CPU — no hypervisor, no VM,
- in your session — same desktop, `whoami` still shows *you*,
- with your environment — `PATH` and friends are inherited.

…but the kernel's ordinary ACL checks no longer recognize it. A file the policy didn't grant answers "Access is denied." Sockets are gated the same way at the network layer — egress denied by default, loopback included. The windowing and GDI syscall surface (Win32k) can be switched off per-process. And a job object fences the whole process tree, so the workload can't escape by spawning children.

No VM. No magic. The same locks that were always on your files, plus a stranger holding the key ring.

## The four pieces

The moving parts, as best I can map them:

| Piece | What it is |
|---|---|
| **Policy (JSON)** | The declarative request: command, working directory, timeout in milliseconds, filesystem grants (read-only / read-write / denied paths), network posture, UI access, lifetime. |
| **`wxc-exec.exe`** | Microsoft's executor. Reads the request, derives the sandbox identity, launches the workload inside it, and passes stdio through so the caller's piping works unchanged. |
| **AppContainer** | The Windows kernel feature doing the actual enforcement — the derived SID, the ordinary ACL checks, the job object, the per-process mitigations. |
| **MXC SDK** | Microsoft's libraries for building these requests in-process. There's also the plain `wxc-exec` CLI, which is what I'm using — simpler, and good enough for a field test. |

The piece I find genuinely elegant: **the policy lives outside the workload.** A contained process cannot edit its own grants, because the grants were never in its reach — the launcher consumed the policy file before the process existed. There's no unprivileged-to-privileged transition to attack, because there's no transition at all. The sandbox was always the parent's decision.

## Learn versus enforce

Same boundary, two postures, and this is the design detail that sold me:

- **Learn** — denied accesses are still denied, but they're *recorded* to a JSON report. This is the authoring loop: run the workload, read the report, grant what it legitimately needed, repeat until the reports go quiet.
- **Enforce** — the same boundary, minus the recording. The production posture.

What makes this more than a convenience feature: you never have to guess what to grant. The OS *tells you*. Getting to least-privilege usually requires either telepathy or a week of flailing at "permission denied" errors; here the failures are the specification. Run the real workload, collect the denials, sort legitimate from suspicious, grant the legitimate ones. The security policy gets *derived from observation* instead of authored from optimism.

## Four tiers, one in use

MXC actually offers several isolation backends, of increasing weight:

| Backend | Isolation |
|---|---|
| `processcontainer` | AppContainer — same session, filtered identity |
| `isolation_session` | separate Windows account and desktop |
| WSL container | a real Linux distro, via WSL |
| microVM | hardware-virtualized |

The lightest tier is the interesting one for agents: per-command containers with low latency, where each shell call the agent makes becomes a transient container that spawns, runs, and destroys. The heavier tiers trade exactly the things an interactive agent needs — speed, session access, a working terminal — for isolation against genuinely hostile code. Different threat models. Worth knowing they exist; not the story here.

## What I don't understand yet

The section that would have been left out of a braver-looking post, left in on purpose:

- **How the derived SID is actually constructed.** I know it's derived from the policy and that different policies produce different identities, but not the mechanics. "It's a SID and the kernel honors it" is about my depth.
- **What `needsDaclAugmentation` augments.** The probe tool reports it, `false` is the good answer, and my investigation stopped exactly there.
- **How the filesystem virtualization works** — a contained process sees a *virtualized* `%TEMP%` that isn't the real one. I know it happens; I couldn't tell you where those files actually live or how the redirect is wired.
- **Whether loopback can ever be allowed** in the base tier, or whether "deny" is the only honest posture there.
- **The msys story.** Git-bash binaries simply fail to initialize inside an AppContainer — I have the failure code and a plausible explanation involving a privilege that sandboxed identities structurally lack, but that's inference from a corpse, not understanding. Deserves its own post once it's earned one.

If you know these answers, the comment box is open. If you don't, welcome — we can be unsure together and grow.

## Next

Part two is the part with blood in it: handing MXC to a real agent as its default shell sandbox, learning mode as the authoring loop, what a workload actually asks for when it thinks nobody's watching — and at least one discovery I did not see coming. The vow stands. The platform is still underserved and the torch is lit.
