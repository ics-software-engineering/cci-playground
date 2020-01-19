# Java

The java/ subdirectory supports Java solutions using the JUnit 4 testing framework.

## 1. Install Java

You must have the [Java JDK](https://www.oracle.com/technetwork/java/javase/downloads/index.html) installed, 1.8 or newer.

## 2. In IntelliJ IDEA create a new Module from existing sources

Select **File | New | Module from Existing Sources...**

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-new-module-existing-source.png">

Select "Create module from existing sources". Press **Next**.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-create-module-from-existing-sources.png">

Choose the cci-playground/java/ directory. Ensure that IDEA detects the src/ directory.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-module-source-directory.png">

Press **Next**.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-module-libraries.png">

The Import Module tool will not detect any libraries. We'll be installing JUnit later. Press **Nex**.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-import-module.png">

At this step you can change the name of the module by pressing the pencil icon.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-import-finish.png">

Press **Finish**.

If you open any java files you will notice an error 

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-project-sdk-not-defined.png">

We will be setting the java/ module's SDK not cci-playground's. Choose **File | Project Structure...**

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-project-structure.png">

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-no-project-sdk.png">

Choose **Modules** from the left-hand menu. Then choose the java/ module's name (cci-playground1).
 
<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-cci-playground1-sdk.png">

Select a Java SDK from the dropdown menu.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-choose-module-sdk.png">

## 3. Update the java module's paths

Open the Project Structure, make sure the java module is selected and choose the Paths tab. Choose Use module compile output path.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-output-paths.png">

Output path and Test output path should be a directory under the java module. I've chosen the bin directory.

## 4. Add JUnit 4 to the java module

Open the Project Structure. Click **Libraries**

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-empty-libraries.png">

Click the Plus icon to add a library. Choose From Maven.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-libraries-from-maven.png">

Then enter `junit:junit:4.12` in the Download library from Maven Repository and press the Search Icon. When the search indicates that it found one press **OK**. Then choose the name of the java module.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-add-library-to-java-module.png">

You should see the JUnit library in the list of Libraries. Press **OK**.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-added-junit-library.png">

## 5. Add Checkstyle to IDEA

Choose **Preferences | Plugins**. Click the Marketplace tab and search for checkstyle.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-checkstyle.png">

Install the Checkstyle-IDEA (CSI) plugin.

Download the [ICS-SE checkstyle configuration file](http://courses.ics.hawaii.edu/ics211s20-1/morea/050.software-engineering/ics-checkstyle.xml). Add the configuration to the Checkstyle preferences.

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-checkstyle-config.png">

## 6. Build and run the Java code

To build the project select **Build | Build Project**. This will compile all the Java code.
 
<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-build-project.png">
 
Then you can run the tests by right clicking on the java module and selecting **Run All Tests**. 

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-run-all-tests.png">

<img src="https://github.com/ics-software-engineering/cci-playground/raw/master/images/java-test-results.png">

Or you can right click on a Java file and choose **Run**.

