package main

import (
  "fmt"
)

func main() {
  word := "XMAS"
  rev := reverse(word)
  word_len := len(word)

  count := 0
  space := SearchSpace{data: make([]*string, len(word)), length: 0}
  scanner := open_file("./example.input")
  for scanner.Scan() {
    space.Push(scanner.Text())

    line := *(space.Last())
    bound := len(line) - len(word) + 1
    fmt.Println(line)

    var comp *string
    for i := 0; i < len(line); i++ {
      // check for triggers
      switch line[i] {
        case word[0]:
          comp = &word
        case rev[0]:
          comp = &rev
        default:
          continue
      }
      fmt.Println("Found:", string((*comp)[0]), count)

      // check the horizontal
      if i < bound {
        count += 1
        for j := 1; j < word_len; j++ {
          if line[i + j] != (*comp)[j] {
            count -= 1
            break
          }
        }
      }

      if space.length < len(word) {
        continue
      }

      // check the vertical
      count += 1
      for j := word_len - 1; j >= 0; j-- {
        if (*(space.data)[j])[i] != (*comp)[j] {
          count -= 1
          break
        }
      }

      // check the north-east diagonal
      if i < bound {
        count += 1
        for j := 1; j < word_len; j++ {
          y := word_len - 1 - j
          x := i + j
          if (*(space.data)[y])[x] != (*comp)[j] {
            count -= 1
            break
          }
        }
      }

      // check the north-west diagonal
      if i >= word_len - 1 {
        count += 1
        for j := 1; j < word_len; j++ {
          y := word_len - 1 - j
          x := i - j
          if (*(space.data)[y])[x] != (*comp)[j] {
            count -= 1
            break
          }
        }
      }
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
