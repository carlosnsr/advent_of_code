package main

type Point struct {
  x int
  y int
}

func (p Point) add(q Point) Point {
  return Point{p.x + q.x, p.y + q.y}
}
