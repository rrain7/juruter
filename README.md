# Rust 从放弃到入门

这个教程以 `Jupyter Note` 作为基础工具，弥补传统文档和代码的不足，使用`evcxr` 让 `rust` 可以运行在 `Jupyter` 环境中。

前提条件：需要安装 `Python3.x` 和 `Rust`

安装 jupyter

```
pip install jupyterlab
pip install notebook
```

在 jupyter 中运行 Rust代码

可以运行如下的命令安装 `evcxr`

```sh
cargo install evcxr_jupyter
```

执行
```sh
jupyter lib

# or
jupyter notebook
```

> **Note**  
项目参考了 CS101L，文档中的图片也来自于此课程

## 学习进度
### 1. Rust 所有权
- 参见 [ownership](./ownership.ipynb)

### 2. 错误处理
- 参见 [error handing](./error-handling.ipynb)

### 3. 自定义类型
- 参见 [custom types 1](./custom-types-1.ipynb)

### 4. 垃圾回收
- [ ] todo
