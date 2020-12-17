require './solution.rb'

RSpec.describe '#break_code' do
  let(:preamble) { 5 }
  let(:input) do
    [
      35, 20, 15, 25, 47,
      40, 62, 55, 65, 95,
      102, 117, 150, 182, 127,
      219, 299, 277, 309, 576
    ]
  end

  it 'returns 127' do
    enumerator = load_code(input)
    expect(break_code(enumerator, preamble)).to eq(127)
  end
end

RSpec.describe '#load_code' do
  let(:input) { [ 35, 20, 15, 25, 47 ] }

  it 'returns a lazy enumerator' do
    expect(load_code(input)).to be_a(Enumerator::Lazy)
  end

  context 'given an input of numeric strings' do
    let(:input) { [ "35\n", "20\n", "15\n", "25\n", "47\n" ] }

    it 'returns a lazy enumerator that returns integers' do
      enumerator = load_code(input)
      expect(enumerator.take(3).force).to eq([35, 20, 15])
    end
  end
end

RSpec.describe '#has_sum_operands' do
  context 'given two operands and a number' do
    let(:operands) { [1, 2] }

    it "returns true if the operands' sum equals the number" do
      expect(has_sum_operands(3, operands)).to be_truthy
    end

    it "returns false if the operands' sum does NOT equal the number" do
      expect(has_sum_operands(5, operands)).to be_falsey
    end

    it "returns false if the operands' sum can only be the product of the same number" do
      expect(has_sum_operands(4, operands)).to be_falsey
    end
  end
end
