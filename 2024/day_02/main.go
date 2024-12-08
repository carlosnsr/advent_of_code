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
  // each line, check the levels
  for scanner.Scan() {
    line := scanner.Text()
    levels := Map(strings.Split(line, " "), to_i)

    is_safe := true
    // entire line must be increasing or decreasing
    // the change per level must be 1-3
    // count how many lines meet the above criteria
    comp := levels[0]
    par := parity(levels[1] - comp)
    for i := 1; i < len(levels); i++ {
      step := levels[i] - comp
      if step == 0 || abs(step) > 3 || parity(step) != par {
        is_safe = false
        break
      }
      comp = levels[i]
    }

    if is_safe {
      safe_lines += 1
    }
  }

  fmt.Println("Safe lines:", safe_lines)
}

func parity(i int) int {
  if i > 0 {
    return 1
  }
  return -1
}
