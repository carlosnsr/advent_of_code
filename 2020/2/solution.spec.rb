require './solution.rb'

RSpec.describe 'extract_rule_and_password' do
  context 'given a line of rule-password input' do
    let(:line) { "3-7 r: mxvlzcjrsqst\n" }

    it 'extracts the password' do
      _, password = *extract_rule_and_password(line)
      expect(password).to eq('mxvlzcjrsqst')
    end

    it 'extracts the rule' do
      rule, _ = *extract_rule_and_password(line)
      expect(rule).to eq({ min: 3, max: 7, char: 'r' })
    end
  end
end

RSpec.describe 'valid_password?' do
  let(:rule) { { min: 1, max: 3, char: 'r' } }

  context 'given a rule and a valid password' do
    it 'returns true' do
      expect(valid_password?(rule, 'r')).to be_truthy
      expect(valid_password?(rule, 'r------r')).to be_truthy
      expect(valid_password?(rule, 'rrr')).to be_truthy
    end
  end

  context 'given a rule and a invalid password' do
    it 'returns false' do
      expect(valid_password?(rule, 'aaaa')).to be_falsey
      expect(valid_password?(rule, 'rrrr')).to be_falsey
    end
  end
end
