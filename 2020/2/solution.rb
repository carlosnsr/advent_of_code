def extract_rule_and_password(line)
  rule_str, password = *line.strip.split(': ')
  range, char = *rule_str.split(' ')
  min, max = *range.split('-').map(&:to_i)
  [{ min: min, max: max, char: char }, password ]
end

def valid_password?(rule, password)
  count = password.count(rule[:char])
  (rule[:min] <= count && count <= rule[:max])
end
