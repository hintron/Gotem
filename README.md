# got-em

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.


Notes:

Specs Docs: https://specs.amethyst.rs/docs/tutorials/01_intro.html

A system is nothing more than a function that runs once each frame and
potentially makes some changes to components.



Improvements (Note: this is with the 0.13 docs while running with 0.12):

* Explain the whole `#` thing.

* What's with the use amethyst::<Error> thing in the #?

* Some of the # are wrong.

* The specs links are dead. Point to new links.

* amethyst::core::SystemDesc dne! (0.13 new feature!)

* let mut world = World::new(); is not needed