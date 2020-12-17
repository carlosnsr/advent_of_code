def load_code(input)
  input.lazy.map { |e| Integer(e) }
end

def break_code(enumerator, preamble)
  operands = []
  enumerator.each do |number|
    if operands.size < preamble
      operands << number
      next
    end

    return number unless has_sum_operands(number, operands)
    # discard first operand and add number as a new one
    operands.shift
    operands << number
  end
end

def has_sum_operands(sum, operands)
  operands.each_index do |i|
    number = operands[i]
    difference = sum - number
    (i+1..operands.size).each do |j|
      return true if operands[j] == difference
    end
  end
  false
end
