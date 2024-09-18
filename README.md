# Numeric

The `numeric` project is a mathematical framework for Rust. It contains math traits,
implementations of number types such as big integers and fixed-point values, matrix and
vector types, and more. It aims to be an efficient, well-designed, and comprehensive
toolbox for many kinds of mathematical work in Rust.

## Features

- Extended integer types
  - `U<N>` - N-byte unsigned integer
  - `I<N>` - N-byte signed integer
  - `BigInt` - Unbounded signed integer
- Extended real-valued types
  - `F<N>` - N-byte floating point value
  - `P<N>` - N-byte posit value
  - `Fixed<T, N>` - N-**bit** fixed point value stored as integer `T`
  - `Rat<T>` - Real value number stored as integer `T / T`
- Compounds, Matrices, and more
  - All `T` represent a numeric type of minimal bounds to be useful.
  - `Vec<T, N>` - N long vector
  - `Matrix<T, N, M>` - NxM matrix
  - `Complex<T>` - Imaginary value
  - `Rotor<T, N>` - N-dimension rotor
  - `BiVector<T, N>` - N-dimension bivector

## FAQ

### What about `num_traits`?

`numeric`'s traits crate is very similar in some ways to `num_traits`, which may lead one
to wonder why not just use that crate. In short, while `num_traits` aims to be good for
working with generic numeric types, it falls short of what `numeric` requires. It has
overly strict bounds on `Real` making it unsuitable for our use, and is missing many of
the custom operator traits we require. While the first may someday in the far future be
fixed by a breaking change, the latter issue is unlikely to ever change as it's not in
scope for `num_traits`.

As such, the decision was made to work entirely through our own, more tailored base traits.
On the other hand, support for `num_traits` is a desirable goal under a feature gate
eventually, to allow other crates that use it to support `numeric` types transparently.

## Design Thoughts

### Ideas

- Replace TaggedOffset with a TaggedPtr into the linked list
- Use fetch_add in interner adder to prevent race conditions
- Maybe we should change `BigInt` to `IBig` for consistency, and a potential future `UBig`.

### Even More Number Types

- P-adics
  - How do these get represented? Every useful one is infinite
  - May be useful to work with truncated representations, as long as they can produce outputs

### Other Math Stuff

- Rotors
- Quaternions
- All the math stuff I can imagine
