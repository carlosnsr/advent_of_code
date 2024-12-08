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
  dampened_lines := 0
  // each line, check the levels
  for scanner.Scan() {
    line := scanner.Text()
    levels := Map(strings.Split(line, " "), to_i)

    deltas := make_deltas(levels)
    passed, _ := check_deltas(deltas)
    if passed {
      safe_lines += 1
      continue
    }

    // brute_forcing it
    for i := 0; i < len(levels); i++ {
      snubbed := clone_rm_i(levels, i)

      deltas = make_deltas(snubbed)
      passed, _ = check_deltas(deltas)
      if passed {
        dampened_lines += 1
        break;
      }
    }
  }

  fmt.Println("Problem 1: Safe lines:", safe_lines)
  fmt.Println("Problem 2: Safe lines:", safe_lines + dampened_lines)
}

func make_deltas(levels []int) []int {
  deltas := make([]int, len(levels) - 1)
  for i := 1; i < len(levels); i++ {
    deltas[i - 1] = levels[i] - levels[i - 1]
  }
  return deltas
}

func check_deltas(deltas []int) (bool, int) {
  // entire line must be increasing or decreasing
  // the change per level must be 1-3
  // count how many lines meet the above criteria
  par := parity(deltas[0])
  for i := 0; i < len(deltas); i++ {
    delta := deltas[i]
    if delta == 0 || abs(delta) > 3 || parity(delta) != par {
      return false, i
    }
  }
  return true, -1
}

func parity(i int) int {
  if i > 0 {
    return 1
  }
  return -1
}
