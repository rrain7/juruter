# Rust 从放弃到入门

这个教程以 `Jupyter Note` 作为基础工具，弥补传统文档和代码的不足，使用`evcxr` 让 `rust` 可以运行在 `Jupyter` 环境中。

前提条件：需要安装 `Python3x` 和 `Rust`

安装 jupyter

```
pip install jupyterlab
pip install notebook
```

exec rust code in jupyter

可以运行如下的命令安装 `evcxr`
```sh
cargo install evcxr_jupyter
```

执行
```
jupyter lib

# or
jupyter notebook
```
