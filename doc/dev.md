# dev

## 1. Type

### 1.1. Pirimitive Types

#### 1.1.1. Rust

https://doc.rust-lang.org/std/#primitives

```
bool
never
char
unit
pointer
array
slice
str
tuple
f32
f64
i8
i16
i32
i64
i128
u8
u16
u64
u128
isize
usize
ref
fn
```

#### 1.1.2. Python

https://docs.python.org/3/library/stdtypes.html

### 1.2. Type system for py2rust

#### 1.2.1. Type Class

#### 1.2.2. Type Hint

#### 1.2.3. Tag

## 2. Static Analysis

### 2.1. Static Type Inference

### 2.2. Dynamic Type Inference

``` python
def iff(x, a, b):
  if x == 0:
    return a
  else:
    return b
Z = iff(5, 0, "0")
```

What is return type of `iff` function?

#### 2.2.1. Symbolic Execution

The return type of the `iff` function cannot be determined at compile time.
Therefore, at compile time, the function's execution path must be analyzed to break the function like this.

``` python
Z = None
if 5 == 0:
  Z = 0
else:
  Z = "0"

C = any_function(Z, ...)
```

Nevertheless, it can be seen that the type of `Z` was not determined.
Now we need to find all the parts where Z is used and recursively break up the statements.
Since inlining code by analyzing all the control paths(CFG) of a function is very expensive, so it is assumed that a function has only one return type, and the symbolic exeuction is performed only in special cases.

### 2.3. Exception Handling

## 3. Rust Codegen