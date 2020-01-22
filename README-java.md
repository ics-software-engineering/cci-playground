# Java

The java/ subdirectory supports Java solutions using the JUnit 4 testing framework.

## 1. Install Java

You must have the [Java JDK](https://www.oracle.com/technetwork/java/javase/downloads/index.html) installed, 1.8 or newer.

## 2. Create an IntelliJ IDEA project.

Create a new IntelliJ IDEA project, selecting the java/ directory as the root folder for the project. You probably want to rename the project from "java" to something like "cci-playground-java". Here's what the dialog box might look like right before creating the project:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-new-project-dialog.png">

## 3. Display the IsUnique.java file

If all goes well, you should be able to open the IsUnique.java file, and your IntelliJ window should look like this:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-isunique.png">

## 4. Run the tests

If you open the IsUniqueTest.java, you'll notice a lot of red lines because the JUnit library is not yet installed:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-isunique-test-junit-problem.png">

To fix this, go to File | Project Structure, then click on Libraries, then click "+", then select the "Search Maven" option, then enter "junit" as the search term and select "junit:junit-4.12" from the list of possible matches.  Once you've done all this, the dialog box will look like this just before installing JUnit:
 
<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-isunique-test-junit-install.png">

After you click "OK", JUnit will be installed and the IsUniqueTest.java file will be displayed like this:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-isunique-test-junit-ok.png">

You can now run JUnit by clicking the green icon in the gutter on lines 8 or 14.  After running the unit tests, the window will look like this:

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-isunique-test-junit-run.png">
