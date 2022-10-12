# Summer of Rust Lab 2

In this lab, we're going to practice a bit from the [third chapter of the Rust
book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).

## Welcome to Rabbit University!

Hello! 🐰 Here at Rabbit University, we take great pride in our esteemed 🐇
students. We have the best grading system in the world, and we can prove it with
our Rabbit University Safety Testing Suite (RUSTs). But before we can do that,
we need you to get it running for us!

For this lab, you'll implement three functions, each for a different test. You
shouldn't need to do anything outside of these functions, but don't let that
stop you if you want to try something!

Since this is the first lab, there might be some bugs. Please reach out to
Forest if this is the case 👍

### 1. Validate the grading system

The first test is making sure the grading system is working. The equation for a
student's grade is as follows:

`(carrots + nuts) * (carrots + nuts + seeds)`

To do this, you'll need to complete the `calculate_grade`. In it, it shows
`todo!()`. This is so the compiler knows that you'll work on the function later,
and doesn't complain if there isn't a return value.

### 2. Validate the safety system

The second test verifies that the safety system is in check.

A student is considered safe if:

- There are no wolves nearby
- It is daytime
- Either the above, or the student has a carrot
- Either the above, or the student has more than 3 friends nearby

For this part, fix the `calculate_safety_status` function. Note, the function
signature for this one is longer, and so Rust format has broken it up into
multiple lines.

```rust
fn calculate_safety_status(
    wolves_nearby: bool,
    day_time: bool,
    has_carrot: bool,
    friends_nearby: i32,
) -> bool {
    todo!()
}
```

A shorter form of this function might look like this:

```rust
fn calculate_safety_status(w: bool, d: bool, h: bool, f: i32) -> bool {
    todo!()
}
```

### 3. Validate the simulation

Finally, we need to make sure our simulation is working as expected. For it, we
use a simple mathematical model that [cannot be solved(?!?
Clickbait?)](https://www.youtube.com/watch?v=094y1Z2wpJg). The simulation tracks
the number of rabbits every day under the situation that every day is unsafe for
students. In the end, there will only be one rabbit left.

Every day, there are two options. If there is an odd number of rabbits, then the
rabbits "multiply like wild", specifically by three. Then one more rabbit shows
up. Specifically, there will be `(3 * rabbits) + 1` rabbits the next day.

On the other hand, if there is an even number of rabbits, then that's the
perfect number for the wolves to eat! The next day, there will be half as many
rabbits. Specifically, there will be `rabbits / 2` rabbits the next day.

This function should return the number of days that pass before there is only
one rabbit left.

### 4. Make sure the tests pass

Now that everything is done, check that the tests pass with `cargo test`.

After everything looks good, you should also run `cargo fmt`. This might already
happen when you save your file, but it will format your code for you.

If that all works, then you're good to go! Commit that code and watch those
tests pass 😎

### 4.1. Bonus

Finally, one more cool thing to try is running `cargo doc --open`. This will
compile documentation from your code, and open it in your browser. You'll notice
that functions have descriptions beside them. These actually come from comments
in the code! When I add a comment above a function like `/// <comment>`, it will
get rendered into the docs.

![](https://media.discordapp.net/attachments/444005079410802699/970818200222720060/unknown.png)

This makes it really easy to get an overview of a codebase. In fact, this is how
you look at docs from other Rust crates! But instead of having to run `cargo
doc` on each one, you can go to [docs.rs](https://docs.rs), where each public
crate has its documentation automatically generated.

Another cool thing about Rustdoc is that you can see the source code of anything
mentioned. To try this, go into `validate_safety_system` and click `source` in
the top right. Pretty cool!

![](https://cdn.discordapp.com/attachments/444005079410802699/970819886328733786/unknown.png)

If you want to learn more about Rustdoc, here's a [link to check
out](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/documentation.html).

### That's all!

See you next week 🏖️

## License

The Summer of Rust Labs is duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))