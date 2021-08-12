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
