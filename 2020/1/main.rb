TARGET_SUM = 2020

def find_sum_pair(numbers, target_sum)
  mid_point = target_sum / 2
  numbers.each do |lowest|
    # once we hit numbers higher than mid_point there are no more possible solutions
    break if lowest >= mid_point
    difference = target_sum - lowest
    found = numbers.bsearch { |num| difference - num }
    return [lowest, found] unless found.nil?
  end
  []
end

def main
  numbers = []
  ARGF.each { |line| numbers << line.strip.to_i }
  numbers.sort!

  puts 'Part 1:'
  puts find_sum_pair(numbers, TARGET_SUM)
  puts find_sum_pair(numbers, TARGET_SUM).inject { |acc, num| acc * num }
end

main
