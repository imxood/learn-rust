## 编译 c 动态库

```sh
cd c_library_dll
cmake -B build && cmake --build build
```

## 编译 c 静态库

```sh
cd c_library_static
cmake -B build && cmake --build build
```

## 编译 rust 动态库

```sh
cd rust_library_dyn
cargo build
```

## 编译 rust 静态库

```sh
cd rust_library_static
cargo build
```

## 项目根路径下 编译 测试项目

```sh
cargo build
```
