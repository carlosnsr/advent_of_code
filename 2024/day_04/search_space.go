package main

type SearchSpace struct {
  data []*string
  length int
}

func (s SearchSpace) Len() int {
  return s.length
}

// updating its data and its length
func (s *SearchSpace) Push(str string) {
  if s.length == len(s.data) {
    // "drop" the first string by making all strings point to the next string
    for i := 0; i < s.length - 1; i++ {
      s.data[i] = s.data[i + 1]
    }
    // add the new string at the end
    s.data[s.length - 1] = &str
  } else {
    s.data[s.length] = &str
    s.length = s.length + 1
  }
}

func (s SearchSpace) Last() *string {
  return s.data[s.length - 1]
}

