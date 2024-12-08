package main

import (
  "bufio"
  "fmt"
  "os"
  "strings"
)

func main() {
  // read in the file
  f, err := os.Open("./input")
  check(err)
  scanner := bufio.NewScanner(f)

  safe_lines := 0
  // dampened_lines := 0
  // each line, check the levels
  for scanner.Scan() {
    line := scanner.Text()
    levels := Map(strings.Split(line, " "), to_i)

    deltas := make([]int, len(levels) - 1)
    for i := 1; i < len(levels); i++ {
      deltas[i - 1] = levels[i] - levels[i - 1]
    }

    // entire line must be increasing or decreasing
    // the change per level must be 1-3
    // count how many lines meet the above criteria
    is_safe := true
    par := parity(deltas[0])
    for i := 0; i < len(deltas); i++ {
      delta := deltas[i]
      if delta == 0 || abs(delta) > 3 || parity(delta) != par {
        is_safe = false
        break
      }
    }

    if is_safe {
      safe_lines += 1
    }
  }

  fmt.Println("Problem 1: Safe lines:", safe_lines)
}

func parity(i int) int {
  if i > 0 {
    return 1
  }
  return -1
}
