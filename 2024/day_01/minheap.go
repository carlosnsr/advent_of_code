package main

// MinHeap is a heap of ints
type MinHeap []int

func (h MinHeap) Len() int { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(val any) {
  *h = append(*h, val.(int))
}

func (h *MinHeap) Pop() any {
  old := *h
  n := len(old)
  val := old[n - 1]
  *h = old[0 : n - 1]
  return val
}
