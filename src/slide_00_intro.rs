//@ # The Concurrent State of Rust
//@
//@ A look at various techniques for handling concurrent operations.
//@
//@ ## A Little About Me
//@
//@ Chris Griffing
//@
//@ Full Stack Web Developer in Bellevue at Fresh Consulting
//@
//@ ### Rust Novice Alert!!!
//@
//@ I have a small node.js api I am rewriting in various backend languages. So far, Rust and Go. Elixir(/Phoenix) and Swift are next on my list.
//@
//@ <div class="notes">
//@
//@ - Mostly Javascript, but I still dip into PHP (especially lately... anybody hiring?)
//@ - I want a language I can move onto from JS for highly performant backend tasks. 
//@ - Learning a language I can also use for systems programming is hugely ideal. That is part of the allure of JS, usage in multiple environments accross as many platforms as possible.
//@ - And yes... server side Swift is a thing. I have not tried it, but at a meetup I met someone who convinced me to give it a shot.
//@
//@ </div>
//@
//@ ## Tango
//@
//@ Presentation made with Tango
//@
//@ [https://github.com/pnkfelix/tango](https://github.com/pnkfelix/tango)
//@
//@ Markdown to .rs file conversion and back. The Rust lives in the markdown in a code block, the markdown lives in the rust code as specially formatted comments.
//@
//@ <div class="notes">
//@
//@ - One issue. You have to be sure to run 'cargo build' when you want to switch from editing markdown files to rust files, or vice versa. This is to allow the build to synchronize the files.
//@
//@ - I used a watcher to run cargo build when any markdown or rust file changes. (nodemon just because of familiarity) cargo-watch would need to be custom compiled to watch the markdown files.
//@
//@ - Then pandoc to convert markdown to html. (Also in my watcher).
//@
//@ </div>
//@
//@ ## Some History
//@ "When Rust first began, it baked channels directly into the language, taking a very opinionated stance on concurrency." -Aaron Turon
//@
//@ [https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)
//@
//@ "... we had M:N threading for a long time and went to great pains to remove it. ..." -Patrick Walton
//@
//@ [https://news.ycombinator.com/item?id=11369170](https://news.ycombinator.com/item?id=11369170)
//@
//@ <div class="notes">
//@
//@ - M:N threads === Green Threads === Coroutines === Goroutines
//@ - Green threads are the default of Go and Haskell.
//@ - In today’s Rust, concurrency is entirely a library affair; everything described in this post, including Send, is defined in the standard library, and could be defined in an external library instead.
//@ - I don't specifically recognize Patrick Walton, but he appears to be a dev at Mozilla and his github profile has personal Rust repos dating back as far as 2012. Good enough for me.
//@
//@ </div>
