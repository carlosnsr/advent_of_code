OP_REGEX = Regexp.new(/^(?<op>(acc|jmp|nop)) (?<val>[+-]\d+)\n/)

class CodeReader
  attr_reader :acc, :cursor

  def initialize
    @cursor = 0
    @acc = 0
    @memery = []
  end

  def run(input)
    operations = []
    input.each do |line|
      operations << parse_line(line)
    end

    @cursor = 0
    while @cursor < operations.size
      operation = operations[@cursor]
      return @acc if operation[:visited]
      process_op(operation)
      operation[:visited] = true
    end
    @acc
  end

  def parse_line(line)
    match = OP_REGEX.match(line)
    raise "No match on *#{line}*" if match.nil?
    { op: match[:op], val: match[:val].to_i, visited: false }
  end

  def process_op(op)
    @cursor += 1
    case op[:op]
      when 'nop'
      when 'acc'
        @acc += op[:val]
      when 'jmp'
        @cursor += op[:val] - 1
      else
        raise "Unknown op: #{op}"
    end
    op
  end
end
