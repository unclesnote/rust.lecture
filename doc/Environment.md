# environment

## ubuntu
```rs
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

after installation by above, below line will be added on `~/.bashrc` automatically.
hence, you can use cargo after re-opening terminal
```
. "$HOME/.cargo/env"
```

## vscode
- extension
  - rust
    - rst-analyzer : for syntax highlight
    - CodeLLDB : for debugger
  - markdown
    - Markdown All in One
    - Markdown Preview Enhanced