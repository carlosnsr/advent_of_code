require 'bitarray'

OP_REGEX = Regexp.new(/^(?<op>(acc|jmp|nop)) (?<val>[+-]\d+)\n/)
VISITED = 1

class CodeReader
  def parse(input)
    operations = []
    input.each do |line|
      operations << parse_line(line)
    end
    operations
  end

  def run(operations, cursor = 0, acc = 0)
    visited = BitArray.new(operations.size)
    while cursor < operations.size
      return acc if visited[cursor] == VISITED
      visited[cursor] = VISITED
      operation = operations[cursor]
      cursor, acc = process_op(operation, cursor, acc)
    end
    acc
  end

  def parse_line(line)
    match = OP_REGEX.match(line)
    raise "No match on *#{line}*" if match.nil?
    { op: match[:op], val: match[:val].to_i }
  end

  def process_op(op, cursor, acc)
    cursor += 1
    case op[:op]
      when 'nop'
      when 'acc'
        acc += op[:val]
      when 'jmp'
        cursor -= 1
        cursor += op[:val]
      else
        raise "Unknown op: #{op}"
    end
    [cursor, acc]
  end
end
