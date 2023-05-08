# Rainbow Table Attack

![University](https://img.shields.io/badge/University-Project-2F77DF?labelColor=679EEE&style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=Rust&logoColor=ffffff)

Rainbow table attack on SHA-3-256.

# Collaborater

* [Thanushan Pirabakaran](https://github.com/uvsq21919161)
* [Maya Santini](https://github.com/uvsq22003661)
* [Thomas Joly](https://github.com/uvsq21916099)
* [Fratczack Théo](https://github.com/Bynawers)

# Install

Cargo et Rust installation
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Dependencies
```
cargo build
```

# Commands

Generate Rainbow Table with the chosen parameters (constants.rs)
```
cargo run table
```
Launch the interface with which you can start an attack
```
cargo run start
```

Start the attack with the chosen parameters (constants.rs)
```
cargo run attack
```

Check the performance of the reduction functions and the rainbow table
```
cargo run perf -t <type>
```
With theses types : 
```
reduction
attack
table
```

Delete json file in data (all files with --all)
```
cargo run delete <--all / -a>
```

Start SHA-3-256 with a password you give
```
cargo sha3 <password>
```

Unit test of SHA-3-256 functions
```
cargo test
```
