A simple PRNG based on the idea of [Squirrel Eiserloh presented at GDC 2017](https://youtu.be/LWFzPP8ZbdU)
of taking a hash function, a counter and a seed as an RNG.
Using hash functions found by [Chris Wellons](https://nullprogram.com/blog/2018/07/31/).

The RNG takes a counter, hashes it, xors with the seed and hashes again.

**I make no quality guarantees**, but the generator is lightweight and 
it is trivial to skip to any point in the sequence.
I wrote it for games in which you want to generate random events deterministically
and jump back and forth in them at will.
