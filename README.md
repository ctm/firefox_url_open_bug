# A Firefox bug, I think

This repository illustrates what I think is a bug in Firefox on MacOS,
although there's certainly the chance that I've overlooked something.

All my development and testing has been done on MacOS running 10.15.3.
Other people are using the software on a variety of platforms and
browsers, but I don't know which (if any) are encountering the bug.

## Background

I'm developing a web client for a poker server I wrote.  I'm doing it in
Rust, using the [yew](https://github.com/yewstack/yew) framework.  A
couple of days ago I noticed a Firefox specific glitch.  When a
tournament starts, the player should get a pop-up window with the
table and the lobby should show the new table that was created.  That
works fine in all the other browsers I've tested, but _sometimes_ the
lobby would not show the new table in Firefox.

After a day and a half of debugging, I have this test-case that fails 100%
on Firefox and works 100% on Safari, Chrome and Brave.  This test-case does
not use yew, but also isn't quite the same as what I do in yew.  Specifically,
in this test-case I open the pop-up in the callback that receives the data
from the WebSocket, but in yew, I open the call-back in the update method
that is run after the callback has finished.

My yew-based test case (not included here) runs fine in Firefox 55.0
(and 55.0.3), but fails in Firefox 56.0b1, so it appears that this bug
crept in starting with Firefox 56.0.  However, I do not know why, but
the test software here doesn't run at all in Firefox 55.0.

If it would help, I can add the yew-based test case to this
repository.  Currently it's part of my private poker repository and
had a ton of things in it that aren't relevant to this bug.

I do not normally program at the wasm-bindgen level and I know
exceedingly little about JavaScript in general, much less using
WebSockets in JavaScript.  As such, I can't be sure that what I'm
seeing is definitely a bug (but remember, the problem occurs even when
I open the pop-up after the callback completes) or that it's not a bug
in [wasm-bindgen](https://docs.rs/wasm-bindgen/0.2.59/wasm_bindgen/),
but I do know the difference in behavior is Firefox specific.  I
_believe_ that what I'm doing is sufficiently uncommon that such a bug
could have been around for a couple of years and the fact that what
I'm doing (in yew) used to work in Firefox 55.0 makes me think Firefox
is the culprit.

## How to Reproduce

### Prerequisites

You need to [install Rust](https://www.rust-lang.org/tools/install) and
[install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

### Build and start the modified echo server

In a terminal, type `cargo run`.  It should look something like this:
```
$ cargo run
...
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/server`
```

### Build and start the client

In a separate terminal, install the binaries from the https package using
`cargo install https`.  It should look something like this:
```
$ cargo install https
    Updating crates.io index
...
   Compiling rfsapi v0.1.0
    Finished release [optimized] target(s) in 1m 02s
  Installing /Users/ctm/.cargo/bin/http
  Installing /Users/ctm/.cargo/bin/httplz
   Installed package `https v1.9.1` (executables `http`, `httplz`)
```

Now build and launch the client using `wasm-pack build --target web &&
http`.  It should look something like this:
```
$ wasm-pack build --target web && http
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
...
    Finished release [optimized] target(s) in 28.51s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: ✨   Done in 28.62s
[INFO]: 📦   Your wasm pkg is ready to publish at ./pkg.
Hosting "." on port 8000 without TLS and no authentication...
Ctrl-C to stop.
```

In the browser you want to test, go to the url
[http://localhost:8000](http://localhost:8000) if pop-ups are blocked,
re-enable them and reload. You should get two Duck Duck Go tabs or
windows and the Web (or JavaScript) console should say:

```
socket opened websockets.js:268:17
message successfully sent websockets.js:268:17
message event, received data: "ping" websockets.js:268:17
```
and the ping received message should either appear twice or it should appear
once with an annotation that says it was produced twice. If you see the
following, you're running into the bug.
```
Error: closure invoked recursively or destroyed already
```
