package main

import (
  "container/heap"
  "bufio"
  "fmt"
  "os"
  "strconv"
)

// IntHeap is a heap of ints
type IntHeap []int

func (h IntHeap) Len() int { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(val any) {
  *h = append(*h, val.(int))
}

func (h *IntHeap) Pop() any {
  old := *h
  n := len(old)
  val := old[n - 1]
  *h = old[0 : n - 1]
  return val
}

type Node struct {
  h *IntHeap
  label string
  next *Node
}

func main() {
  // set up heaps and linked list
  h1 := &IntHeap{}
  h2 := &IntHeap{}
  // create 2-element looped linked list
  head := &Node{h1, "left", nil}
  head.next = &Node{h2, "right", head}
  curr := head

  // load up each heap with the input
  scanner := make_input_scanner("./input")
  for scanner.Scan() {
    i, err := strconv.Atoi(scanner.Text())
    check(err)

    heap.Push(curr.h, i)
    curr = curr.next
  }

  // strip the heaps
  strip := func(h *IntHeap) {
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
