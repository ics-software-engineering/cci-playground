#include <string>
#include <iostream>

#include "IsUnique.hpp"

/**
 * @brief Returns true if input string has only unique characters.
 *        (Assumption: characters are 8-bit chars, 256 possible values)
 *
 * @param s: input string
 *
 * @return true/false
 */
bool IsUnique::isUnique(std::string &s) {

    // Bit field to keep track of previously seen characters 
    unsigned long bitfield[4] = {0,0,0,0};

    for (auto const &c : s)  {
        if (bitfield[c/32] & (1lu << (c % 32u)))  {
            return false;
        } else {
            bitfield[c/32] |= (1lu << (c % 32u));
        }      
    }

    return true;
}
