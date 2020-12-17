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

  def run(operations, cursor = 0, acc = 0, visited = nil)
    visited = visited || BitArray.new(operations.size)
    while cursor < operations.size
      break if visited[cursor] == VISITED
      visited[cursor] = VISITED
      operation = operations[cursor]
      cursor, acc = process_op(operation, cursor, acc)
    end
    { acc: acc, completed: cursor == operations.size }
  end

  def resolve(operations)
    cursor = 0
    acc = 0
    visited = BitArray.new(operations.size)
    # run through the operations once
    while cursor < operations.size
      break if visited[cursor] == VISITED
      visited[cursor] = VISITED
      operation = operations[cursor]
      # for each jmp/nop, substitute a nop/jmp and test running this substitution
      if operation[:op] == 'jmp' || operation[:op] == 'nop'
        substitute = {
          op: (operation[:op] == 'nop' ? 'jmp' : 'nop'),
          val: operation[:val]
        }
        sub_cursor, sub_acc = process_op(substitute, cursor, acc)
        alternate_run = run(operations, sub_cursor, sub_acc, visited.dup)
        return alternate_run if alternate_run[:completed]
      end
      # continue calculating this branch
      cursor, acc = process_op(operation, cursor, acc)
    end
    { acc: acc, completed: cursor == operations.size }
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
