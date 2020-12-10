const fs = require('fs')

// returns the index of the value that matches the value
// end: size of array
const binary_search_index = (sorted, value, start, end) => {  // O(log(n))
  if (end <= start)
    return -1

  const middle = Math.floor((start + (end - 1)) / 2)
  const middle_value = sorted[middle]
  // console.log(`Searching for ${start} ${end} ${middle}: ${middle_value}`)
  if (middle_value == value) {
    return middle
  } else if (value < middle_value ) {
    return binary_search_index(sorted, value, start, middle)
  } else {
    return binary_search_index(sorted, value, middle + 1, end)
  }
}

// numbers: sorted array of numbers
const find_pair = (numbers) => {  // O(nlog(n))
  let found = -1  // assume not found
  for(let i = 0; i < numbers.length; ++i) {
    // console.log(`Checking ${i}: ${numbers[i]}`)
    compare = numbers[i]
    difference = 2020 - compare
    found = binary_search_index(numbers, difference, i + 1, numbers.length)
    if (found != -1)
      return [i, found]
  }
  return []
}

// Read in and sort input
const file_contents = fs.readFileSync('./input', 'utf8')  // O(n)
const numbers = file_contents.split('\n').map( str => parseInt(str) ) // O(n)
const ascending_sort = (a, b) => a - b
const sorted_numbers = numbers.sort(ascending_sort) // O(nlog(n)
// Part 1
const answer = find_pair(sorted_numbers)
answer.forEach( index => console.log(`${index}: ${numbers[index]}`) )
const [i, j] = answer
const product = numbers[i] * numbers[j]
console.log(product)
