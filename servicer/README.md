<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/144px-Rust_programming_language_black_logo.svg.png" />
# ServiceR

**ServiceR is Service registry made in Rust**

Integration tests using Node and Cypress


## Install Rust

```bash
  $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install Integration Test Suite 

```bash
  $ npm install
```

### Run API

```bash
  $ cd servicer
  $ cargo run
```

### Run Test Suite (headless)

```bash
  $ npm run test
```

### Run Test Suite (Interactive)

```bash
  $ npm run cy:open
```