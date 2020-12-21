require './solution.rb'

PREAMBLE = 25
code = load_code(ARGF)
code_break = break_code(code, PREAMBLE)
puts "Part 1: #{code_break[:number]}"
puts "Port 2: #{find_weakness(code, **code_break)}"
