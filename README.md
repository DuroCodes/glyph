# Glyph

> [!WARNING]
> This is my first attempt at making a programming language—so it's probably not very good.

Glyph is a stack-based programming language (made with Rust, btw), centered around the concept of glyphs and arrays. (Inspired by APL)

## What's a glyph, anyway?

A glyph is a symbol that does something. In this language, glyphs are used to represent functions. For example, the glyph `+` represents addition.

> [!NOTE]
> This language is stack-based, so operations are done in reverse polish notation.
>
> IE: `[1] [2] +` would be equivalent to `[1] + [2] → [3]`.

## Glyphs

> You can check the [source code](./src/operations.rs) for the full list of glyphs (in case I forgot them)

### Stack glyphs

These glyphs manipulate the stack.

- `↑` creates a range from `1` to `n`
- `⋕` pushes the length of the array
- `↔` reverses the array
- `⇕` sorts the array
- `⊃` filters the array by a boolean array
- `⊕` copies the array
- `∅` clears the stack
- `⋈` joins two arrays together
- `⋉` splits array at index
- `⊣` pops first element from array
- `⊢` pops last element from array
- `⊲` adds element to start of array
- `⊳` adds element to end of array

### Arithmetic glyphs

These glyphs perform arithmetic operations on arrays.

For instance, `[1 2] [2 1] +` would return `[3, 3]`. (1 + 2 = 3, 2 + 1 = 3)

- `+` addition
- `-` subtraction
- `×` multiplication
- `/` division
- `%` modulo
- `∑` sum
- `∏` product

### Comparison glyphs

These glyphs return a boolean array, where `1` is true and `0` is false.

For instance, `[1 2] [2 1] >` would return `[0, 1]`. (1 is not greater than 2, 2 is greater than 1)

- `>` greater than
- `≥` greater than or equal to
- `<` less than
- `≤` less than or equal to
- `¬` not (inverts the array)
- `=` equal

## Examples

Here are some examples of what you can do with Glyph.

### Sum of the first 10 even numbers

```
10 ⇡ # Create a range from 0 to 10
⊕ 2 % ¬ ⊃ # Filter out the odd numbers
∑ # Sum the array
```

### Factorial of 5

```
5 ⇡ # Create a range from 0 to 5
∏ # Multiply the array
```

### Array manipulation example
