const fs = require('fs')

// Binary search for a value equal to or smaller than the given value
// LTE match (Less than or equal)
function bestMatch(arr, find){
  let start = 0
  let end   = arr.length -1
  let mid
  while (start <= end){
    mid = Math.ceil((start + end) / 2)
    // console.log([start, mid, end])
    if (find == arr[mid]){
      return mid
    } else if (find < arr[mid]){
      end = mid -1
    } else if (find > arr[mid]){
      start = mid + 1
    }
  }
  // left edge case for when no value smaller than the given value is found
  if (mid == 0 && find != arr[mid]){
    return -1
  }
  return mid
}

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
const find_pair = (numbers, target_sum = 2020, start = 0, end = undefined) => {  // O(nlog(n))
  if (!end) {
    end = numbers.length
  }
  let found = -1  // assume not found
  for(let i = start; i < end; ++i) {
    // console.log(`Checking ${i}: ${numbers[i]}`)
    compare = numbers[i]
    difference = target_sum - compare
    // console.log('in find_pair', compare, difference)
    found = binary_search_index(numbers, difference, i + 1, end)
    if (found != -1)
      return [i, found]
  }
  return []
}

const find_trio = (numbers, target_sum = 2020) => {
  let found = -1  // assume not found
  let pair = []
  for(let i = 0; i < numbers.length; ++i) {
    compare = numbers[i]
    difference = target_sum - compare
    upper_limit = 1 + bestMatch(numbers, difference)
    pair = find_pair(numbers, difference, i + 1, upper_limit)
    // console.log('in find_trio:', pair)
    if (pair.length != 0) {
      return [i, ...pair]
    }
  }
  return []
}

const part_1 = (numbers) => {
  const answer = find_pair(numbers)
  answer.forEach( index => console.log(`${index}: ${numbers[index]}`) )
  const product = answer.reduce((acc, index) => acc * numbers[index], 1)
  console.log(product)
}

const part_2 = (numbers) => {
  const answer = find_trio(numbers)
  // console.log(answer)
  answer.forEach( index => console.log(`${index}: ${numbers[index]}`) )
  const product = answer.reduce((acc, index) => acc * numbers[index], 1)
  console.log(product)
}

const main = () => {
  // Read in and sort input
  const file_contents = fs.readFileSync('./input', 'utf8')  // O(n)
  const numbers = file_contents.split('\n').map( str => parseInt(str) ) // O(n)
  const ascending_sort = (a, b) => a - b
  const sorted_numbers = numbers.sort(ascending_sort) // O(nlog(n)

  const test_numbers = [
    299,
    366,
    675,
    979,
    1456,
    1721,
  ]
  // Part 1
  // part_1(test_numbers)
  part_1(sorted_numbers)
  // Part 2
  // part_2(test_numbers)
  part_2(sorted_numbers)
}

main()
