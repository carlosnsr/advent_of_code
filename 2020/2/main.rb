require './solution.rb'

def main
  valid_passwords = 0
  ARGF.each do |line|
    rule, password = *extract_rule_and_password(line)
    valid_passwords += 1 if valid_password?(rule, password)
  end
  puts valid_passwords
end

main
