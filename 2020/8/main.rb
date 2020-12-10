require './solution.rb'

def main
  reader = CodeReader.new
  reader.run(ARGF)
  puts "Part 1: #{reader.acc}"
end

main
