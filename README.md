# nanosat

An almost-efficient implementation of the Conflit-Driven Clause Learning (CDCL) framework for solving SAT problems in Rust.

This project was developed for learning purposes based on a project I previously worked on at UFMG for the *Theory and Practice of SMT Solving* course.

## Current progress

- [x] CI
- [x] CLI
- [x] Parser for DIMACS inputs
- [x] Proper error handling (thiserror, anyhow, etc)
- [x] CDCL basic architecture
- [ ] 2-watched literals
- [x] VSIDS heuristic
- [x] Unit tests
- [ ] Regression tests
- [ ] DRAT proof generation
- [ ] Benchmark set
- [ ] Evaluation against MiniSAT

## Contributing

Feel free to open issues or submit pull requests if you'd like to contribute to this project.

## License

This project is licensed under the [MIT License](LICENSE).
