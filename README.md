Advent of Code 2022 in Rust
===========================

If you've never heard of Advent of Code, you're missing out. Check it out now at [adventofcode.com](https://adventofcode.com/about)!

I mostly do AoC puzzles for fun, generally coding in Python, but I've also found that it's a great way to get some experience with other programming languages, such as Julia or Rust.

In this repository, I plan to solve all [AoC 2022](https://adventofcode.com/2022) puzzles using Rust.

As I'm still learning Rust, the code may not always be as simple or idiomatic as it could be. If you have suggestions for things I could improve, please don't hesitate to file an issue or submit a PR. Thanks!

I've also included a little `get_data.py` utility to automatically download the data of the day, at the right time (you'll get a countdown if you're early). I was too lazy to code it in Rust, but perhaps I'll port it one day. To use it, just type the following command in a terminal, replacing `{day}` with the day you want:

```
cd /path/to/this/repository
python get_data.py 2022 {day}
```

The first time you run this tool, it will ask you to login to AoC in your browser, and save your session cookie into a `.session` file in the current directory.

Have fun!

