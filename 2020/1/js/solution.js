const fs = require('fs')
const _ = require('lodash')

// numbers: sorted array of numbers
const curried_find = _.curry((fn, numbers, target_sum = 2020, start = 0) => {
  let mid_point = Math.ceil(target_sum / 2)
  let stop  = 1 + _.sortedIndex(numbers, mid_point)
  for(let i = start; i < stop; ++i) {
    const difference = target_sum - numbers[i]
    const result = fn(numbers, difference, i + 1)
    if (result.length != 0)
      return [i, ...result]
  }
  return []
})

const find_pair = curried_find((numbers, difference) => {
  const index = _.sortedIndex(numbers, difference)
  return (numbers[index] === difference ? [index] : [] )
})

const find_trio = curried_find(find_pair)

const get_solution = (part, numbers, fn) => {
  console.log(`Part ${part}`)
  const answer = fn(numbers)
  answer.forEach(index => console.log(`${index}: ${numbers[index]}`))
  const product = answer.reduce((acc, index) => acc * numbers[index], 1)
  console.log(`Product: ${product}`)
}

const main = () => {
  // Read in and sort input
  const file_contents = fs.readFileSync('../puzzle_input', 'utf8')  // O(n)
  const numbers = file_contents.split('\n').map( str => parseInt(str) ) // O(n)
  const ascending_sort = (a, b) => a - b
  const sorted_numbers = numbers.sort(ascending_sort) // O(nlog(n)

  // Part 1
  get_solution(1, sorted_numbers, find_pair)
  // Part 2
  get_solution(2, sorted_numbers, find_trio)
}

main()
