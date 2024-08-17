Adversarial property-based testing for Rust.

# To-do

Adversary is still in development, and has a number of incomplete or missing
features:
- [ ] Make to-do list
- [ ] Create base generators
  - [ ] Canonical
    - [x] [unit](https://doc.rust-lang.org/std/primitive.unit.html)
    - [x] [bool](https://doc.rust-lang.org/std/primitive.bool.html)
    - [ ] integers
    - [ ] floats
    - [ ] [char](https://doc.rust-lang.org/std/primitive.char.html)
    - [ ] [tuple](https://doc.rust-lang.org/std/primitive.tuple.html)
    - [ ] [array](https://doc.rust-lang.org/std/primitive.array.html)
    - [ ] [String](https://doc.rust-lang.org/std/string/struct.String.html)
    - [ ] [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
    - [ ] [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
    - [ ] [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
    - [ ] [Result](https://doc.rust-lang.org/std/result/enum.Result.html)
    - [ ] [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
    - [ ] [Duration](https://doc.rust-lang.org/std/time/struct.Duration.html)
    - [ ] [SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
    - [ ] [NonZero](https://doc.rust-lang.org/std/num/struct.NonZero.html)
    - [ ] [atomics](https://doc.rust-lang.org/std/sync/atomic/index.html)
    - [ ] [std::net](https://doc.rust-lang.org/std/net/index.html) types
      - [ ] [Ipv4Addr](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html)
      - [ ] [Ipv6Addr](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html)
      - [ ] [SocketAddrV4](https://doc.rust-lang.org/std/net/struct.SocketAddrV4.html)
      - [ ] [SocketAddrV6](https://doc.rust-lang.org/std/net/struct.SocketAddrV6.html)
      (maybe? what is this
      [flowinfo/scope_id](https://doc.rust-lang.org/std/net/struct.SocketAddrV6.html#method.new)
      stuff?)
      - [ ] [IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html)
      - [ ] [SocketAddr](https://doc.rust-lang.org/std/net/enum.SocketAddr.html)
    - [ ] [pointers](https://doc.rust-lang.org/std/primitive.pointer.html) (maybe?)
    - [ ] [Instant](https://doc.rust-lang.org/std/time/struct.Instant.html) (maybe?)
    - [ ] TODO: more libstd types
    - [ ] TODO: popular crate types
  - [x] A single T value with no shrinking
    - [x] just(T)
    - [x] just_with(impl Fn() -> T)
  - [ ] Numerical ranges
    - [x] All range types (inclusive/exclusive, bounded/unbounded)
    - [x] Unsigned integers
    - [ ] Signed integers
    - [ ] Floating point numbers
  - [ ] String regexes
  - [ ] Bool with probability
    - [ ] From float in 0.0..=1.0
    - [ ] From ratio (3 in 5, for example)
  - [ ] Vec<T> from length range and InputGenerator<Input = T>
  - [ ] Result<T, E> from probability, shrink direction (Ok or Err), and
  both Inputgenerator<Input = T> and InputGenerator<Input = E>
  - [ ] Random T from a &[T] (just picks a value from the slice)

# License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
