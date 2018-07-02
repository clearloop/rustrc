# Syntax and Semantics

## Basic

### Number Types

以i开头的代表有符号整数而u开头的代表无符号整数。可能的整数大小是 8、16、32 和 64 位。

## Mainn

### Variable-bindings

+ Patterns
+ Type annotations
+ Mutability
+ Initializing bindings
+ Scope and Shadowing

### Functions

+ 表达式和语句
Rust 是一个基于表达式的语言，只有两种语句，其他一切都是表达式。`表达式返回一个值，而语句不是。`
```
+ 语句
  + 声明语句
  + 表达式语句
+ 表达式
```
+ Pannic
`pannic!` 是一个宏, 类似于 `println!`; `!`代表发散，并不会返回。

+ Point
`let f: fn(i32) -> i32;`
