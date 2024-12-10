package main

import (
  "fmt"
)

func main() {
  word := "XMAS"
  rev := reverse(word)

  count := 0
  space := SearchSpace{grid: make([]*string, len(word)), length: 0}
  scanner := open_file("./input")
  lower_bound := len(word) - 1
  for scanner.Scan() {
    space.Push(scanner.Text())

    line := *(space.Last())
    bound := len(line) - len(word) + 1

    var comp *string
    for i := 0; i < len(line); i++ {
      // check for first letter of word or rev
      switch line[i] {
        case word[0]:
          comp = &word
        case rev[0]:
          comp = &rev
        default:
          continue
      }

      // fmt.Println("Found:", string(line[i]), "at", i)
      if i < bound && space.match_right(i, *comp) {
        count += 1
      }

      if space.length < len(word) {
        continue
      }

      if space.match_up(i, *comp) {
        count += 1
      }

      if i < bound && space.match_up_right_diagonal(i, *comp) {
        count += 1
      }

      if i >= lower_bound && space.match_up_left_diagonal(i, *comp) {
        count += 1
      }
    }
  }

  fmt.Println("Solution 1:", count)
}

func reverse(s string) (result string) {
  for _, r := range s {
    result = string(r) + result
  }
  return
}
