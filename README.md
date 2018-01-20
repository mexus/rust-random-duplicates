# rust-random-duplicates
Random generator duplicates test

In this simple test I generate random integers (`u32`) in multiple threads (actually 4) using a
[ChaChaCha](https://doc.rust-lang.org/rand/rand/struct.ChaChaRng.html)
random generator using a predefined seed (`&[913453253u32]` for instance).

The goal is to see how many duplicates will there be. And it seems to be a lot :(

The maximum amount of samples that still gives no duplicates (with this starting conditions) is 20399 samples per thread.

For 100 million samples per thread I observe about 4.5% of duplicates,
while for 1 million there is only 0.05% (which is still a lot).
