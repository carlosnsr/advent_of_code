package main

import (
  "fmt"
  "regexp"
)

func main() {
  re, err := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
  check(err)

  scanner := open_file("./input")
  sum := 0
  for scanner.Scan() {
    line := scanner.Text()
    muls := re.FindAllStringSubmatch(line, -1)
    for _, mul := range muls {
      a := to_i(mul[1])
      b := to_i(mul[2])
      sum += a * b
    }
  }
  fmt.Println("Problem 1: Sum:", sum)
}
