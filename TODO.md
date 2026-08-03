# TODO — v0.2.0

What is deliberately left for the next version, and enough of the reasoning to
pick it up cold. Everything here was measured rather than guessed; where a
number appears, it came from a run on a real machine.

---

## 1. The scriptlet library is incomplete

**Status:** the `+js(...)` path works. It did not until 2026-08-03.

### What was wrong

Every `+js(...)` rule in the filter lists resolved to nothing — 2,856 of them —
and had done for as long as the feature existed. The cause was one field.
`scriptlets.rs` labelled all 144 entries parsed from uBlock Origin's library as
`MimeType::FnJavascript`. Brave's engine treats that kind as a _helper_ that
other entries may depend on, and refuses to invoke it from a rule. Only entries
whose name ends `.js` are invokable; `.fn` files are the helpers.

Measured on the real library after the fix — injected script for a page:

| Page        |   Before |    After |
| ----------- | -------: | -------: |
| YouTube     |  3,585 B | 66,557 B |
| reuters.com | 18,372 B | 29,386 B |
| reddit.com  |    756 B | 13,431 B |

Two tests now guard it, including one that asserts a rule naming a scriptlet
actually produces injected script. The unit assertion alone can pass while the
engine still injects nothing, which is exactly how this hid for so long.

### What is still missing

**Four dependencies are absent**, so any scriptlet needing them is dropped
whole: `arglist-parser.fn`, `jsonpath.fn`, `range-parser.fn`, `urlskip.fn` — 17
unmet references in total. Only `scriptlets.js` is downloaded, and uBO splits
its library across roughly 31 files.

_Shape of the work:_ add the missing resource files to `sources.rs` and fetch
them alongside the main library. The parser already handles both registration
syntaxes; nothing else should need to change.

**108 rules use `trusted-` scriptlets**, which Brave's engine refuses by design.
27 of those are YouTube's. That is not a bug to fix — see the next item.

---

## 2. YouTube pre-rolls on a directly-opened video

**Status:** half working, and the README says so plainly.

Opening a video _from another video_ usually has no ad. Opening one **directly**
— a link, a bookmark, the app's start page — usually still plays a pre-roll.
Confirmed by watching a real video, not by reading code.

### Why

YouTube delivers the player response three ways. `site_scripts.rs` hooks two of
them — `JSON.parse` and `Response.prototype.json` — and strips the ad keys from
both. The third is `ytInitialPlayerResponse`, inlined into the page's own HTML
on a fresh load. That is script _source_, not parsed JSON, so neither hook ever
sees it and `adPlacements` survives.

uBlock Origin solves this with `trusted-replace-xhr-response`, one of the 108
trusted scriptlets the engine will not run.

### What was already tried and did not work

A plain `Object.defineProperty` accessor on `window.ytInitialPlayerResponse`.
It leaves the watch page **dead** — no body, nothing renders. Verified by A/B
against the same video and probe that work fine without it, twice.

Worth knowing: an earlier session removed the same hook and recorded the cause
as a misdiagnosed DMABUF crash. That note was wrong — the hook genuinely breaks
the page — and the source now says so, to stop it being tried a third time.

### Two routes

**A — write it ourselves (recommended, smaller).** Implement
`override-property-read` properly. The naive version failing is almost
certainly the implementation, not the approach: Vivaldi ships a working one.
Start by finding out _why_ it breaks the page — read the console error — rather
than assuming it cannot be done.

**B — adopt eyeo's snippets (larger).** Vivaldi ships eyeo's Anti-Circumvention
Snippets at
`/opt/vivaldi/resources/vivaldi/adblocker_resources/injectable_resources.json`
(`abp-main.js`, ~46 KB), licensed **GPL-3.0** — compatible with this project's
GPL-3.0-or-later. They are generic primitives, `json-prune` and
`override-property-read` among them, invoked by ABP `#$#` filter syntax. Brave's
engine only parses uBO's `##+js()`, so this needs a syntax mapping layer as well
as the library.

_Correction worth keeping:_ Vivaldi's **matching** engine is Brave's
`adblock-rust` — the same crate this project uses. Swapping engines would change
nothing; the snippet layer is the entire difference.

---

## 3. Smaller things

- **Two UI fixes were never seen working.** The download panel's scrollbar no
  longer covers the _show in folder_ button, and the hover toolbar's dead space
  is gone. Both compile and both are reasoned, but revealing the toolbar needs a
  real pointer and the panel needs a click, so neither was confirmed visually.
- **`checkJs` is off for the Node scripts.** `tsconfig.node.json` used to
  typecheck `scripts/*.mjs` against their JSDoc annotations. Comment stripping
  removed the annotations, so the checking was turned off rather than left
  emitting implicit-`any` noise. Restoring it means writing real type
  declarations, not comments.
- **Eight checks need a person on a real desktop** — taskbar identity, tray and
  autostart, a real account sign-in, camera prompts, Firefox import with Firefox
  running, and the extension popup. They are listed in `x11test.md`.

---

## Not being done

macOS, Windows code signing, and an auto-updater. All three are decisions, not
omissions: macOS is a second implementation rather than a port, signing costs
money for a free tool, and reinstalling is the update path — which is what lets
the app say honestly that it never calls home.
