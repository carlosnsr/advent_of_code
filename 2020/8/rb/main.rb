require './solution.rb'

def main
  reader = CodeReader.new
  operations = reader.parse(ARGF)
  puts "Part 1: #{reader.run(operations)}"
  puts "Part 2: #{reader.resolve(operations)}"
end

main
