require './solution.rb'

def main
  rules = BagRules.new
  rules.load(ARGF)
  result = rules.can_contain('shiny gold')
  puts "goes inside: #{result[:goes_inside].size}"
  puts "has inside: #{result[:has_inside]}"
end

main
