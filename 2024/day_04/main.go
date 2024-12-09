package main

import (
  "fmt"
)

func main() {
  const WORD = "XMAS"
  space := SearchSpace{data: make([]*string, len(WORD)), length: 0}

  scanner := open_file("./example.input")
  for scanner.Scan() {
    space.Push(scanner.Text())
  }
  fmt.Println(space)
}
