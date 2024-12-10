package main

import (
  "container/list"
  "fmt"
  "regexp"
  "strings"
)

func is_valid_manual(pages *[]int, rules *map[int](map[int]bool)) bool {
  for i := 0; i < len(*pages); i++ {
    for j := i + 1; j < len(*pages); j++ {
      if !(*rules)[(*pages)[i]][(*pages)[j]] {
        return false
      }
    }
  }
  return true
}

func main() {
  re, err := regexp.Compile(`(\d+)\|(\d+)`)
  check(err)

  // read in the rules
  rules := make(map[int](map[int]bool))
  scanner := open_file("./input")
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

  // get ready to save the invalid manuals

  invalids := make([](*[]int), 100) // pre-counted how many invalids there are in `input`
  invs := 0

  // process the manuals
  var mid int
  sum := 0
  for scanner.Scan() {
    pages := Map(strings.Split(scanner.Text(), ","), to_i)

    if is_valid_manual(&pages, &rules) {
      mid = len(pages) / 2
      sum += pages[mid]
    } else {
      // save the invalid manual for later
      invalids[invs] = &pages
      invs++
    }
  }
  fmt.Println("Solution to Part 1:", sum)

  // process and fix the invalid manuals
  sum = 0
  var inserted bool
  for i := 0; i < invs; i++ {
    pages := *invalids[i]

    fixed := list.New()  // TODO: how to reuse/reset this list
    fixed.PushBack(pages[0])
    for j := 1; j < len(pages); j++ {
      val := pages[j]
      inserted = false
      for e := fixed.Front(); !inserted && e != nil; e = e.Next() {
        // check if val comes before this node
        if rules[val][e.Value.(int)] {
          fixed.InsertBefore(val, e)
          inserted = true
        }
      }
      if !inserted { // goes at the end
        fixed.PushBack(val)
      }
    }

    // confirm is fixed
    // fixed_a := list_to_a(fixed)
    // is_fixed := is_valid_manual(&fixed_a, &rules)
    // fmt.Println("Fixed manual:", fixed_a, "is valid:", is_fixed)

    // find the center
    mid = len(pages) / 2
    e := fixed.Front()
    for i := 0; i < mid && e != nil; i++ {
      e = e.Next()
    }
    sum += e.Value.(int)
  }
  fmt.Println("Solution to Part 2:", sum)
}
