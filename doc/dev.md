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

#### 1.2.2. Tag

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

## 3. Rust Codegen