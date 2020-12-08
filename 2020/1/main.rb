TARGET_SUM = 2020

def find_sum_pair(numbers, target_sum, start = 0)
  mid_point = target_sum / 2
  (start..numbers.size).each do |index|
    lowest = numbers[index]
    # once we hit numbers higher than mid_point there are no more possible solutions
    break if lowest >= mid_point
    difference = target_sum - lowest
    found = numbers.bsearch { |num| difference - num }
    return [lowest, found] unless found.nil?
  end
  []
end

def find_sum_trio(numbers, target_sum)
  lowest = numbers.shift
  mid_point = target_sum / 2
  numbers.each_index do |index|
    lowest = numbers[index]
    difference = target_sum - lowest
    found = find_sum_pair(numbers, difference, index + 1)
    return [lowest].concat(found) unless found.empty?
  end
  []
end

def main
  numbers = []
  ARGF.each { |line| numbers << line.strip.to_i }
  numbers.sort!

  part_1 = find_sum_pair(numbers, TARGET_SUM)
  puts 'Part 1:'
  puts part_1
  puts part_1.inject { |acc, num| acc * num }

  part_2 = find_sum_trio(numbers, TARGET_SUM)
  puts 'Part 2:'
  puts part_2
  puts part_2.inject { |acc, num| acc * num }
end

main
