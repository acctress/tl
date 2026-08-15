# tl
A toy language written in Rust, to be used as a learning reference.

# Project File Structure
```
- crates/
    - tl-parser/        -- AST generation
    - tl-analyser/      -- AST semantic analysis
    - tl-eval/          -- AST evaluation
    - tl-vm/            -- AST to bytecode compilation and VM
- src/                  -- main binary
```

# Implementation Features
- [x] Recursive Descent Parser
- [x] Semantic Analysis (e.g. type checking)
- [ ] AST Evaluator
- [ ] Bytecode Compilation
- [ ] Bytecode VM