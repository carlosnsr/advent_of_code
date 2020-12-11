require './solution.rb'

def main
  validator = PassportValidator.new
  validator.load(ARGF)
  puts "Valid passports: #{validator.valid}"
end

main
