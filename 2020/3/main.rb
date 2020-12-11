require './solution.rb'

def main
  strategies = [
    { lateral: 1, vertical: 1 },
    { lateral: 3, vertical: 1 },
    { lateral: 5, vertical: 1 },
    { lateral: 7, vertical: 1 },
    { lateral: 1, vertical: 2 },
  ]

  trees_hit = count_trees(ARGF, strategies)

  puts "Part 1"
  puts "Trees hit: #{trees_hit[1]}"

  puts "Part 2"
  puts "Trees hit: #{trees_hit}"
  puts "Trees-hit product: #{trees_hit.inject(&:*)}"
end

main
