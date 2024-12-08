package main

// method to help with error-checking
func check(e error) {
  if e != nil {
    panic(e)
  }
}

// returns the absolute value of the given integer
func abs(x int) int {
  if x < 0 {
    return -x
  }
  return x
}
