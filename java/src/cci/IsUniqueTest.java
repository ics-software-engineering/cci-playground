package cci;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;
import org.junit.Test;

public class IsUniqueTest {

  /**
   * Tests the isUnique method.
   */
  @Test
  public void isUnique() {
    assertTrue("should return true for a unique string", IsUnique.isUnique("abcde"));
    assertFalse("should return false for a non-unique string", IsUnique.isUnique("abcdee"));
  }
}
