require './src/solution.rb'

voltages = load_integers(ARGF)
counts = count_voltage_differences(voltages)
puts "Part 1: #{counts.reduce { |acc, n| acc * n }}"
