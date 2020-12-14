require './solution.rb'

def main
  reader = CodeReader.new
  puts "Part 1: #{reader.run(ARGF)}"
end

main
