# CCI Playground

I share the view of many people that [Cracking the Coding Interview](http://www.crackingthecodinginterview.com/) (CCI) is currently the most authoritative, comprehensive, and high quality reference for computer science students preparing to enter the job market. For more details on why I feel this way, and some "life hacks" on how to use CCI effectively, feel free to watch my screencast by clicking below:

<a href="https://www.youtube.com/watch?v=M-4XRcoMuWs"><img style="border:5px solid black" src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/how-to-get-competent.png"></a>


CCI Playground is a repository designed to simplify the process of writing sample solutions for [Cracking the Coding Interview]().  For example, when I started writing solutions in Javascript, I had to invest some time to: (a) configure ESLint, (b) support testing in Mocha/Chai, and (c) use ES7 imports in Node.

Assuming that there will be similar non-trivial setups for other languages, I thought it would be useful to create a repo that provides setups for various languages (Typescript, Python, C#, C++, Java, etc) along with a sample solution for the very first CCI Problem, called Is Unique:

<blockquote>
1.1. Is Unique: Implement an algorithm to determine if a string has all unique characters. What if you cannot use additional data structures? (Hints: #44, #117, #132)
</blockquote>

At the moment, this repo only has boilerplate for Javascript.  I hope to include support for more languages soon. 

## Installation

To get started, [fork this repo](https://help.github.com/en/github/getting-started-with-github/fork-a-repo) into your own GitHub account.  Forking is good for two reasons:

  1. If you want to publish support for another language, just update your fork with your new solution to IsUnique and any configuration files for your new language, then [create a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork).

  2. If and when support for other languages is incorporated into this repo, you can get these additions into your fork by [syncing your fork](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/syncing-a-fork).

## Languages

The following files provide documentation for each language supported in this repo:

  * [Javascript](README-javascript.md)
