require 'set'

class CustomsCounter
  attr_reader :count, :group_tally

  def initialize
    @count = 0
    @group_tally = Set.new
  end

  def load(unsafe_line)
    line = unsafe_line.strip
    if line.empty?
      @count += tally_group
    else
      add_to_group(line)
    end
  end

  def add_to_group(line)
    line.each_char { |char| @group_tally << char }
    @group_tally.size
  end

  def tally_group
    tally = @group_tally.size
    @group_tally = Set.new
    tally
  end

  def final_count
    @count += tally_group unless @group_tally.empty?
    @count
  end
end

def count_customs(input)
  counter = CustomsCounter.new
  input.each { |line| counter.load(line) }
  counter.final_count
end

puts count_customs(ARGF)
