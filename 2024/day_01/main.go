package main

import (
  "container/heap"
  "bufio"
  "fmt"
  "os"
  "strconv"
)

func main() {
  h1, h2 := load_input("./input")


  fmt.Println("hello")
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

// struct for the below function's linked-list
type Node struct {
  h *MinHeap
  next *Node
}

// given two heaps, reads the input into those heaps
func load_input(filename string) (h1, h2 *MinHeap) {
  // set up heaps for storing the read-in input
  h1 = &MinHeap{}
  h2 = &MinHeap{}

  // set up my cyclic linked list loop
  head := &Node{h1, nil}
  head.next = &Node{h2, head}
  curr := head

  // load input into each heap
  scanner := make_input_scanner(filename)
  for scanner.Scan() {
    i, err := strconv.Atoi(scanner.Text())
    check(err)

    heap.Push(curr.h, i)
    curr = curr.next
  }

  return h1, h2
}

// stripmines the passed in heap, and returns it's min and max values
// at the end, the heap will be empty
func strip(h *MinHeap) (min, max int) {
  min = heap.Pop(h).(int)
  max = min
  for h.Len() > 0 {
    max = heap.Pop(h).(int)
  }
  return min, max
}
