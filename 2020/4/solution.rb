class PassportValidator
  attr_reader :count, :valid

  def initialize
    @count = 0
    @valid = 0
  end

  def load(input)
    passport = []
    input.each do |line|
      line.strip!
      passport << line
      if line.empty?
        validate(passport)
        passport = []
      end
    end
    validate(passport)
  end

  def validate(passport)
    @count += 1
    fields = passport.join(' ').split(' ')
    headers = fields.map { |field| field.split(':')[0] }
    headers.delete('cid')  # delete the optional field
    if headers.count == 7
      @valid += 1
      return true
    end
    false
  end
end
