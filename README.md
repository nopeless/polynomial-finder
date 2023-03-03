# Polynomial finder implementation in Rust

Simple polynomial finder based on a few mathematical rules

Please :star: if you found this intersting

### Usage

```sh
> cargo run
1 2 3 4
1 2 3 4
Numbers: 1 2 3 4

  1   2   3   4
    1   1   1
      0   0

A pattern was found. Attempting to generate polynomial using voodoo magic...
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
Polynomial equation: "x + 1"
```

```sh
> cargo run
-42 -56 -64 -66 -62
Numbers: -42 -56 -64 -66 -62

-42 -56 -64 -66 -62
  -14  -8  -2   4
      6   6   6
        0   0

A pattern was found. Attempting to generate polynomial using voodoo magic...
-42 -56 -64 -66 -62 -52 -36 -14 14 48 88 134 186 244 308 378 454 536 624 718
Polynomial equation: "3x^2 - 17x - 42"
```

# Implementation notes

It uses a falling factorial to generate the polynomial coefficients

[falling factorial](https://en.wikipedia.org/wiki/Falling_and_rising_factorials)

You technically don't need to use falling factorials if you just want to generate more numbers.

## Contributing

Bug reports in Issues tab are welcome