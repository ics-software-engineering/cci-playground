#include "TestIsUnique.hpp"
#include "IsUnique.hpp"

#include <iostream>

/**
 * @brief Run all IsUnique tests
 */
void TestIsUnique::runAllTests() {
     unsigned int num_passed=0, num_failed=0;
    std::cerr << "* IsUnique: \n";
    (runTestUniqueString() ? num_passed++ : num_failed++);
    (runTestNonUniqueString() ? num_passed++ : num_failed++);
    std::cerr << "\t" << num_passed << " tests passed; " << num_failed << " tests failed\n";
}


/**
 * @brief Run IsUnique test on a unique string
 * @return true if test passed, false otherwise
 */
bool TestIsUnique::runTestUniqueString() {
    std::string s = "abcdefghi";
    bool passed = IsUnique::isUnique(s);
    std::cerr << "\t" << (passed ? "✓" : "x") <<
        " should return true for a unique string\n";
    return passed;
}

/**
 * @brief Run IsUnique test on a non-unique string
 * @return true if test passed, false otherwise
 */
bool TestIsUnique::runTestNonUniqueString() {
    std::string s = "abcdefghic";
    bool passed = not IsUnique::isUnique(s);
    std::cerr << "\t" << (passed ? "✓" : "x") <<
        " should return false for a non-unique string\n";
    return passed;
}

