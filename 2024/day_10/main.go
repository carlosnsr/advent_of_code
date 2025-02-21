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
        clear(walked) // TODO: each trailhead can now use results from previous trailheads, right?
        start := Point{x, y}
        found := walk(start, start, &g, &walked)
        fmt.Printf("At (%d, %d), found %d trails\n", y, x, found)
        // fmt.Println(walked)
        trails += found
      }
    }
  }
  fmt.Println("Solution to Part 2:", trails)
}

type Marker struct {
  walks int
  summitted bool
}

type WalkedMap map[Point]*Marker

func c_at(p Point, g *Grid) byte {
  return g.grid[p.y][p.x]
}

func walk(old_p, p Point, g *Grid, w *WalkedMap) (count int) {
  // mark that we've been here
  var m *Marker
  if n, ok := (*w)[p]; !ok {
    fmt.Println("Creating m")
    m = &Marker{1, false}
    (*w)[p] = m
  } else {
    fmt.Println("Existing m")
    m = n
    m.walks += 1
  }

  o := c_at(p, g)
  fmt.Printf("Walking %c (%d, %d)\n", o, p.y, p.x)

  if m == nil {
    panic("Marker should not be nil at this point")
  }

  if o == '9' { // OMG, we found the end!
    fmt.Printf("Hit the summit!!! (%d, %d)\n", p.y, p.x)
    // mark as summitted
    m.summitted = true
    return 1
  }

  // for each direction, walk if its height is exactly one greater
  var c byte
  for _, d := range directions {
    q := p.add(d)
    if q == old_p { // don't go back to where we came from
      // fmt.Printf("(%d, %d): Skipping the way we came (%d, %d)\n", p.y, p.x, q.y, q.x)
      continue
    }

    // check if out of bounds
    if q.y < 0 || q.y >= g.size || q.x < 0 || q.x >= len((g.grid)[p.y]) {
      fmt.Printf("(%d, %d): Skipping (%d, %d)\n", p.y, p.x, q.y, q.x)
      continue
    }

    qm := (*w)[q]
    if qm != nil {
      if qm.summitted { // connecting to a path that has already summitted
        fmt.Printf("(%d, %d): Hit a summitted path at (%d, %d)\n", p.y, p.x, q.y, q.x)
        count++
        continue
      }

      if qm.walks > 1 { // we've been here before. An extra visit is allowed to help with backtracking
        continue
      }
    }

    c = c_at(q, g)
    // fmt.Printf("......examining %c (%d, %d)\n", c, q.y, q.x)
    if c - o == 1 { // can only continue if the next cell is one greater in height
      found := walk(p, q, g, w)
      if found > 0 { //  when walking back, note that we've been summitted
        fmt.Printf("(%d, %d): Summitted (%d, %d)\n", p.y, p.x, q.y, q.x)
        m.summitted = true
      }
      count += found
    }
  }

  return count
}
