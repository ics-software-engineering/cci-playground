import unittest
from IsUnique import is_unique, is_unique_without_datastructures


class Test(unittest.TestCase):
    def test_unique(self):
        #self.fail()
        self.assertTrue(is_unique('ABC'))
        self.assertTrue(is_unique('AaBbCc'))

    def test_not_unique(self):
        self.assertFalse(is_unique('AA'))
        self.assertFalse(is_unique('ABCA'))

    def test_unique_without_datastructure(self):
        self.assertTrue(is_unique_without_datastructures('ABC'))
        self.assertTrue(is_unique_without_datastructures('AaBbCc'))

    def test_not_unique_without_datastructure(self):
        self.assertFalse(is_unique_without_datastructures('AA'))
        self.assertFalse(is_unique_without_datastructures('ABCA'))


if __name__=="__main__":
    unittest.main()
