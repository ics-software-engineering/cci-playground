import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

/**
 * Represents a IsUniqueTest.
 * @author Cam Moore
 *
 */
class IsUniqueTest {

  /**
   * Test method for {@link IsUnique#isUnique(java.lang.String)}.
   */
  @Test
  void testIsUnique() {
    assertTrue("should return true for a unique string", IsUnique.isUnique("abcde"));
    assertFalse("should return false for a non-unique string", IsUnique.isUnique("abcdee"));
  }

  @Test
  public void isUnique() {
    assertTrue("should return true for a unique string", IsUnique.isUnique("abcde"));
    assertFalse("should return false for a non-unique string", IsUnique.isUnique("abcdee"));
  }
}
