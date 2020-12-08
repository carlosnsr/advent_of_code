class CustomsCounter
  attr_reader :common_count, :count, :group_count, :group_tally

  def initialize
    @common_count = 0
    @count = 0
    reset_groups
  end

  def reset_groups
    @group_tally = Hash.new { |hash, key| hash[key] = 0 }
    @group_count = 0
  end

  def load(unsafe_line)
    line = unsafe_line.strip
    if line.empty?
      tally_group
    else
      add_to_group(line)
    end
  end

  def add_to_group(line)
    @group_count += 1
    line.each_char { |char| @group_tally[char] += 1 }
    @group_tally.size
  end

  def tally_group
    @count += @group_tally.size
    # count each answer that everyone answered 'yes' to
    @common_count += @group_tally.values.inject(0) do |sum, val|
      sum += 1 if val == @group_count
      sum
    end
    reset_groups
  end

  def final_count
    tally_group unless @group_tally.empty?
    {
      count: @count,
      common_count: @common_count
    }
  end
end

def count_customs(input)
  counter = CustomsCounter.new
  input.each { |line| counter.load(line) }
  counter.final_count
end
