package main

import (
  "fmt"
)

type Point struct {
  x int
  y int
}

type PointMap map[Point]int

// if the point exists, returns true
// if it doesn't exist, adds it and returns false
func (p PointMap) exists_or_add(x, y int) bool {
  point := Point{x, y} // coordinates of the middle letter 'A'
  if p[point] == 1 {
    return true
  } else {
    p[point] = 1
    return false
  }
}

func main() {
  word := "MAS"
  rev := reverse(word)

  count := 0
  space := SearchSpace{grid: make([]*string, len(word)), length: 0}
  x_map := make(PointMap)


  scanner := open_file("./input")
  lower_bound := len(word) - 1
  lines := 0
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

      if space.length < len(word) {
        continue
      }

      if i < bound &&
        space.match_up_right_diagonal(i, *comp) &&
        x_map.exists_or_add(i + 1, lines - 1) {
        count += 1
      }

      if i >= lower_bound &&
        space.match_up_left_diagonal(i, *comp) &&
        x_map.exists_or_add(i - 1, lines - 1) {
        count += 1
      }
    }

    lines++
  }

  fmt.Println("Solution 1:", count)
}

func reverse(s string) (result string) {
  for _, r := range s {
    result = string(r) + result
  }
  return
}
