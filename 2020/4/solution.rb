YEAR_REGEX = Regexp.new(/^(?<num>\d{4})$/)
HGT_REGEX = Regexp.new(/^(?<num>\d+)(?<dim>(in|cm))$/)
HCL_REGEX = Regexp.new(/^#[0-9a-f]{6}$/)
ECL_REGEX = Regexp.new(/^(amb|blu|brn|gry|grn|hzl|oth)$/)
PID_REGEX = Regexp.new(/^\d{9}$/)

class PassportValidator
  attr_reader :actually_valid, :count, :valid

  def initialize
    @count = 0
    @valid = 0
    @actually_valid = 0
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
    validate(passport) unless passport.empty?
  end

  def validate(passport)
    @count += 1
    fields = passport.join(' ').split(' ').map { |field| field.split(':') }
    fields.delete_if { |ele| ele[0] == 'cid' }
    if fields.count == 7
      @valid += 1
      @actually_valid += 1 if fields.all? do |ele|
        header, value = ele
        self.send("validate_#{header}", value)
      end
      return true
    end
    false
  end

  def validate_year(year, min, max)
    match = YEAR_REGEX.match(year)
    return false if match.nil?
    number = match[:num].to_i
    (min <= number && number <= max)
  end

  def validate_hgt(hgt)
    match = HGT_REGEX.match(hgt)
    return false if match.nil?
    number = match[:num].to_i
    case match[:dim]
      when 'cm'
        (150 <= number && number <= 193)
      when 'in'
        (59 <= number && number <= 76)
      else
        false
    end
  end

  def validate_hcl(hcl)
    validate_regex(hcl, HCL_REGEX)
  end

  private

  def validate_byr(byr)
    validate_year(byr, 1920, 2002)
  end

  def validate_iyr(iyr)
    validate_year(iyr, 2010, 2020)
  end

  def validate_eyr(eyr)
    validate_year(eyr, 2020, 2030)
  end

  def validate_regex(val, regex)
    !(regex.match(val).nil?)
  end

  def validate_ecl(ecl)
    validate_regex(ecl, ECL_REGEX)
  end

  def validate_pid(pid)
    validate_regex(pid, PID_REGEX)
  end
end
