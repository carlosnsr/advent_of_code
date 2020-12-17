BAG_REGEX = Regexp.new(/^(?<bag>.+?) bags contain (?<subbags>.+)\./)
SUBBAG_REGEX = Regexp.new(/^(?<number>\d+) (?<colour>.+?) bags?/)

class BagRules
  attr_reader :lookup, :ruleset

  def initialize
    @ruleset = {}
    @lookup = Hash.new { |hash, key| hash[key] = [] }
  end

  def load(input)
    input.each do |line|
      rule = extract_rule(line)
      update_ruleset(rule)
    end
  end

  def extract_rule(line)
    match = BAG_REGEX.match(line)
    subbags = match[:subbags]
      .split(',')
      .inject({}) do |acc, line|
        submatch = SUBBAG_REGEX.match(line.strip)
        acc[submatch[:colour]] = submatch[:number].to_i unless submatch.nil?
        acc
      end.compact

    {
      bag: match[:bag],
      subbags: subbags,
    }
  end

  def update_ruleset(rule)
    @ruleset[rule[:bag]] = rule[:subbags]
    rule[:subbags].keys.each { |colour| @lookup[colour] << rule[:bag] }
  end

  def can_contain(colour)
    {
      directly: ruleset.keys.include?(colour),
      goes_inside: recursively_lookup(colour).uniq,
      has_inside: recursively_total_contents(colour) - 1, # exclude this bag
    }
  end

  private

  def recursively_lookup(colour)
    lookup[colour]
      .collect { |container| [container].concat(recursively_lookup(container)).flatten }
      .flatten
  end

  def recursively_total_contents(colour)
    # count itself
    count = 1
    # recursively count each bag in it
    current = ruleset[colour]
    current.each { |subbag, number| count += number * recursively_total_contents(subbag) } unless current.nil?
    count
  end
end
