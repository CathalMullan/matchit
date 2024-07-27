# `matchit`

A high performance, zero-copy URL router.

## Benchmarks

As it turns out, this method of routing is extremely fast. In a benchmark matching 4 paths against 130 registered routes, `matchit` find the correct routes
in under 200 nanoseconds, an order of magnitude faster than most other routers. You can view the benchmark code [here](https://github.com/ibraheemdev/matchit/blob/master/benches/bench.rs). 

```text
Compare Routers/matchit
time:   [252.25 ns 252.49 ns 252.83 ns]

Compare Routers/actix
time:   [24.118 µs 24.199 µs 24.279 µs]

Compare Routers/path-tree
time:   [380.76 ns 381.05 ns 381.35 ns]

Compare Routers/regex
time:   [1.3426 µs 1.3452 µs 1.3477 µs]

Compare Routers/route-recognizer
time:   [4.6427 µs 4.6479 µs 4.6535 µs]

Compare Routers/routefinder
time:   [6.1235 µs 6.1282 µs 6.1331 µs]

Compare Routers/gonzales
time:   [177.19 ns 177.34 ns 177.53 ns]
```

## Credits

A lot of the code in this package was based on Julien Schmidt's [`httprouter`](https://github.com/julienschmidt/httprouter).
