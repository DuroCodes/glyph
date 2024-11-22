#[derive(Debug)]
pub enum Op {
    Number(i32),     // numeric literal
    Array(Vec<i32>), // Array literal [1 2 3]
    Range,           // ↑ creates range [1..=n]
    Add,             // + adds arrays elementwise
    Subtract,        // - subtracts arrays elementwise
    Multiply,        // × multiplies arrays elementwise
    Divide,          // / divides arrays elementwise
    Modulo,          // % calculates the remainder of arrays elementwise
    Sum,             // Σ sums all elements
    Product,         // Π multiplies all elements
    Reverse,         // ↔ reverses array elements
    Sort,            // ⇕ sorts array elements
    Length,          // ⋕ pushes array length
    Greater,         // > compares arrays elementwise
    GreaterEqual,    // ≥ compares arrays elementwise
    Less,            // < compares arrays elementwise
    LessEqual,       // ≤ compares arrays elementwise
    Equal,           // = compares arrays elementwise
    Clear,           // ∅ clears the stack
    Filter,          // ⊃ filters array using condition
    Not,             // ¬ inverts boolean array (0->1, 1->0)
    Duplicate,       // ⊕ duplicates top stack item
    Concatenate,     // ⋈ joins two arrays together
    Split,           // ⋉ splits array at index
    PopLeft,         // ⊣ removes first element
    PopRight,        // ⊢ removes last element
    AppendLeft,      // ⊲ adds element to start
    AppendRight,     // ⊳ adds element to end
}
