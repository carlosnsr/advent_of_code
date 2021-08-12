def load_integers(input)
  input.map { |e| Integer(e) }.sort
end

# assumes that
#   - the passed-in voltages is sorted
#   - each voltage differs by no more than 3 volts from the previous voltage
#   - excludes the hidden, last voltage of the (last voltage + 3)
# returns the count of 1-volt and 3-volt differences
def find_voltage_differences(voltages)
  last = 0
  result = voltages
    .each_with_object(Array.new(4, 0)) do |voltage, acc|
      index = voltage - last
      acc[index] += 1
      last = voltage
    end
  [result[1], result[3] + 1]
end

# returns an array of the volt-differences between each volt
# NOTE: the first voltage is compared to 0
# NOTE: the last voltage is afterwards compared to itself + 3
def differences(voltages)
  last = 0
  differences = voltages
    .each_with_object([]) do |voltage, acc|
      acc << voltage - last
      last = voltage
    end
  differences << 3
  differences
end

def combinations(differences, start = 0)
  # each adapter can be separated by 1-3 volts from the next.
  # to get the combinations, one can graph it.
  # each parent has 1-3 children, each child being 1-3 volts higher than the parent's voltage.
  # the number of leaf nodes, is the number of combinations.
  # e.g. [1, 2, 3] would be a graph of
  #   0 -- 1 -- 2 -- 3 -- 7
  #     |    |- 3 -- 7
  #     |- 2 -- 3 -- 7
  #     |- 3 -- 7
  # ... for a total of 4 leaf nodes (i.e. 4 combinations)
  # [1, 1, 1] (i.e. 3 1-diffs in a row) => 3 child nodes
  # [1, 1, x] or [1, 2, x] or [2, 1, x] => 2 child nodes
  # [1, x, y] => 1 child node
  return 1 if start == differences.size - 1

  triplet = differences.slice(start, 3)
  if triplet.size == 3 && triplet.reduce(&:+) == 3
    combinations(differences, start + 1) \
      + combinations(differences, start + 2) \
      + combinations(differences, start + 3)
  elsif triplet[0] + triplet[1] <= 3
    combinations(differences, start + 1) \
      + combinations(differences, start + 2)
  else
    combinations(differences, start + 1)
  end
end
