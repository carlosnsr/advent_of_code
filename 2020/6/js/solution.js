const fs = require('fs')

function count_unique_answers(group_answers) {
  const all_answers = group_answers.split('\n').join('')
  const set = new Set(all_answers)
  set.delete('\n')
  return set.size
}

function count_universal_answers(group_answers) {
  const answers = group_answers.split('\n')
  if (answers.length == 1) {
    return answers[0].length
  }

  const count_hash = {}
  answers.forEach((answer) => {
    answer.split('').forEach((letter) => count_hash[letter] = count_hash[letter] +  1 || 1)
  })
  const group_size = answers.length
  const inter = Object.values(count_hash)
    .filter((count) => count == group_size)
  return inter.length
}

function main() {
  // read file and split it
  const file_contents = fs.readFileSync('puzzle_input', 'utf8')  // O(n)
  const group_answers = file_contents.trim().split('\n\n')
  const total_unique_answers = group_answers
    .map(count_unique_answers)
    .reduce((acc, answers) => acc + answers, 0)
  console.log('Part 1:', total_unique_answers)

  const total_universal_answers = group_answers
    .map(count_universal_answers)
    .reduce((acc, answers) => acc + answers, 0)
  console.log('Part 2:', total_universal_answers)
}

main()
