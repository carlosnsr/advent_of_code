TARGET_SUM = 2020
MID_POINT = TARGET_SUM / 2

def main
  numbers = []
  ARGF.each { |line| numbers << line.strip.to_i }
  numbers.sort!

  highest = numbers.pop
  # once we hit numbers less than MID_POINT there are no more possible solutions
  while highest >= MID_POINT do
    difference = TARGET_SUM - highest
    found = numbers.bsearch { |num| difference - num }
    unless found.nil?
      puts "first: #{highest}"
      puts "second: #{found}"
      puts "product: #{highest * found}"
      break
    end
    highest = numbers.pop
  end
end

main
