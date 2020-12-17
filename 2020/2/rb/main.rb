require './solution.rb'

def main
  valid_passwords = 0
  official_passwords = 0
  ARGF.each do |line|
    rule, official_rule, password = *extract_rules_and_password(line)
    valid_passwords += 1 if valid_password?(password, rule)
    official_passwords += 1 if official_password?(password, official_rule)
  end
  puts "Part 1: #{valid_passwords}"
  puts "Part 2: #{official_passwords}"
end

main
