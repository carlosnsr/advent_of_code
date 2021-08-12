def load_integers(input)
  input.map { |e| Integer(e) }.lazy
end

def find_voltage_differences(enumerator)
  last = 0
  result = enumerator
    .sort
    .each_with_object(Array.new(4, 0)) do |voltage, acc|
      index = voltage - last
      acc[index] += 1
      last = voltage
    end
  [result[1], result[3] + 1]
end
