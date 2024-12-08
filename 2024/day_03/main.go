package main

import (
  "fmt"
  "regexp"
)

func main() {
  re, err := regexp.Compile(`do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)`)
  check(err)

  scanner := open_file("./input")
  sum := 0
  enabled := true
  for scanner.Scan() {
    line := scanner.Text()
    ops := re.FindAllStringSubmatch(line, -1)
    for _, op := range ops {
      if op[0] == "do()" {
        enabled = true
      } else if op[0] == "don't()" {
        enabled = false
      } else if enabled {
        if len(op) != 3 {
          msg := fmt.Sprintf("Invalid operation: %v", op)
          panic(msg)
        }
        a := to_i(op[1])
        b := to_i(op[2])
        sum += a * b
      }
    }
  }
  fmt.Println("Problem 2: Sum:", sum)
}
