(ns clj.core
  (:gen-class))

(defn is-unique?
  "
  Determines if all characters in a string are unique.

  Compares the length of a string to the length of a set containing the characters of a string.
  "
  [s]
  (= (count s)
     (count (set s))))

(defn is-unique-no-ds?
  "
  Determines if all characters in a string 's' are unique without using data structures.

  The string is first sorted, then the first two characters are recursively compared for equality.
  "
  [s]
  (let [sorted (sort s)]
    (if (<= (count sorted) 1)
      true
      (if (= (first sorted)
             (second sorted))
        false
        (recur (rest sorted))))))

(defn -main
  "Entry point into Clojure app"
  [& args]
  (println (is-unique-no-ds? "abc"))
  (println (is-unique? "foo")))
