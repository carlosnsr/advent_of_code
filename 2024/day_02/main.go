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
    passed, index := check_deltas(deltas)
    if passed {
      safe_lines += 1
      continue
    }

    // if the parity changes, drop the item before
    // e.g.          1 3  2 4 5
    // gives deltas:   2 -1 2 1
    // dropping the 3, we get: 1 2 4 5
    // which gives deltas:       1 2 1
    // which is okay

    // if the delta == 0, remove that item
    // e.g.         1 3 3 4 5
    // gives deltas:  2 0 1 1
    // dropping the 3, we get: 1 3 4 5
    // which gives deltas:       2 1 1
    // which is okay

    // if the delta > 3, remove that item
    // e.g.         1 3 7 8 9
    // gives deltas:  2 4 1 1
    // dropping the 7, we get: 1 3 8 9
    // which gives deltas:       2 5 1
    // which is unsalvageable

    fmt.Println("Old levels:", levels)
    fmt.Println("Old deltas:", deltas)
    fmt.Println("Index:", index)

    // brute_forcing it
    is_saved := false
    for i := 0; i < len(levels); i++ {
      snubbed := make([]int, 0)
      snubbed = append(snubbed, levels[:i]...)
      snubbed = append(snubbed, levels[i + 1:]...)

      deltas = make_deltas(snubbed)
      passed, index = check_deltas(deltas)
      if passed {
        dampened_lines += 1
        fmt.Println("New levels:", snubbed)
        fmt.Println("New deltas:", deltas)
        fmt.Println("Conclusion: Safe")
        is_saved = true
        break;
      }
    }
    if !is_saved {
      fmt.Println("Conclusion: UnSafe")
    }
    fmt.Println("--------------------")
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
