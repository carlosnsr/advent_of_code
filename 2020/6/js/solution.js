const {group} = require('console')
const fs = require('fs')

function get_unique_answers(group_answers) {
  const all_answers = group_answers.split('\n').join('')
  const set = new Set(all_answers)
  set.delete('\n')
  return set.size
}

function main() {
  // read file and split it newline character
  const file_contents = fs.readFileSync('puzzle_input', 'utf8')  // O(n)
  const group_answers = file_contents.split('\n\n')
  const total_unique_answers = group_answers
    .map(get_unique_answers)
    .reduce((acc, answers) => acc + answers, 0)
  console.log('Part 1:', total_unique_answers)
}

main()
