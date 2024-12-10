package main

import (
  "fmt"
  "regexp"
  "strings"
)

func main() {
  re, err := regexp.Compile(`(\d+)\|(\d+)`)
  check(err)

  // read in the rules
  rules := make(map[int](map[int]bool))
  scanner := open_file("./example.input")
  for scanner.Scan() {
    parts := re.FindStringSubmatch(scanner.Text())
    if len(parts) == 0 {
      break
    }
    pre := to_i(parts[1])
    post := to_i(parts[2])
    if _, ok := rules[pre]; !ok {
      rules[pre] = make(map[int]bool)
    }
    rules[pre][post] = true
  }

  // process the manuals
  valid := 0
  var is_valid bool
  for scanner.Scan() {
    pages := Map(strings.Split(scanner.Text(), ","), to_i)

    is_valid = true
    for i, page := range pages {
      for j := i + 1; j < len(pages); j++ {
        if _, ok := rules[page][pages[j]]; !ok {
          is_valid = false
          break
        }
      }
      if !is_valid {
        break
      }
    }

    if is_valid {
      valid++
    }
  }
  fmt.Println("Solution to Part 1:", valid)
}
