/**
 * CCI 1.1: Determines if a string has all unique characters.
 * Returns true if all unique, false otherwise.
 */
export function isUnique(str: string): boolean {
  const charTable: { [key: string]: boolean } = {};
  for (let i = 0; i < str.length; i++) {
    if (charTable[str.charCodeAt(i)]) {
      return false;
    }
    charTable[str.charCodeAt(i)] = true;
  }
  return true;
}
