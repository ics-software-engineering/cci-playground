(ns clj.core-test
  (:require [clojure.test :refer :all]
            [clj.core :refer :all]))

(defn map-res
  "Maps each test function over the same input returning a sequence of results"
  [fns s]
  (map #(% s) fns))

(def test-all
  "
  Returns a new function that closes around the functions being tested.

  The function that is returned has the form (fn [s]) -> bool.
  "
  (partial map-res [is-unique? is-unique-no-ds?]))

(defn all-true?
  "Returns true if all values in a sequence are true, false otherwise"
  [seq]
  (every? true? seq))

(defn all-false? [seq]
  "Returns true if all values in a sequence are false, false otherwise"
  (every? false? seq))

(deftest clj-tests
  (testing "empty unique"
    (is (all-true? (test-all ""))))
  (testing "single space unique"
    (is (all-true? (test-all " "))))
  (testing "single char unique"
    (is (all-true? (test-all "a"))))
  (testing "basic unique"
    (is (all-true? (test-all "subdermatoglyphic"))))
  (testing "basic not unique"
    (is (all-false? (test-all "bookkeeper"))))
  (testing "spaces not unique"
    (is (all-false? (test-all "A B C"))))
  (testing "mixed case unique"
    (is (all-true? (test-all "SomeThing"))))
  (testing "mixed case not unique"
    (is (all-false? (test-all "SomeString"))))
  (testing "none unique"
    (is (all-false? (test-all "FFFFFFFF")))))

(clj-tests)

