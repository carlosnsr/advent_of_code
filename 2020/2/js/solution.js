const fs = require('fs')

// output should be Number of valid passwords
console.log("answer", main())

function main() {
  // read file and split it newline character
  const file_contents = fs.readFileSync('../puzzle_input', 'utf8')  // O(n)
  const passwordList = file_contents.split('\n') // O(n)

  // counts the number of valid passwords
  let counter = 0

  for (let i = 0 ; i < passwordList.length; i++){
    const parts = passwordList[i].split(' ')
    const policy = parts[0]
    const character = parts[1][0]
    const password = parts[2]

    if (validatePasswordCorporatePolicy(policy, character, password)) counter++
  }

  return counter
}

// validation function
function validatePasswordSledRental(policy, character, password) {
  const [min, max] = policy.split('-').map((x) => parseInt(x))

  const occurence = (password.match(new RegExp(character, "g")) || []).length

  return (min <= occurence && max >= occurence) 
}

function validatePasswordCorporatePolicy(policy, character, password) {
  const [position_1, position_2] = policy.split('-').map((x) => parseInt(x))

  const condition_1 = password[position_1 - 1] === character
  const condition_2 = password[position_2 - 1] === character

  return (condition_1 ^ condition_2) 
}
