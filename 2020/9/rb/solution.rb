def load_code(input)
  input.map { |e| Integer(e) }.lazy
end

def break_code(enumerator, preamble)
  operands = []
  enumerator.with_index.each do |number, i|
    if operands.size < preamble
      operands << number
      next
    end

    return { number: number, breakpoint: i } unless has_sum_operands(number, operands)
    # discard first operand and add number as a new one
    operands.shift
    operands << number
  end
end

def has_sum_operands(target, operands)
  operands.each_index do |i|
    number = operands[i]
    difference = target - number
    (i+1..operands.size).each do |j|
      return true if operands[j] == difference
    end
  end
  false
end

def find_weakness(enumerator, number:, breakpoint:)
  # extract numbers before index
  possible_operands = enumerator.take(breakpoint).force
  operands = find_contiguous_operands(number, possible_operands).sort
  raise "No contiguous operands found" if operands.empty?
  # return addition of min and max operands
  (operands[0] + operands[-1])
end

# find contiguous operands that sum number
def find_contiguous_operands(target, operands)
  operands.each_index do |i|
    # starting at beginning, sum range
    sum = 0
    (i..operands.size - 1).each do |j|
      sum += operands[j]
      break if sum > target
      return operands.slice(i, j - i + 1) if sum == target
    end
  end
  []
end
