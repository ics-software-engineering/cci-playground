# Clojure

Clojure is a lisp dialect that runs on the JVM and can be ran in the browser or locally by compiling to JavaScript.

A great resource for starting with Clojure is the free book [Clojure for the Brave and True](https://www.braveclojure.com/clojure-for-the-brave-and-true/). 

## 1. Clojure installation

[Leiningen](https://leiningen.org) is a Clojure build tool and the easiest way of getting started with Clojure.

Follow the installation instructions at: https://leiningen.org/#install

For more information on Leiningen, see: https://github.com/technomancy/leiningen/blob/master/doc/TUTORIAL.md

## 2. Configure Clojure support in IntelliJ Idea

The [Cursive plugin](https://cursive-ide.com/) for Clojure provides support for editing and running Clojure projects within IntelliJ. It also works directly with Leiningen which was installed on the previous step.

This plugin provides a free license for non-commercial use. First sign up for a license at https://cursive-ide.com/buy.html. Make sure you select `Non-Commercial license`. You should not have to enter any payment details. If you are asked for payment details, you are likely signing up for the wrong license. 

Once you've received your Cursive license, install the Cursive plugin as described at https://cursive-ide.com/userguide/.

## 3. Open the Clojure project in IntelliJ Idea

With the cci-playground project open, select `File -> New -> Module From Existing Sources`. Then select the clj root directory within the cci-playground directory.

<img src="../master/images/clj-idea-new-module.png">

Next you should see a screen titled `Import Module`. Select `Import module from external model` and then choose `Leiningen`.

<img src="../master/images/clj-idea-import-project.png">

From here, click through the defaults until the project is imported. 

Once the project is imported, you should see a layout that looks similar to:

<img src="../master/images/clj-idea-layout.png">

The is-unique functions and the main method are defined in `clj/src/clj/core.clj`. Unit tests can be found in `clj/test/clj/core_test.clj`.

## 4. Running the project

#### Running from the IDE

Open the `clj.core` file and click the green play button beside the main function. You should see something that looks like:

<img src="../master/images/clj-idea-run-ide.png">

#### Running from the terminal

Open a terminal and navigate to the root `clj` directory. Then run the command:

`lein run`

You should see something that looks like:

<img src="../master/images/clj-idea-run-cli.png">

#### REPL

One of the greatest things about Clojure is that you can evaluate parts of the source in a REPL (Read-Eval-Print-Loop). This goes hand-in-hand with iterative development which I find is a necessity for working on these types of problems. This topic is generally outside the scope of this discussion, but it would be worth your while getting familiar with the REPL.  

For more details, see: https://cursive-ide.com/userguide/repl.html

## 5. Running the tests

Clojure unit tests can be ran either from the IDE or from the terminal. I personally believe that running the tests from the terminal provide more useful feedback. 

#### Running tests from the IDE

Right click on the `core_test.clj` file and select `Run 'clj.core-test'`. You should see output similar to:

<img src="../master/images/clj-idea-tests-ide.png">

When all tests pass, you will only see `Process finished with exit code 0`. If any of the tests fail, you will see the failures here.

#### Running tests from the terminal

Open a terminal and navigate to the root `clj` directory.

Run the command `lein test`. You should see output similar to:

<img src="../master/images/clj-idea-tests-cli.png">

