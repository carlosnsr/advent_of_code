const fs = require('fs')
const TREE = '#'

const solver = (map) => {
  let i = 0
  let position = 0
  let trees = 0
  while (i < map.length) {
    const terrain = map[i]
    // console.log(position, terrain, terrain[position])
    if (terrain[position] == TREE)
      trees++
    position = (position + 3) % terrain.length
    ++i
  }
  return trees
}


const main = () => {
  // Read in map
  const file_contents = fs.readFileSync('puzzle_input', 'utf8')  // O(n)
  const map = file_contents.split('\n')
  // test map
  const test_map = [
    "..##.......",
    "#...#...#..",
    ".#....#..#.",
    "..#.#...#.#",
    ".#...##..#.",
    "..#.##.....",
    ".#.#.#....#",
    ".#........#",
    "#.##...#...",
    "#...##....#",
    ".#..#...#.#",
  ]

  // pass map to solver
  console.log("test: ", solver(test_map))
  console.log("part 1: ", solver(map))
}

main()
