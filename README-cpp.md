# C++

The cpp/ subdirectory supports C++ solutions using a custom testing framework (for now). 

## 1. Install a C++ compiler and cmake

You must have installed a C++ compiler (g++, clang++) and [cmake](https://cmake.org/) on your computer. 

## 3. Create a CLion (or Eclipse) cmake project

Set up a cmake project that points to the cpp/ directory in tour IDE ("New CMake project from Sources" on the "Welcome to CLion" initial popup).  Then open the src/IsUnique.cpp file. It should look like this using CLion:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/cpp-clion-isunique.cpp.png">

CLion will load compiler-specific style toolds (e.g., Clang-Tidy)

## 4. Build the project

Clicking on the "hammer" icon, or  selecting "Build > BuildProject" menu item should build the project in a few seconds.

## 5. Run the tests

A nice way to develop your solutions is to write a set of tests to check that your code performs as expected. In the cpp/test directory you will find test code. For example, here is a file containing a couple of simple tests of my IsUnique solution:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/cpp-clion-isunique-test-code.png">

You can run the tests from CLion by clicking on the "play" icon, or selecting "Run > Run 'cpp'" menu item should run the test from within CLion and produce this output: 

```
* IsUnique: 
	✓ should return true for a unique string
	✓ should return false for a non-unique string
	2 tests passed; 0 tests failed
```

## 6. Everything from the command-line?

Of course you can do everything from the command line:

```
% cd cpp
% mkdir build
% cd build
% cmake ..
% make
% ./unit_tests
```     






















