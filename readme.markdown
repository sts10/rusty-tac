# Rusty Tac

I'm writing a command line tic-tac-toe game as I try to learn Rust! It seems to work OK as of this writing, but wow, this was hard

## Some Notes on My Plan

To check for a winner, I'm using a second array called `sums` that adds up each of the possible wins in the game of tic-tac-toe. (Fun fact: I used this idea (and drew the sketch below) back in 2013 as part of my admission test to The Flatiron School.)

![sums explained](img/map.png)

## About My Process

Woof, just writing Rust that compiles is hard. 

**Here is [a long blog post I wrote about making this little game](https://sts10.github.io/2017/11/18/trying-go-and-rust.html)**

### Resources consulted

- [Rust homepage](https://www.rust-lang.org/en-US/index.html) and [Official Rust blog](https://blog.rust-lang.org/)
- [The Book (2nd edition)](https://doc.rust-lang.org/book/second-edition/ch01-02-hello-world.html) -- 2nd edition of Rust's official documentation. In hindsight, I really should have gone through it more slowly before attempting to make this tic-tac-toe game.
- [Rust 101 Linux talk video by E. Dunham](https://www.youtube.com/watch?v=FMqydRampuo) -- This was a _great_ video for learning about the language and community.
- [rustlings](https://github.com/carols10cents/rustlings) -- Helpful little games!
- [Rust playgorund](https://play.rust-lang.org) -- Allows you to run Rust in a browser-- ironically, it worked better in Chrome than Firefox 57 for me 
- [Rust by Example](https://rustbyexample.com/) -- Seems like some unofficial documentation, so I was a little reluctant. But to be frank there isn't much documentation out there, so I took what I could find. 
- [Intro to Rust video](https://www.youtube.com/watch?v=agzf6ftEsLU) -- Another video, focusing more on Rust's concept of ownership.
- [#rust-beginners IRC channel](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners) -- At one point I was so stuck I made [a share-able playground link](https://play.rust-lang.org/?gist=40257dc021809a8c8a6750ab2f133a8a&version=stable) and hopped into the #rust-beginners IRC channel. Even though there were only a few people active on a Friday morning, I got insanely concise help I needed. 

![My IRC chat](img/irc-chat.png)

- [Subreddit](https://www.reddit.com/r/rust/)

- [Derek Banas video](https://www.youtube.com/watch?v=U1EFgCNLDB8) was also helpful

