require './src/solution.rb'

voltages = load_integers(ARGF)
differences = find_voltage_differences(voltages)
puts "Part 1: #{differences.reduce { |acc, n| acc * n }}"
