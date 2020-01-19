package cci;

import java.util.Hashtable;

/**
 * CCI 1.1: Determines if a string has all unique characters.
 *
 * @author Cam Moore
 *
 */
public class IsUnique {

  /**
   * Checks the given string, returns true if all the characters are unique, false if duplicates.
   *
   * @param str the string to check.
   * @return true if there are no duplicate characters, false otherwise.
   */
  public static boolean isUnique(String str) {
    Hashtable<Character, Boolean> charTable = new Hashtable<>();
    for (int i = 0; i < str.length(); i++) {
      if (charTable.get(str.charAt(i)) != null) {
        return false;
      }
      charTable.put(str.charAt(i), true);
    }
    return true;
  }


  /**
   * Command line interface.
   *
   * @param args the arguments.
   */
  public static void main(String[] args) {
    if (!isUnique("abcde")) {
      System.out.println("should return true for a unique string 'abcde'");
    } else if (isUnique("abcdee")) {
      System.out.println("should return false for a non-unique string 'abcdee'");
    } else {
      System.out.println("isUnique passes two simple tests");
    }
  }

}
