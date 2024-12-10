package main

import (
  "fmt"
  "regexp"
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
  fmt.Println(rules)
}
