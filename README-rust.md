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

## 4. Configure the IDE to provide automatic linting and formatting

The Rust plugin for Idea based IDEs provides several useful features that should be enabled. 

#### Enable automatic linting

The rust plugin provides automatic linting in the IDE. By default `cargo check` is used to perform the linting, but `clippy` can also be used which provides more advanced linting capabilities. For the purpose of CCI, the default `cargo check` is easiest to use. 

To enable automatic linting:

1. Open the IntelliJ or CLion preferences.
2. Navigate to `Languages & Frameworks -> Rust -> Cargo`
3. Select `Cargo Check` for the External linter
4. Check the box `Run external linter to analyze code on the fly`

<img src="../master/images/rust-intellij-linting-config.png">

#### Enable automatic formatting

The rust plugin provides the ability to automatically format rust code on save. This ensures that rust code looks similar between multiple developers and multiple code bases. 

To enable automatic formatting:

1. Open the IntelliJ or CLion preferences.
2. Navigate to `Languages & Frameworks -> Rust -> Rustfmt`
3. Check the box next to `Run rustfmt on Save`

<img src="../master/images/rust-intellij-fmt-config.png">

## 5. Running the project

Select Run from the main menu, then run

## 6. Run the tests

A nice way to develop your solutions is to write a set of tests to check that your code performs as expected. Unit tests are provided at the bottom of the `lib.rs` module. For example, here is a file containing simple tests of my is_unique solution:

<img src="../master/images/rust-clion-isunique-test-code.png">

There are two ways to run the unit tests. You can either click the green play button beside the tests or individual tests, or you can run the test by opening a Terminal window within Clion and typing:

```
cargo test
```

If you run the tests from the IDE, your IDE will look like the following:

<img src="../master/images/rust-clion-isunique-test-ide.png">

If you run the tests from the terminal, the results should like the following:

<img src="../master/images/rust-clion-isunique-test-terminal.png">

## 7. Everything from the command-line?

Of course, you can do everything from the command line:

```
% cd rust
% # to run the binary use
% cargo run
% # to perform linting use
% cargo check
% # to run the tests use
% cargo test
```     
