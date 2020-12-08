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
      .map do |line|
        submatch = SUBBAG_REGEX.match(line.strip)
        unless submatch.nil?
          { colour: submatch[:colour], number: submatch[:number].to_i }
        end
      end.compact

    {
      bag: match[:bag],
      subbags: subbags,
    }
  end

  def update_ruleset(rule)
    @ruleset[rule[:bag]] = rule[:subbags]
    rule[:subbags].each { |subbag| @lookup[subbag[:colour]] << rule[:bag] }
  end

  def can_contain(colour)
    {
      directly: ruleset.keys.include?(colour),
      inside: recursively_lookup(colour).uniq
    }
  end

  private

  def recursively_lookup(colour)
    lookup[colour].collect { |container| [container].concat(recursively_lookup(container)).flatten }.flatten
  end
end

def main
  rules = BagRules.new
  rules.load(ARGF)
  result = rules.can_contain('shiny gold')
  puts "answer: #{result[:inside].uniq.size}"
end

main
