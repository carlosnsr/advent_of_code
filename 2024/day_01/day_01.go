package main

import (
  // "container/heap"
  "bufio"
  "fmt"
  "os"
)

func main() {
  scanner := make_input_scanner("./input")
  for scanner.Scan() {
    fmt.Println(scanner.Text())
  }
}

// read in file

// method to help with error-checking
func check(e error) {
  if e != nil {
    panic(e)
  }
}

func make_input_scanner(filename string) *bufio.Scanner {
  f, err := os.Open(filename)
  check(err)

  scanner := bufio.NewScanner(f)
  scanner.Split(bufio.ScanWords)
  return scanner
}
