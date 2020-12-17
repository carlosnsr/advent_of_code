def extract_rules_and_password(line)
  rule_str, password = *line.strip.split(': ')
  range, char = *rule_str.split(' ')
  min, max = *range.split('-').map(&:to_i)
  [
    { min: min, max: max,char: char },
    # REMINDER: these indices assume a non-zero index start
    { first: min - 1, second: max - 1 ,char: char },
    password
  ]
end

def valid_password?(password, min:, max:, char:)
  count = password.count(char)
  (min <= count && count <= max)
end

def official_password?(password, first:, second:, char:)
  (password[first] == char) ^ (password[second] == char)
end
