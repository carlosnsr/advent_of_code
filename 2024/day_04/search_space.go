package main

type SearchSpace struct {
  grid []*string
  length int
}

func (s SearchSpace) Len() int {
  return s.length
}

// updating its grid and its length
func (s *SearchSpace) Push(str string) {
  if s.length == len(s.grid) {
    // "drop" the first string by making all strings point to the next string
    for i := 0; i < s.length - 1; i++ {
      s.grid[i] = s.grid[i + 1]
    }
    // add the new string at the end
    s.grid[s.length - 1] = &str
  } else {
    s.grid[s.length] = &str
    s.length = s.length + 1
  }
}

func (s SearchSpace) Last() *string {
  return s.grid[s.length - 1]
}

func (s SearchSpace) match_right(i int, t string) bool {
  y := s.length - 1 // last row
  // NOTE: starting at 1 because we're assuming you have already matched the first letter
  for j := 1; j < len(t); j++ {
    if (*s.grid[y])[i + j] != t[j] {
      return false
    }
  }
  return true
}

func (s SearchSpace) match_up(i int, t string) bool {
  y := s.length - 1 // last row
  if y != len(t) - 1 {
    panic("Assumption failed: that grid must have as many rows as the target word has letters")
  }
  // NOTE: starting at 1 because we're assuming you have already matched the first letter
  for j := 1; j < len(t); j++ {
    if (*(s.grid)[y - j])[i] != t[j] {
      return false
    }
  }
  return true
}

func (s SearchSpace) match_up_right_diagonal(i int, t string) bool {
  y := s.length - 1 // last row
  if y != len(t) - 1 {
    panic("Assumption failed: that grid must have as many rows as the target word has letters")
  }
  // NOTE: starting at 1 because we're assuming you have already matched the first letter
  for j := 1; j < len(t); j++ {
    if (*(s.grid)[y - j])[i + j] != t[j] {
      return false
    }
  }
  return true
}

func (s SearchSpace) match_up_left_diagonal(i int, t string) bool {
  y := s.length - 1 // last row
  if y != len(t) - 1 {
    panic("Assumption failed: that grid must have as many rows as the target word has letters")
  }
  // NOTE: starting at 1 because we're assuming you have already matched the first letter
  for j := 1; j < len(t); j++ {
    if (*(s.grid)[y - j])[i - j] != t[j] {
      return false
    }
  }
  return true
}
