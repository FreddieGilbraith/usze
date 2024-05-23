# usze
> A reverse polish notation CLI calculator

Sometimes you just need to do some quick arithmetic.
Takes input from stdin or cli arguments

## Example

### Interactive Mode
```bash
calc
``` 
> `enter` to evaluate, `ctrl-c` to exit

### Argument Mode
```bash
calc 2 3 + 4 /
```

## Operators
- `+` Add
- `-` Minus
- `*` or `x` Multiply
- `/` Divide
- `^` Pow
- `%` Swap
- `#` Duplicate
- `_` Drop
- `log` Logarithm
- `get` Set Register
- `set` Get from Register

## Features
- [x] basic operation
- [x] 256 registers
- [ ] stdin / args interleaving
- [ ] separate inprogress output to stderr
