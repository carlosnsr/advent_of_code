package main

import (
  // "container/heap"
  "fmt"
  "os"
)

func main() {
  read_input("./input")
}

// read in file

// method to help with error-checking
func check(e error) {
  if e != nil {
    panic(e)
  }
}

func read_input(filename string) { // []string {
  data, err := os.ReadFile(filename)
  check(err)
  fmt.Println(string(data))
}
