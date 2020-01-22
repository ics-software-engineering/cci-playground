# Javascript

The js/ subdirectory supports Javascript solutions using [ESLint](https://eslint.org/) as a coding standard, and [Mocha](https://mochajs.org/) and [Chai](https://www.chaijs.com/) for testing. The code can be written using ES7 constructs (such as import and export) and run in Node via [ESM](https://www.npmjs.com/package/esm).

## 1. Install Node.

You must have Node installed on your computer. See [Install Node](https://nodejs.org/en/download/) for details.

## 2. Install node libraries.

Change directories to cci-playground/js, then invoke `npm install`:

```
~/g/i/c/js (master|…) $ npm install
added 276 packages from 656 contributors and audited 887 packages in 5.652s
found 0 vulnerabilities

~/g/i/c/js (master|…) $
```

## 3. Create an IntelliJ IDEA (or WebStorm) project

Set up a project that points to the js/ directory in IntelliJ, then open the IsUnique.js file. It should look like this:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/js-intellij-isunique.js.png">

## 4. Run the tests

A nice way to develop your solutions is to write a set of tests to check that your code performs as expected. For example, here is a file containing a couple of simple tests of my IsUnique solution:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/js-intellij-isunique-test-code.png">

If you click on the double left arrow on line five, you'll get a menu that allows you to "Run IsUnique.test.js". Unfortunately, the first time you do so, you'll get an error like this:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/js-intellij-mocha-error.png">

This error occurs because Node does not understand ES7 constructs like import.  The fix is to select Run > Edit Configurations..., then add '-r esm' as a Node option:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/js-intellij-mocha-run-config.png">

After adding this config option, you can run the test without error inside IntelliJ:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/js-intellij-mocha-success.png">

## 5. Everything from the command-line?

Of course you can do everything from the command line:

```
~/g/i/c/js (master|✔) $ npm run test

> js@1.0.0 test /Users/philipjohnson/github/ics-software-engineering/cci-playground/js
> mocha -r esm *.test.js
  isUnique
    ✓ should return true for a unique string
    ✓ should return false for a non-unique string
  2 passing (9ms)
~/g/i/c/js (master|✔) $
```





















