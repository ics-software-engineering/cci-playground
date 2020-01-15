#include "TestIsUnique.hpp"

class UnitTests {

    public:
        static void runAllTests() {
            // Test IsUnique
            TestIsUnique::runAllTests(); 
        }
};

int main(int argc, char **argv) {
    UnitTests::runAllTests();
}
