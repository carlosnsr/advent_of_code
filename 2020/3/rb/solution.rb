TREE = '#'
LATERAL_TRAVEL = 3

DEFAULT_STRATEGY = [{ lateral: 3, vertical: 1 }]

def count_trees(map, strategies = DEFAULT_STRATEGY)
  trees = Array.new(strategies.size, 0)
  positions = Array.new(strategies.size, 0)
  terrain_index = -1
  map.each do |terrain|
    terrain.strip!
    terrain_index += 1

    strategies.each_index do |strategy_index|
      strategy = strategies[strategy_index]
      if (terrain_index % strategy[:vertical] == 0)
        position = positions[strategy_index]
        trees[strategy_index] += 1 if (terrain[position] == TREE)
        positions[strategy_index] = (position + strategy[:lateral]) % terrain.size
      end
    end
    trees
  end
  return trees
end
