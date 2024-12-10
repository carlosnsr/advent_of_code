package main

import (
  "fmt"
)

var directions [4]Point // global, oh no!

type Grid struct {
  grid []string
  size int
}

func main() {
  const MAX_SIZE = 60
  g := Grid{grid: make([]string, MAX_SIZE), size: 0}

  // read in the grid
  scanner := open_file("./input")
  for scanner.Scan() {
    g.grid[g.size] = scanner.Text()
    g.size++
  }

  // set the allowed directions
  directions = [4]Point{
    Point{-1, 0}, // left
    Point{1, 0}, // right
    Point{0, -1}, // up
    Point{0, 1}, // down
  }

  // find each zero, and walk to as many nines as possible
  trails := 0
  walked := make(WalkedMap)
  for y := 0; y < g.size; y++ {
    row := g.grid[y]
    for x, c := range row {
      if c == '0' {
        // fmt.Printf("Found a zero at (%d, %d)\n", y, x)
        clear(walked)
        trails += walk(Point{x, y}, &g, &walked)
      }
    }
  }
  fmt.Println("Solution to Part 1:", trails)
}

type WalkedMap map[Point]bool

func c_at(p Point, g *Grid) byte {
  return g.grid[p.y][p.x]
}

func walk(p Point, g *Grid, w *WalkedMap) (count int) {
  (*w)[p] = true // marking that we've been here

  o := c_at(p, g)
  if o == '9' { // OMG, we found the end!
    return 1
  }

  // for each direction, walk if its height is exactly one greater
  var c byte
  for _, d := range directions {
    q := p.add(d)
    if (*w)[q] ||
      q.y < 0 || q.y >= g.size ||
      q.x < 0 || q.x >= len((g.grid)[0]) {
      continue
    }

    c = c_at(q, g)
    // fmt.Printf("......examining %c (%d, %d)\n", c, q.y, q.x)
    if c - o == 1 { // can only continue if the next cell is one greater
      count += walk(q, g, w)
    }
  }

  return count
}
