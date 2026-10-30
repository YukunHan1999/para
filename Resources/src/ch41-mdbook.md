# mdbook

## 1 init

```text
# 生成到当前目录
mdbook init

# 创建时指定目录
mdbook init /workspace/github.com/mybook

# 创建带有主题目录
mdbook init --theme --title="my amazing book" --ignore=git /workspace/github.com/YukunHan1999/test/
```


## 2 build

```text
# 默认目录是book.toml中的build.build-dir配置
mdbook build

# 手工指定路径
mdbook build /workspace/github.com/mybook/book
```

## 3 watch 

```text
mdbook watch /workspace/github.com/mybook/book
```

## 4 serve

```text
mdbook serve /workspace/github.com/mybook/book -p 3000 -n 127.0.0.1 
```

## 5 clean

```text
# 清理当前目录
mdbook clean

# 清理指定目录
mdbook clean /workspace/github.com/mybook
```

## 6 link

[mdbook文档](https://rust-lang.github.io/mdBook/)
