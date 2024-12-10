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
        found := walk(Point{x, y}, &g, &walked)
        fmt.Printf("At (%d, %d), found %d trails\n", y, x, found)
        fmt.Println(walked)
        trails += found
      }
    }
  }
  fmt.Println("Solution to Part 1:", trails)
}

type Marker struct {
  walked bool
  summitted bool
}

type WalkedMap map[Point]Marker

func c_at(p Point, g *Grid) byte {
  return g.grid[p.y][p.x]
}

func walk(p Point, g *Grid, w *WalkedMap) (count int) {
  fmt.Printf("Marking (%d, %d)\n", p.y, p.x)
  (*w)[p] = Marker{true, false} // marking that we've been here

  o := c_at(p, g)
  if o == '9' { // OMG, we found the end!
    fmt.Printf("Hit the summit!!! (%d, %d)\n", p.y, p.x)
    // (*w)[p] = Marker{true, true}
    return 1
  }

  // for each direction, walk if its height is exactly one greater
  var c byte
  for _, d := range directions {
    q := p.add(d)
    if (*w)[q].summitted { // connecting to a path that has already summitted
      fmt.Printf("Hit a summitted path at (%d, %d)\n", q.y, q.x)
      count++
      continue
    }

    if (*w)[q].walked ||
      q.y < 0 || q.y >= g.size ||
      q.x < 0 || q.x >= len((g.grid)[0]) {
      continue
    }

    c = c_at(q, g)
    // fmt.Printf("......examining %c (%d, %d)\n", c, q.y, q.x)
    if c - o == 1 { // can only continue if the next cell is one greater
      found := walk(q, g, w)
      if found > 0 {
        fmt.Printf("Summitted (%d, %d)\n", q.y, q.x)
        (*w)[q] = Marker{true, true}
      }
      count += found
    }
  }

  return count
}
