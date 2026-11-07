+++
title = "Enter MXC, Part 3: The Fence Goes In"
date = 2026-11-06
description = "Part three of the MXC field report: the fence gets built and shipped, and the shell question answers itself — on Windows, bash cannot exist inside an MXC container at all. Plus the launch failure that every green test missed, because the tests were checking a fixture instead of the operating system."
categories = ["Field Notes"]
tags = ["windows", "mxc", "security", "agents", "sandboxing", "rust"]
draft = false
aliases = ["/2026-11-06-enter-mxc-part-3-the-fence-goes-in"]
+++

Part two ended with a plan I was proud of, and a line I'd underline again: *understand in public, then build.* It also ended with a sequencing decision that, in hindsight, I quietly violated.

The plan said Phase 0 first — the file-tool guard, the environment allowlist, no MXC required, closing two of three kill chains on every platform. What actually got built first was the MXC runner. The fence went in before the guard did.

I want to lead with that, because it's the same shape of mistake part two was about. Not "I built the wrong thing," but "I built the thing that was more interesting." The difference this time is that I know it, and the docs say it. More on that at the end.

## What actually shipped

A proof of concept, deliberately narrow. It begins with a *posture* in `jinn.toml`:

```toml
[containment]
posture = "mxc"
```

Three postures, and the defaults matter:

| Posture | What it means |
|---|---|
| `off` | Nobody provides a boundary. Today's behavior. The default, and absent section reads as this. |
| `mxc` | The MXC fence, configured by the keys below it. |
| `external` | You provide the boundary — another user account, a dev container. jinn says so in its log and does not verify it. |

That last one deserves a sentence. `external` is not a fence. It's an assertion, and it exists so the honest person who *has* put the agent in a dev container gets a log line saying "containment posture: external — jinn's own containment is off; the process boundary is whatever user account or container jinn is running under, and jinn does not verify it." Saying that out loud is worth more than silently assuming it.

Under `mxc`, exactly one tool runs inside the container: `powershell`. It runs with the model's command passed as `-EncodedCommand`, the same stdout/stderr streaming, the same truncation limits, the same exit-code passthrough. **The model's contract doesn't change.** It still speaks the same tool, gets the same event, reads the same shape of result. Which shell runs inside the fence is nobody's business outside the runner.

## The shell that cannot exist

Why `powershell` and not `bash`? Because on Windows, `bash` inside an MXC container is not a policy decision. It is a thing that does not run.

Part one left this open with a body count attached: Git Bash binaries fail to initialize inside an AppContainer, and I had a failure code plus a plausible explanation but no understanding. The corpse is real and it is reproducible:

```text
Exited(-1073741502)   =  0xC0000142  =  STATUS_DLL_INIT_FAILED
```

No output. The process starts and dies during DLL initialization, before it exists. `cmd.exe` in the same container with the same policy exits 0, and native PowerShell runs fine. So it isn't the fence refusing Git Bash; it's Git Bash failing to become a process at all.

Since I started writing this, the question went from my inference to a filed bug. [microsoft/mxc#1061](https://github.com/microsoft/mxc/issues/1061) describes exactly this, was triaged and assigned to the AppContainer team, and is still open. The triage analysis names the mechanism better than I did:

> AppContainer's named-object redirection (via `BaseGetNamedObjectDirectory`) only covers Win32 APIs and does not rewrite absolute NT paths. `msys-2.0.dll` calls `NtCreateDirectoryObject` with an absolute `\BaseNamedObjects\...` path, which bypasses the redirection and hits the global namespace access check.

So it isn't "MSYS can't reach a namespace" as a vague statement. It's a specific architectural mismatch: AppContainer redirects *Win32* object lookups into a per-container namespace, and MSYS2 sidesteps the redirection by calling the NT API directly with an absolute path. The creation then fails the global access check, because `SeChangeNotifyPrivilege` lets a sandboxed process traverse that directory but not create in it.

Which explains something I'd have got wrong otherwise. The symptom depends on how wide your policy is:

| Policy shape | What you see |
| --- | --- |
| Wider | `NtCreateDirectoryObject(\BaseNamedObjects\msys-2.0S5-...): 0xC0000022` — a visible access denial, naming the object |
| Narrower | Bare `0xC0000142`, no output at all |

Same root cause, two faces. If you go to reproduce this with a generous policy and see a different error than mine, you haven't found a different bug. And the visible face is the one that matters for diagnosis, because the quiet one tells you nothing.

Two more facts from the issue and its neighbours, both of which strengthen the conclusion:

**It is not tier-specific.** Reported on `appcontainer-dacl` as well as `base-container`. The object namespace boundary belongs to AppContainer itself, so picking a different containment tier does not help.

**There is a second wall behind the first.** A project that confined Git Bash with a restricted token, getting past the named objects entirely, then hit MSYS's signal pipe: `couldn't create signal pipe, Win32 error 5`. Two independent obstacles, and fixing the first does not imply the second goes away.

The design consequence is stronger than "bash is inconvenient in there":

**On Windows, no policy makes `bash` work inside the fence.** There is no grant to add, no deny to remove, no tier to select. A shell built on MSYS2 cannot initialize under an AppContainer identity, and MXC's Windows backend establishes exactly that identity. So `bash` cannot be contained. It can only be withheld or left outside.

That's why the tool inside the fence is `powershell`. Not because PowerShell is safer or more predictable, but because it is a native Windows process and native Windows processes start under AppContainer. The container's mechanics chose my shell for me.

And nine tools that could start a process outside the fence are withheld while the posture is `mxc`:

```
bash, grep, interactive_term, interactive_term_send, interactive_term_kill,
run_lifecycle, capabilities_apply, restart_mcp_server, load_attendant
```

Absent from the prompt. Absent from the picker. Refused at dispatch, with a message that says the tool is unavailable under the mxc posture and shouldn't be retried — distinct from the unknown-tool and filter refusals, so the model doesn't learn to try a different route.

One predicate answers "does containment withhold this name." Every site consults it. That's the part I'd fight for in review: a withheld list restated in four places is a list that will be wrong in three of them.

Two caveats, because this is exactly where I'd rather be corrected than confident.

**This is a Windows fact, not a fence fact.** The same `mxc-sdk` on Linux runs under Bubblewrap, and there `bash` works inside the fence — measured, not assumed: `bwrap` 0.13.1, separate mount and PID namespaces, a contained bash that reads granted paths and is refused ungranted ones. A future Linux runner can contain `bash` directly, and this whole section shrinks to a Windows footnote. The platform gate on the runner is the seam that allows it.

**"Cannot be contained" is not "cannot be reached."** Withholding `bash` removes the tool, not the capability. `powershell` inside the fence can call `bash.exe` and get the same failure for its trouble. The fence isn't fragile there; it's the same impossibility seen from a second entrance. Should a future Windows build run MSYS2 under a different identity, re-measure this first.

One last thing about this wall that cost me a detour: **learn mode cannot see it.** MXC classifies `\BaseNamedObjects` as an unsupported object type, outside its diagnostic model, so the denial is filtered out of the actionable report and survives only in verbose logging. A denial your authoring loop cannot record is a denial you debug by hand. Worth knowing before you go looking for it in a learning-mode report and conclude nothing happened.

Part one said the question deserved its own post once it had earned it. It has one now, and the answer is better than my guess: not "MSYS is unlucky in a sandbox" but a documented mismatch between how AppContainer redirects object lookups and how MSYS2 asks for them, with an open upstream bug tracking it.

## Everyone else chose it too

Here's the part that surprised me, because I'd assumed I was the only one.

**Every agent that ships on Windows made the same choice.**

Claude Code is the clearest, and it's explicit about bash being optional. Git for Windows is a *recommended* install, not a required one: "If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead." It exposes a separate PowerShell tool with its own documentation, and if you want bash it reads `CLAUDE_CODE_GIT_BASH_PATH`. If neither shell is found it refuses to start.

Codex goes further in the same direction. PowerShell is the **default** Windows session shell, not a fallback, and there's an open issue asking to *configure* bash because it isn't the default ([#16579](https://github.com/openai/codex/issues/16579)). The same issue reports the MSYS failure in MXC while native PowerShell keeps working.

GitHub Copilot CLI doesn't offer the choice at all. Its supported-OS list reads "Windows from within Powershell and WSL", full stop.

Three vendors, three implementations, one answer: **a PowerShell tool, and no bash shim anywhere.** No translation layer, no bundled POSIX runtime, no attempt to make the model believe it's on Linux.

That's worth sitting with, because the usual objection here is that models are steeped in bash, so a Windows-only shell tool should be a constant drag. I assumed so myself. Every model I worked with while building my own harness said it, more than once, and the point wasn't silly: the training data is mostly POSIX, and plenty of shell idioms from it don't exist here.

In practice it barely mattered. The models adapt to the tool in front of them, because the tool description is the contract, and a schema that says PowerShell gets PowerShell. The failures I actually hit were not "the model wrote bash" but "the model wrote PowerShell I didn't like," which is a different problem with a different fix. What nobody warned me about was the thing that actually bit: not the model's shell fluency, but my own assumption that the shell was a free choice.

The one that should give anyone pause is what Claude Code's own sandbox documentation says about the same machine I'm writing this on: "The sandbox runs on macOS, Linux, and WSL2. **On native Windows, Claude Code runs commands unsandboxed.**" The most widely used agent harness has no filesystem or network containment on native Windows at all, and its advice is to move to WSL2 if you want some. Codex has an MXC-backed sandbox but keeps it behind a preference flag and a build requirement. Copilot has one, PowerShell-only.

I want to be careful not to turn that into a boast, because it isn't one — a mature product that says plainly "unsandboxed on native Windows" is being honest, and it has years of scar tissue I don't. But it does reframe what this MVP is. It isn't catching up to a crowded field. On native Windows, almost nobody is doing OS-level containment for the shell yet, and the thing that's genuinely unusual here isn't the fence — it's withholding the tools it can't fence instead of quietly running them uncontained.

Which is precisely the trap part two warned about. My first attempt hit this wall and "solved" it by re-spelling commands through `cmd` to get anything to run — it worked, and it meant the container's calling convention had leaked into the model's contract. The model asked for bash, got cmd's syntax rules, and learned something true only because of my sandbox. The fix wasn't a cleverer translation layer but a boundary: one runner, one tool, and the model never learns which shell is inside. `bash` is simply gone while the posture is mxc.

Worth noting how the others avoid that leak, because it's the same trick and it took me a while to see why it works: the model isn't tricked into thinking it has bash. It's handed a tool whose description tells it the truth. Codex's issue proposes exactly this, updating the Windows shell tool descriptions "so they refer to the configured default shell rather than hardcoding PowerShell wording." No translation, no lying, no leaked calling convention — just honest tool surfaces.

Three major vendors, three implementations, and not one of them tried to pretend the shell was bash. That's usually the sign that the answer is the constraint talking and not a preference.

## What the policy grants, and what it denies

The default grants the user's home directory read-write. That's not conservative, and I'll defend it:

If you don't grant home, nothing works. Not `cargo`, not `npm`, not `git` — they all want `.cargo`, `.gitconfig`, `.rustup`, `node_modules` caches. A policy that breaks the tooling gets turned off within a day, and a fence that's switched off protects nothing. So home is granted, and then a fixed set of credential locations and jinn's own state are denied *inside* that grant:

`~/.ssh`, `~/.aws`, the jinn config directory, and the jinn data directory.

**A deny nests inside a grant, and the deny wins.** This is the shipped shape and it's the one that matters: home granted, sensitive subdirectories carved back out. The configuration accepts it; the backend matches deny before grant. I'll say more about how well I proved that below, because it's on the open list.

The environment is the other half, and it's the half I'd been putting off since part two. **The child no longer inherits your keys.** The contained process gets an allowlist snapshot taken at launch — `SYSTEMROOT`, `LOCALAPPDATA`, `PATH`, `TEMP`, `TMP`, plus anything you name in `env_passthrough` — with `inherit_default_environment = false`. MXC rejects a supplied environment that omits `SYSTEMROOT` and `LOCALAPPDATA`, which is a measured fact and a slightly funny one: the two names a Windows process cannot start without are not secrets and never were.

Network: no policy supplied, so egress is denied by MXC's default. That closes the `curl`-something-out case and *does not close* the conversation case from part two. Nothing here stops a contained command reading a file and putting it in a tool result. Read-only home is one prompt injection away from disclosure, and I know it.

Learn mode exists and records denials to a report under jinn's data directory, appended to the tool result as a summary. Its honest scope is narrower than the position paper's vision: with home granted, what it records is *violations of the deny list*, a command that trips `~/.ssh`. That's drift detection ("something just tried the credentials"), not the least-privilege authoring loop I described in part two. The report directory sits under the jinn data deny above, so a contained command can neither read the reports nor silence them.

`read_image` gets its own small rule: under `mxc`, it never starts ImageMagick. Natively decodable images attach as always; anything needing conversion fails with a message naming the posture. ImageMagick on model-influenced input is a long exploit history, and part two listed it as "the cheapest real win." This is the cheap version of that win.

## The bug that every test missed

Here is the part I'd write the post for even if nothing else happened.

The suite was green. Thirteen tests against the real MXC SDK, proving a contained command writes in its cwd, reads elsewhere in home, is denied `~/.ssh` with nothing left on the host afterwards, is denied reads outside every grant, cannot resolve a network host, sees an empty value for a non-allowlisted variable, and passes through a non-zero exit code. `just test` at 8,939 passing. `just lint` clean.

Then I ran the built binary from my own terminal and it refused to start:

```
error: [containment] posture = "mxc" needs SYSTEMROOT in jinn's environment,
but it is not set
```

`SYSTEMROOT` was set. I could see it in the environment.

The cause is four lines of ordinary Rust:

```rust
for required in baseline.required {
    if !inputs.environment.contains_key(*required) {
        return Err(ContainmentError::MissingBaselineVariable { variable: required });
    }
}
```

`inputs.environment` is a `BTreeMap<String, String>` built from `std::env::vars()`. A `BTreeMap` lookup is case-sensitive. Windows environment names are not. The session manager stores the name as `SystemRoot`; the registry has `windir`; my policy table says `SYSTEMROOT`. Three spellings of the same thing, and one exact-match lookup that only accepts the one the OS didn't use.

Why did every test pass? Because every test injected its own environment map, and every one of those maps was written with the uppercase spelling to match the table. The tests were checking my arithmetic, not the OS. And why did *my* first several attempts to reproduce it also pass? Because I was launching from Git Bash, and MSYS uppercases every variable name it imports. My reproduction harness was silently correcting the bug it was supposed to expose.

That's the lesson, and it's a nastier version of part two's: **a test fixture is a map, and a map is not the territory.** Thirteen tests of real MXC, and not one of them exercised the actual boundary between my code and the operating system. The fixture was where the bug lived.

The fix is a helper that compares exactly first, then case-insensitively on Windows only:

```rust
fn variable<'a>(
    environment: &'a BTreeMap<String, String>,
    platform: Platform,
    name: &str,
) -> Option<&'a String> {
    if environment.contains_key(name) {
        return environment.get(name);
    }
    match platform {
        Platform::Windows => environment
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value),
        Platform::Linux | Platform::MacOs => None,
    }
}
```

Linux and macOS keep exact matching, because their environments are case-sensitive and I don't want to teach the code a Windows habit it will carry somewhere it's wrong. Three tests came out of it, and the one that matters is the one that would have caught the original bug in the first place.

The wider point, which is worth more than the fix: **this bug made `mxc` unlaunchable from Windows Terminal, Explorer, or cmd — which is most users.** All that real-MXC test coverage, and the mode was dead on arrival for everyone who didn't happen to launch it from a shell that uppercases its environment for its own reasons. You cannot test your way out of that with unit tests alone. Somebody has to launch the thing.

## What I proved by running it

After the fix, I ran the real binary with `posture = "mxc"` and had a look at the fence from the inside, from a contained shell:

- `LOCALAPPDATA` rewritten to `...\Packages\sandbox.{guid}\AC`, with a different GUID per invocation, so each call gets its own container. The environment was reduced to exactly the seven allowlisted names, with no `USERPROFILE`, `APPDATA`, `COMPUTERNAME`, or `USERNAME` anywhere in it.
- Filesystem drives not enumerable; CIM unavailable; `C:\Utilities` denied outright.
- `~/.ssh`, `~/.aws`, and jinn's own `Roaming\jinn` all refused with `UnauthorizedAccessException`.
- The working directory listing fine.
- `bash` gone from my own tool list. That one's the most satisfying, because it's the fence confirming itself: the harness withheld a tool from the model, and from me, because of a posture in a config file.

I also wrote a file into home and deleted it, which confirmed two things at once: home is genuinely granted read-write, and cleaning up after myself still works inside the fence.

One thing I did *not* expect: `C:\Windows\win.ini` was readable, and `C:\Windows` enumerated 114 entries. That's probably correct — the container needs system files to start a process at all — but I can't tell "system paths" from "user data" from inside, and no test pins it. It's on the list.

## What's still open

Honest list, as promised:

- **The file-tool guard still doesn't exist.** This is the big one, and it's the ordering mistake from the top. K1 (exfiltration through the conversation) and K2 (delayed execution through a written file) are *untouched* by everything above. The doc says so plainly. What shipped is the process column, and only for one tool.
- **The withheld list is the honest workaround, not the fix.** Withholding `interactive_term` under enforce is the position paper's option 1 and the only one that keeps "enforce" true. It's also a capability loss, and I want the terminal inside the fence eventually. Same for `bash`: it isn't withheld because the policy dislikes it, it's withheld because on Windows it cannot exist in there at all.
- **Learn mode is drift detection, not authoring.** Home granted means the reports show deny violations, not "here's what your workload needed." Getting the broad loop needs the guard and a narrower default. It also cannot see the one denial that would have explained the whole bash problem: `\BaseNamedObjects` is an unsupported object type, filtered out of the actionable report.
- **The environment allowlist is a floor, not a policy.** Seven names is what makes a Windows process start. It says nothing about which of your secrets a build legitimately needs.
- **Nested-deny-wins is proven in two halves.** A deny is unreadable and a sibling file in the same grant is readable — but never in the same run. The criterion wanted one demonstration and I have two.
- **`external` is unverified by construction.** That's the point of it, but it means a typo'd posture is indistinguishable from a real dev container in the logs.
- **The MSYS2 story is upstream's now, but only partly mine.** The failure code, the `cmd.exe` contrast, and the two symptom faces are measured on my machine; the named-object mechanism comes from Microsoft's triage in [mxc#1061](https://github.com/microsoft/mxc/issues/1061) rather than from instrumentation. I still have not watched the namespace call fail. And the bug is open, assigned, and had one workaround attempt fail — so this may stop being true, in either direction.
- **Linux doesn't exist yet.** The tables are there, the code is `#[cfg(windows)]`, and it has never been built or run on a non-Windows target. Stated as an unexecuted check, because that's what it is. When it lands, `bash` comes back inside the fence and this post's middle section stops applying.

## About the reordering

Back to the thing I opened with. The plan said guard first, sandbox fourth. I built the sandbox. The position paper's own coverage table says a process sandbox closes the process column and leaves K1, K2, and K3 open, and I shipped a process sandbox.

I don't think that was wrong, exactly. The MXC runner was the piece where the unknowns were largest: does the SDK do what the docs say, does the environment allowlist actually work, does a deny nested in a grant behave. Those were questions no amount of reading would answer, and answering them changed the shape of everything downstream. The guard is the piece where the design is obvious and the work is tedium. Doing the uncertain thing first, while my attention was fresh, was the better order for me even though it was the worse order for the threat model.

But it needs saying in these words: **the fence is up and the two kill chains it doesn't touch are the two I said were the whole game.** Calling this post "the fence goes in" without that sentence would be exactly the kind of grinning-with-swamp-land-in-my-pocket honesty failure that part two was about. The mode is called enforce. For the one tool it covers, it enforces. For the effects that actually cost you your credentials, it is a fence around the wrong field.

The docs say that. The withheld list says that. And this paragraph says that.

## Next

The guard. Canonicalizing, symlink- and junction-aware resolution confined to policy grants, consulted by every filesystem tool, denied writes surfaced as visible tool errors. No sandbox dependency, every platform, closes K1 and K2. It should have been first and it's next.

After that, the interesting one: the harness's own authority. `capabilities_apply` and writes to `jinn.toml` let the model widen its own fence across a restart, and no runner can stop them, because the socket jinn opens on the model's behalf isn't in a container. That's K3, and it's a design problem rather than an implementation one, which is why I keep circling it.

*The comment box is open, as always. If you know why `GetFinalPathNameByHandleW` with `VOLUME_NAME_DOS` fails inside tier-1 containers — the paper mentions it and it's still inference rather than understanding — I'd like to hear it.*
