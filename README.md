# An Firefox bug reported in May 2012 makes WebSockets flakey

This repository illustrates a bug in Firefox (at least on MacOS).
Specifically, I think [this eight year old
bug](https://bugzilla.mozilla.org/show_bug.cgi?id=758004) is the
cause, although it got worse two and a half years ago with the relase
Firefox 56.0.

All my development and testing has been done on MacOS running 10.15.3.

## Background

I'm developing a web client for a [poker
server](https://github.com/ctm/mb2-doc).  I'm doing it in Rust, using
the [yew](https://github.com/yewstack/yew) framework.  A couple of
days ago I noticed a Firefox specific glitch.  When a tournament
starts, the player should get a pop-up window with the table and the
lobby should show the new table that was created.  That works fine in
all the other browsers I've tested, but _sometimes_ the lobby wouldn't
note that the new table had been created.

After two days of debugging, I have two test-clients that fail 100% on
Firefox and works 100% on Safari, Chrome and Brave.  One test-client
does not use yew; the other does.  In the non-yew test-client I open
the pop-up in the callback that receives the data from the WebSocket.
In the yew test-client, I open the call-back in the update method, not
the callback, but I strongly suspect that what is happening is that
the data gets back to the thread where update proceeds to run before
the closure that returns the data itself has completely finished.

A different yew-based test client (not included here) runs fine in
Firefox 55.0 (and 55.0.3), but fails in Firefox 56.0b1, so it appears
that this bug crept in starting with Firefox 56.0.  Furthermore, if I
change one line and make it call an alert, rather than create a
pop-up, we get the exact same failure in Firefox 55.0 as we do now.
So that explains why an almost eight year old bug wasn't manifesting
itself two and a half years ago.

Unfortunately, and I don't yet know why, neither test client here runs
on Firefox 55.0.  The biggest difference that I can think of is that
for simplicity the test clients here use wasm-pack, while my other one
(which is really just a super hacked copy of my poker client) uses
web-pack.

If it would help, I'm willing to try to get the other test client
hacked out of my private poker repository and included in here.  Right
now it has a ton of things in it that aren't relevant to this bug.

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

### Build and start the wasm_bindgen client

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

Now cd into `clients/wasm_bindgen`, build and launch the client using `wasm-pack build --target web &&
http`.  It should look something like this:
```
$ cd clients/wasm_bindgen
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
re-enable them and reload. You should get **two** Duck Duck Go tabs or
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
### or Build and start the yew client

The yew client is in `clients/yew`, otherwise rujning it and its
behavior is very similar to the wasm_bindgen client.  When it runs
successfully you should get **two** Duck Duck Go tabs or windows and
there should be no complaint in the JavaScript console about closures.

## Ramifications

This bug took me a while to even notice, much less find.

Now that I know what's going on, I can work around it, e.g., by
telling the server that it can't send any messages down a WebSocket if
a message has been sent that can result in a pop-up.  Instead, all
such outgoing messages have to be buffered at the server and only
released after receiving an incoming message from the client after the
danger is gone (although I'll have to further experiment to even know
that.  I'm hoping everything is safe once I have the handle to the
pop-up).

Unfortunately, as the Rust WASM ecosystem grows, other people are
going to run into this problem too.
