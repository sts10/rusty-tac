# Rusty Tac

I'm writing a command line tic-tac-toe game as I try to learn Rust! It seems to work OK as of this writing, but wow, this was hard

## Some Notes on My Plan

To check for a winner, I'm using a second array called `sums` that adds up each of the possible wins in the game of tic-tac-toe. (Fun fact: I used this idea (and drew the sketch below) back in 2013 as part of my admission test to The Flatiron School.)

![sums explained](img/map.png)

## About The Process

Woof, just writing Rust that compiles is hard. 

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

- Derek Banas video: https://www.youtube.com/watch?v=U1EFgCNLDB8


### Running a Simple Rust program with rustc rather than Cargo

Create `hello_world/main.rs`:

```go
fn main() {
    println!("Hello, world!");
}
```

Save it, and then in the shell run:

```bash
rustc main.rs
./main
Hello, world!
```

[Reference](https://doc.rust-lang.org/book/second-edition/ch01-02-hello-world.html#writing-and-running-a-rust-program)

### Running Rust Programs with Cargo


If you want to use Rust's package manager, [Cargo](http://doc.crates.io/), for your project:

Spinning up a new project:
```
cargo new hello_cargo --bin
cd hello_cargo
```

Building and running an executable:
```
cargo build
./target/debug/hello_cargo
```

Build _OR_ run:
```
cargo run
```


[Reference](https://doc.rust-lang.org/book/second-edition/ch01-02-hello-world.html#building-and-running-a-cargo-project)

I used [rustup](https://rustup.rs/) to manage my versions (or channels) of Rust. 

For my fellow Rubyists, `rustc` is like `ruby`, while [rustup](https://rustup.rs/) (easy install using `curl`) is like `rvm` or `rbenv` (version manager). The versions-- called "channels"-- of Rust that you can manage with `rustup` are stable, beta, and nightly. I did all my work for this project with stable.



### Issues I Ran Into

Rust is crazy. It's compiler is crazy strict and, I'd argue, opinionated. 

As [Dunham says](https://youtu.be/FMqydRampuo?t=4m35s), "Rust has a high priority on safety and performance, so if you've ever managed memory before, you'll know it's easy to make mistakes." 


She [also compares](https://www.youtube.com/watch?v=FMqydRampuo?t=5m38s) Rust to C and other languages in regard to garbage collection and safety: 

- C: "Just follow these rules perfectly, you're smart"
- Java, JS, Ruby, etc.: "Wait a minute, I'll take care of it"
- Rust: "I'll prove correctness at compile time."

Clearly, Rust is way "closer to the metal" than Ruby or JavaScript. But unlike C, its compiler is way more picky. In my very little experience, I found Rust way pickier than Golang. 

I could almost _feel_ Rust's compiler trying to guide me to write my program the way that it wanted me to. It was almost as if my program had already been written by the compiler, and it was nudging me toward that ideal. Rust's compiler even gave me a "warning" (not an "error") when I used camelCased variables rather than snake_case. However the much more frustrating instances of this were obviously when it threw errors and wouldn't compile-- basically it was telling me "This is bad enough that I won't even compile this for you."

[Later in Dunham's talk she said](https://www.youtube.com/watch?v=FMqydRampuo?t=23m50s) something that stuck with me as I kept running into error after error:

> The compiler wants to see your code do things right. Rust wants you to succeed. My mental image of it is that you're apprenticing under some really knowledgeable old hacker who worked on mainframes in the '70s. And they're going to tell you, "Hey, I know this works right now, but it's going to get you in trouble later."

Here are some concrete examples of this.

#### The `player` variable

Here's a "soft" example of the compiler nudging me. In my tic-tac-go game, there's a simple part where we alternate between player 1 and player 2.

So here's what I initially wrote:

```rust
// for this example, let's arbitrarily set turn_number to 6
let turn_number = 6; 

let mut player = 0;
if turn_number % 2 == 1{
  player = 1;
} else {
  player = 2;
}

println!("Player {}'s turn", player);
```

That line `let mut player = 0;` is problematic, the compiler told me. First, know that, by default, Rust variables are immutable(!), meaning their value can't be changed. You have to use the keyword `mut` if you want to change the value at some point later on.

Rust compiler gives me a warning here: "warning: value assigned to `player` is never read". It's basically saying, "why did you assign `player` to 0 when you never use that value 0?".

When I edited that line to `let mut player;`, a new warning appeared: "warning: variable does not need to be mutable". This was a little harder for me to understand, but it makes sense once I realized that, for each time `player` is declared in its scope, it's only ever assigned one value one time (either `1` or `2`, based on this conditional). Thus it never needs to _mutate_ from one value to another value.

Here's what the Rust compiler and I ended up with (no errors, no warnings):

```rust
// for this example, let's arbitrarily set turn_number to 6
let turn_number = 6; 

let player;
if turn_number % 2 == 1{
  player = 1;
} else {
  player = 2;
}

println!("Player {}'s turn", player);
```
