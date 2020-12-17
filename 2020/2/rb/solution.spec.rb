require './solution.rb'

RSpec.describe 'extract_rules_and_password' do
  context 'given a line of rule-password input' do
    let(:line) { "3-7 r: mxvlzcjrsqst\n" }

    it 'extracts the password' do
      _, _, password = *extract_rules_and_password(line)
      expect(password).to eq('mxvlzcjrsqst')
    end

    it 'extracts the range rule' do
      rule, _ = *extract_rules_and_password(line)
      expect(rule).to eq({ min: 3, max: 7, char: 'r' })
    end

    it 'extracts the official rule' do
      _, rule, _ = *extract_rules_and_password(line)
      expect(rule).to eq({ first: 2, second: 6, char: 'r' })
    end
  end
end

RSpec.describe 'valid_password?' do
  let(:rule) { { min: 1, max: 3, char: 'r' } }

  context 'given a rule and a password with that many characters in it' do
    it 'returns true' do
      expect(valid_password?('r', rule)).to be_truthy
      expect(valid_password?('r------r', rule)).to be_truthy
      expect(valid_password?('rrr', rule)).to be_truthy
    end
  end

  context 'given a rule and a password without occurences of that character in the acceptable range' do
    it 'returns false' do
      expect(valid_password?('aaaa', rule)).to be_falsey
      expect(valid_password?('rrrr', rule)).to be_falsey
    end
  end
end

RSpec.describe 'official_password?' do
  let(:rule) { { first: 0, second: 2, char: 'r' } }

  context 'given a rule and a password with the char in at exactly one of those indices' do
    it 'returns true' do
      expect(official_password?('r234', rule)).to be_truthy
      expect(official_password?('12r4', rule)).to be_truthy
      expect(official_password?('12r456', rule)).to be_truthy
    end
  end

  context 'given a rule and a password with the char at both indices' do
    it 'returns false' do
      expect(official_password?('r2r4', rule)).to be_falsey
    end
  end

  context 'given a rule and a password with the char at neither index' do
    it 'returns false' do
      expect(official_password?('1r3r', rule)).to be_falsey
    end
  end
end
