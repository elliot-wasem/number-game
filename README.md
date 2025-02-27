# Number Game
## Description

This game should look familiar to you. It is a 16-tile puzzle game where you need to arrange the numbers in order. The game is played on a 4x4 grid with tiles numbered from 1 to 15 and an empty space. The objective of the game is to arrange the numbers in order by making sliding moves that use the empty space. The game is won when the numbers are arranged in order from 1 to 15, with the empty space in the lower right corner.

The layout looks as follows:

```
+----+----+----+----+
| 01 | 02 | 03 | 04 |
+----+----+----+----+
| 05 | 06 | 07 | 08 |
+----+----+----+----+
| 09 | 10 | 11 | 12 |
+----+----+----+----+
| 13 | 14 | 15 |    |
+----+----+----+----+

```

## Instructions

Use the arrow keys, or H/J/K/L to move pieces into the empty space. The game will automatically detect when you have won and display a message. You can also press `q` to quit the game at any time.

## Building

To build the game, you will need the Rust programming language installed as well as the `ncurses` development library.

## Running

Once the game has been compiled, you can run it from the command line. Either run `cargo run`, or run the compiled binary directly. The `ncurses` library will be statically linked, so feel free to distribute a compiled copy to your friends!
