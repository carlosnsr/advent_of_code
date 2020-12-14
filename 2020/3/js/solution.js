const fs = require('fs')
const TREE = '#'

const count_trees_on_path = (map, right, down) => {
  let position = 0
  let trees = 0
  for(let i = 0; i < map.length; i += down) {
    const terrain = map[i]
    if (terrain[position] == TREE)
      trees++
    position = (position + right) % terrain.length
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

  // Part 1
  // pass map to count_trees_on_path
  console.log("Test 1: ", count_trees_on_path(test_map, 3, 1))
  console.log("Part 1: ", count_trees_on_path(map, 3, 1))

  // Part 2
  const paths = [
    { right: 1, down: 1 },
    { right: 3, down: 1 },
    { right: 5, down: 1 },
    { right: 7, down: 1 },
    { right: 1, down: 2 },
  ]
  let trees_hit = paths.map(({right, down}) => count_trees_on_path(test_map, right, down))
  let product = trees_hit.reduce((acc, num) => acc * num, 1)
  console.log("Test 2: ", product)
  trees_hit = paths.map(({right, down}) => count_trees_on_path(map, right, down))
  product = trees_hit.reduce((acc, num) => acc * num, 1)
  console.log("Part 2: ", product)
}

main()
