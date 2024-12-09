package main

import (
  "fmt"
)

func main() {
  word := "XMAS"
  rev := reverse(word)
  word_len := len(word)

  space := SearchSpace{data: make([]*string, len(word)), length: 0}
  count := 0
  scanner := open_file("./example.input")
  for scanner.Scan() {
    space.Push(scanner.Text())

    line := *(space.Last())
    fmt.Println(line)
    bound := len(line) - len(word) + 1
    var comp *string
    for i := 0; i < bound; i++ {
      // check for triggers
      switch line[i] {
        case word[0]:
          comp = &word
        case rev[0]:
          comp = &rev
        default:
          continue
      }
      fmt.Println("Found:", string((*comp)[0]))

      // check the horizontal
      count += 1
      for j := 1; j < word_len; j++ {
        fmt.Println("Comparing:", string(line[i + j]), string((*comp)[j]))
        if line[i + j] != (*comp)[j] {
          count -= 1
          break
        }
      }
      if space.length < len(word) {
        continue
      }
      // check for verticals & diagonals
      // check the south-east diagonal
    }
  }

  fmt.Println("\nSolution 1:", count)
}

func reverse(s string) (result string) {
  for _, r := range s {
    result = string(r) + result
  }
  return
}
