# `matchit`

[<img alt="crates.io" src="https://img.shields.io/crates/v/matchit?style=for-the-badge" height="25">](https://crates.io/crates/matchit)
[<img alt="github" src="https://img.shields.io/badge/github-matchit-blue?style=for-the-badge" height="25">](https://github.com/ibraheemdev/matchit)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/matchit?style=for-the-badge" height="25">](https://docs.rs/matchit)

A high performance, zero-copy URL router.

```rust
use matchit::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router.insert("/home", "Welcome!")?;
    router.insert("/users/{id}", "A User")?;

    let matched = router.at("/users/978")?;
    assert_eq!(matched.params.get("id"), Some("978"));
    assert_eq!(*matched.value, "A User");

    Ok(())
}
```

## Parameters

The router supports dynamic route segments. These can either be named or catch-all parameters.

Named parameters like `/{id}` match anything until the next `/` or the end of the path. Note that named parameters must be followed
by a `/` or the end of the route. Dynamic suffixes are not currently supported.

```rust,ignore
let mut m = Router::new();
m.insert("/users/{id}", true)?;

assert_eq!(m.at("/users/1")?.params.get("id"), Some("1"));
assert_eq!(m.at("/users/23")?.params.get("id"), Some("23"));
assert!(m.at("/users").is_err());
```

Catch-all parameters start with `*` and match anything until the end of the path. They must always be at the **end** of the route.

```rust,ignore
let mut m = Router::new();
m.insert("/{*p}", true)?;

assert_eq!(m.at("/foo.js")?.params.get("p"), Some("foo.js"));
assert_eq!(m.at("/c/bar.css")?.params.get("p"), Some("c/bar.css"));

// note that this will not match
assert!(m.at("/").is_err());
```

The literal characters `{` and `}` may be included in a static route by escaping them with the same character. For example, the `{` character is escaped with `{{` and the `}` character is escaped with `}}`.

```rust,ignore
let mut m = Router::new();
m.insert("/{{hello}}", true)?;
m.insert("/{hello}", true)?;

// match the static route
assert!(m.at("/{hello}")?.value);

// match the dynamic route
assert_eq!(m.at("/hello")?.params.get("hello"), Some("hello"));
```

## Routing Priority

Static and dynamic route segments are allowed to overlap. If they do, static segments will be given higher priority:

```rust,ignore
let mut m = Router::new();
m.insert("/", "Welcome!").unwrap();      // priority: 1
m.insert("/about", "About Me").unwrap(); // priority: 1
m.insert("/{*filepath}", "...").unwrap();  // priority: 2
```

## How does it work?

The router takes advantage of the fact that URL routes generally follow a hierarchical structure. Routes are stored them in a radix trie that makes heavy use of common prefixes.

```text
Priority   Path             Value
9          \                1
3          ├s               None
2          |├earch\         2
1          |└upport\        3
2          ├blog\           4
1          |    └{post}     None
1          |          └\    5
2          ├about-us\       6
1          |        └team\  7
1          └contact\        8
```

This allows us to reduce the route search to a small number of branches. Child nodes on the same level of the tree are also prioritized
by the number of children with registered values, increasing the chance of choosing the correct branch of the first try.

## Benchmarks

As it turns out, this method of routing is extremely fast. In a benchmark matching 4 paths against 130 registered routes, `matchit` find the correct routes
in under 200 nanoseconds, an order of magnitude faster than most other routers. You can view the benchmark code [here](https://github.com/ibraheemdev/matchit/blob/master/benches/bench.rs). 

```text
Compare Routers/matchit
time:   [220.81 ns 221.00 ns 221.23 ns]

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
