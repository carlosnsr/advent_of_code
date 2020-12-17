const fs = require('fs')
const _ = require('lodash')



const main = () =>{
  // counter for valid passports
  let counter = 0

  // read the file by 'empty line' (\n\n)
  const file_contents = fs.readFileSync('../puzzle_input', 'utf-8')
  const passportList = file_contents.split('\n\n')

  // loop to go over passports
  for (let i = 0; i < passportList.length; i++){ //passportList.length
    // split by space or newling
    const keyValuePairs = passportList[i].split(/[ \n]/)

    const keysMap = new Map()      
    for (let j = 0; j < keyValuePairs.length; j++){
      const [key, value] = keyValuePairs[j].split(':')

      keysMap.set(key, value)
    }

    if (validatePassport(keysMap)) counter++
  }
  
  return counter

}

// validation method for Exist check
const allKeysExist = (keysMap) => {
  const requiredKeys = ['byr','iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'] 

  for (let i = 0; i < requiredKeys.length; i++){
    if (!keysMap.get(requiredKeys[i])) return false
  }

  return true
}

const yearValidation = _.curry((min, max, value) => {
  value = parseInt(value)
  return (value >= min && value <= max) 
})

const heightValidation = (value) => {
  const unit = value.slice(-2)
  const height = value.slice(0, value.length -2)

  if (unit === 'cm'){
    return (height >= 150 && height <= 193)
  }else if (unit === 'in'){
    return (height >= 59 && height <= 76)
  }

  return false
}

const regexValidation = _.curry((regex, value) => {
  return regex.test(value)
})


// validation method for Exist and In-range check
const validatePassport = (keysMap) => {
  const requiredKeys = {
    'byr': yearValidation(1920, 2002),
    'iyr': yearValidation(2010, 2020), 
    'eyr': yearValidation(2020, 2030), 
    'hgt': heightValidation,
    'hcl': regexValidation(/^#[0-9a-f]{6}$/), 
    'ecl': regexValidation(/^(amb|blu|brn|gry|grn|hzl|oth)$/), 
    'pid': regexValidation(/^\d{9}$/)
  } 

  for (const key in requiredKeys){
    const value = keysMap.get(key)

    if (!value) return false
    else {
      const validateFuntion = requiredKeys[key]
      if (!validateFuntion(value)) return false
    }
  }

  return true
}

console.log('Answer: ', main())