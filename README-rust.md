# Rust 

The rust/ subdirectory provides an example of a Rust solution with Rust unit test. 

## 1. Install a Rust Compiler

You must first install Rust using [rustup](https://rustup.rs/) or other [installation options](https://github.com/rust-lang/rustup#other-installation-methods),

## 2. Configure Rust support in IntelliJ of IntelliJ-based IDE (ex. CLion)

Rust is supported in IntelliJ and CLion via the [IntelliJ Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust?_ga=2.199433966.1937618986.1580431331-493015985.1579811855). Install the IntelliJ Rust plugin either directly from the plugin repository or right from your IDE: go to Settings / Preferences | Plugins, switch to Marketplace and type Rust in the search field, then click Install in the plugin description dialog.

## 3. Open the Rust project using IntelliJ or CLion

If you configured the (JetBrain Toolbox)[https://www.jetbrains.com/toolbox-app/] command line launchers, start your preferred IDE from the command line while providing the rust directory as an argument. For example

```
~/launcher/clion rust/
```

where `~/launchers/clion` is the relative path of the clion launcher on my personal computer. Alternatively, open the Rust project by selecting `File` from the main menu, then `Open`. Navigate to and select the rust folder.

## 4. Running the project

Select Run from the main menu, then run

## 5. Run the tests

A nice way to develop your solutions is to write a set of tests to check that your code performs as expected. In the rust/test directory, you will find test code. For example, here is a file containing simple tests of my is_unique solution:

<img src="../images/rust-clion-isunique-test-code.png">

You can run the test by opening a Terminal window within Clion and typing:


```
cargo test
```

You IntelliJ or CLion window will look like the one below.

<img src="../images/rust-clion-isunique-test-terminal.png">



## 6. Everything from the command-line?

Of course, you can do everything from the command line:

```
% cd rust
% # to run the binary use
% cargo run
% # to run the tests use
% cargo test
```     