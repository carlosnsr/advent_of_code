OP_REGEX = Regexp.new(/^(?<op>(acc|jmp|nop)) (?<val>[+-]\d+)\n/)

class CodeReader
  def run(input)
    operations = []
    input.each do |line|
      operations << parse_line(line)
    end

    cursor = 0
    acc = 0
    while cursor < operations.size
      operation = operations[cursor]
      # puts operation
      return acc if operation[:visited]
      cursor, acc = process_op(operation, cursor, acc)
      operation[:visited] = true
    end
    acc
  end

  def parse_line(line)
    match = OP_REGEX.match(line)
    raise "No match on *#{line}*" if match.nil?
    { op: match[:op], val: match[:val].to_i, visited: false }
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
