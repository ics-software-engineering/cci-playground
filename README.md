# CCI Playground

I share the view of many people that [Cracking the Coding Interview](http://www.crackingthecodinginterview.com/) (CCI) is currently the most authoritative, comprehensive, and high quality reference for computer science students preparing to enter the job market. For more details on why I feel this way, and some "life hacks" on how to improve the UX of the physical book, feel free to watch my screencast by clicking below:

<a href="https://www.youtube.com/watch?v=M-4XRcoMuWs"><img style="border:2px solid black" src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/how-to-get-competent.png"></a>

CCI recommends that you both solve the problem on paper (to simulate the whiteboard experience) as well as in correctly running code (in order to help you really assimilate the material). 

After solving quite a few problems on paper, I decided to start over and implement them in Javascript. At that point, I spent at least a couple of days just setting up an environment that (a) configured ESLint, (b) supported testing in Mocha/Chai, and (c) allowed the use of ES7 constructs like `import` in Node.

Assuming that there might be similar non-trivial configuration for other languages, I decided it might be useful to create a repo that provides setups for various languages along with a sample solution for the very first CCI Problem, called Is Unique:

<blockquote>
1.1. Is Unique: Implement an algorithm to determine if a string has all unique characters. What if you cannot use additional data structures? (Hints: #44, #117, #132)
</blockquote>


## Installation

To get started, [fork this repo](https://help.github.com/en/github/getting-started-with-github/fork-a-repo) into your own GitHub account.  Forking is good for two reasons:

  1. If you want to publish support for another language, just update your fork with your new solution to IsUnique and any configuration files for your new language, then [create a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork).

  2. If and when support for other languages is incorporated into this repo, you can get these additions into your fork by [syncing your fork](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/syncing-a-fork).

## Languages

The following pages provide documentation for each language currently supported in this repo:

  * [Javascript](README-javascript.md) 
  * [C++](README-cpp.md) 
  
Pick one (or more!) and follow the instructions to run the code for solving IsUnique in the language(s) of your choice on your laptop.

## Off to the races

Now that you have locally running code for the first problem in the language of your choice, you might want to try the second one:

<blockquote>
1.2. Check Permutation: Given two strings, write a method to decide if one is the permutation of other. (Hints: #1, #84, #122, #131)
</blockquote>

Have fun!

## Thanks

Thanks to:
 
  * [Henri Casanova](https://github.com/henricasanova) for adding C++ support.
