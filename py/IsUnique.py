# IsUnique.py
# Solution to IsUnique problem from Cracking the Coding Interview
import array


def is_unique(string):
    """
    Test if characters in string are all unique.
    :param string: String to test.
    :return: True or False
    """
    return len(set(string)) == len(string)


def is_unique_without_datastructures(string):
    """
    Test if characters in string are all unique.
    :param string: String to test.
    :return: True or False
    """
    seen = array.array('i', [0]*256)
    for c in string:
        if seen[ord(c)] == 1:
            return False
        else:
            seen[ord(c)] = 1
    return True
