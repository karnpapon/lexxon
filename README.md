# `Lexxon`

lexxon (เล็ก-สั้น) an tiny bytebeat interpreter, basically it's a cyclical stack machine (ring buffer) used at an output. implemented in Rust and inspired by [libglitch](https://github.com/erlehmann/libglitch). 


# Usage 
[osx] for pipe playing sound 
```
cargo run -- `cat tracks/42_forever.lexx` | play -v 0.2 -c 1 -b 8 -e unsigned -t raw -r 8k -
```

# OPs
The postfix operators are

`OP_LT (<), OP_GT (>), OP_EQ (=)`

These take the top two things from the stack, do the comparision, then push 0xFFFFFFFF if the result is true or 0x0 if the result is false. Think of it has follows: If the TOP thing on the stack is >, <, or = to the next thing on the stack then 0xFFFFFFFF else 0x0

`OP_DROP`

removes the top thing from the stack

`OP_DUP`

duplicates the top thing on the stack.

`OP_SWAP`

swaps the top 2 things on the stack

`OP_PICK`

pops the top thing from the stack and duplicates one item that many items back. In other words if the stack is 1,2,3,4,5,6,7,3 then pick pops the top thing 3 and duplicates the 3rd thing back counting from 0, which is no 4. The stack is then 1,2,3,4,5,6,7,4.

Another way to look at it is dup is the same as 0 pick.

`OP_PUT`

sets the n'th element from the top of the stack to the current top. In other words if the stack is 1,2,3,4,5,6,7,3,100 then put will pull the top 100 and then set the 3 element back. The stack will then be 1,2,3,4,100,6,7,3.

`OP_DIV (/), OP_ADD (+), OP_SUB (-), OP_MUL (*), OP_MOD (%), OP_RSHIFT (>>),OP_LSHIFT (<<), OP_OR (|), OP_AND (&), OP_XOR (^)`

These operators pop the top 2 values from the stack, apply the operator, then push the result. The order is as follows

```
b = pop
a = pop
push(a op b)
In other words 4 2 / is 4 divided by 2.
```
`OP_NOT (!)`

Pops the top of the stack, applies the binary negate to it, pushes the result.

# Resources
- [IBNIZ](http://viznut.fi/texts-en/ibniz.html)
- [bitwise AND pattern](https://medium.com/biffures/part-2-the-beauty-of-bitwise-and-or-cdf1d8d87891#.oltigrnle)
- [signed,unsigned in Rust](https://towardsdatascience.com/unsinged-signed-integers-and-casting-in-rust-9a847bfc398f)
- [on integer in Rust](https://medium.com/@marcinbaraniecki/on-integer-types-in-rust-b3dc1b0a23d3)
- [ops](https://github1s.com/greggman/html5bytebeat)

