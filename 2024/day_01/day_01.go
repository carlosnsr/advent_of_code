package main

import (
  // "container/heap"
  "bufio"
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
  f, err := os.Open(filename)
  check(err)

  scanner := bufio.NewScanner(f)
  for scanner.Scan() {
    fmt.Println(scanner.Text())
  }

}
