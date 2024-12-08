package main

import (
  "container/heap"
  "bufio"
  "fmt"
  "os"
  "strconv"
)

type Node struct {
  h *MinHeap
  label string
  next *Node
}

func main() {
  // set up heaps and linked list
  h1 := &MinHeap{}
  h2 := &MinHeap{}
  // create 2-element looped linked list
  head := &Node{h1, "left", nil}
  head.next = &Node{h2, "right", head}
  curr := head

  // load input into each heap
  scanner := make_input_scanner("./input")
  for scanner.Scan() {
    i, err := strconv.Atoi(scanner.Text())
    check(err)

    heap.Push(curr.h, i)
    curr = curr.next
  }

  // strip the heaps
  strip := func(h *MinHeap) {
    first := heap.Pop(h)
    last := first
    for h.Len() > 0 {
      last = heap.Pop(h)
    }
    fmt.Println("Stripped", first, last)
  }

  strip(h1)
  strip(h2)
}

// read in file

// method to help with error-checking
func check(e error) {
  if e != nil {
    panic(e)
  }
}

// returns a scanner for the given file
// each call to scanner.Scan() will return the next word in the file
func make_input_scanner(filename string) *bufio.Scanner {
  f, err := os.Open(filename)
  check(err)

  scanner := bufio.NewScanner(f)
  scanner.Split(bufio.ScanWords)
  return scanner
}
