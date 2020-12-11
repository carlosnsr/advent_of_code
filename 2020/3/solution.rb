TREE = '#'
LATERAL_TRAVEL = 3

def count_trees(map)
  trees = 0
  position = 0
  map.each do |terrain|
    terrain.strip!
    trees += 1 if (terrain[position] == TREE)
    position = (position + LATERAL_TRAVEL) % terrain.size
  end
  trees
end
