package main

import (
  "bufio"
  "os"
  "strconv"
)

// ============================== ERRORS ==============================

// method to help with error-checking
func check(e error) {
  if e != nil {
    panic(e)
  }
}

// ============================== CONV ==============================

func to_i(s string) int {
  i, err := strconv.Atoi(s)
  check(err)
  return i
}

// ============================== FILE ==============================

// opens a file, passes it to a scanner, and returns the scanner
func open_file(filename string) *bufio.Scanner {
  // read in the file
  f, err := os.Open(filename)
  check(err)
  scanner := bufio.NewScanner(f)
  return scanner
}

// ============================== MATH ==============================

// returns the absolute value of the given integer
func abs(x int) int {
  if x < 0 {
    return -x
  }
  return x
}

// ============================== OTHER ==============================

// a generic map function
func Map[T, V any](ts []T, f func(T) V) []V {
  vs := make([]V, len(ts))
  for i, t := range ts {
    vs[i] = f(t)
  }
  return vs
}

// creates a new array (with its own memory) but missing the ith element
func clone_rm_i(arr []int, i int) []int {
  a := make([]int, 0)
  a = append(a, arr[:i]...)
  a = append(a, arr[i + 1:]...)
  return a
}
