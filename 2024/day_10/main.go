package main

import (
  "fmt"
)

func main() {
  const MAX_SIZE = 60
  grid := make([]string, MAX_SIZE)

  // read in the grid
  lines := 0
  scanner := open_file("./example.input")
  for scanner.Scan() {
    grid[lines] = scanner.Text()
    lines++
  }
  fmt.Println(grid)

  // find each zero, and walk to as many nines as possible
  for y := 0; y < lines; y++ {
    row := grid[y]
    for x, c := range row {
      if c == '0' {
        fmt.Println(row)
        walk(y, x, grid)
      }
    }
  }
}

func walk(y, x int, grid []string) {
  fmt.Printf("Walking from (%d, %d)\n", y, x)
}
