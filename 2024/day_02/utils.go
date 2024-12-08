package main

import (
  "strconv"
)

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
