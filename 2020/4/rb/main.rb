require './solution.rb'

def main
  validator = PassportValidator.new
  validator.load(ARGF)
  puts "Processed passports: #{validator.count}"
  puts "Part 1: Valid passports: #{validator.valid}"
  puts "Part 2: Actually valid passports: #{validator.actually_valid}"
end

main
