# CCI Playground

CCI Playground is a repository designed to simplify the process of writing sample solutions for [Cracking the Coding Interview](http://www.crackingthecodinginterview.com/).  For example, when I started writing solutions in Javascript, I had to invest some time to: (a) configure ESLint, (b) support testing in Mocha/Chai, and (c) use ES7 imports in Node.

Assuming that there will be similar non-trivial setups for other languages, I thought it would be useful to create a repo that provides setups for various languages (Typescript, Python, C#, C++, Java, etc) along with a sample solution for the very first CCI Problem, called Is Unique:

<blockquote>
1.1. Is Unique: Implement an algorithm to determine if a string has all unique characters. What if you cannot use additional data structures? (Hints: #44, #117, #132)
</blockquote>

## Installation

To get started, [fork this repo](https://help.github.com/en/github/getting-started-with-github/fork-a-repo) into your own GitHub account.  Forking is good for two reasons:

  1. If you want to publish support for another language, just update your fork with your new solution to IsUnique and any configuration files for your new language, then [create a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork).

  2. If and when support for other languages is incorporated into this repo, you can get these additions into your fork by [syncing your fork](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/syncing-a-fork).

## Languages

This section provides language-specific instructions.

### Javascript

The js/ subdirectory supports Javascript solutions using [ESLint](https://eslint.org/) as a coding standard, and [Mocha](https://mochajs.org/) and [Chai](https://www.chaijs.com/) for testing. The code can be written using ES7 constructs (such as import and export) and run in Node via [ESM](https://www.npmjs.com/package/esm).

#### 1. Install Node.

You must have Node installed on your computer. See [Install Node](https://nodejs.org/en/download/) for details.

#### 2. Install node libraries.

Change directories to cci-playground/js, then invoke `npm install`:

```
~/g/i/c/js (master|…) $ npm install
added 276 packages from 656 contributors and audited 887 packages in 5.652s
found 0 vulnerabilities

~/g/i/c/js (master|…) $
```

#### 3. Create an IntelliJ IDEA (or WebStorm) project

Set up a project that points to the js/ directory in IntelliJ, then open the IsUnique.js file. It should look like this:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/js-intellij-isunique.js.png">

Note the little green checkmark in the upper right corner? That indicates that










