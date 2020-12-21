require './solution.rb'

TEST_INPUT = [
  35, 20, 15, 25, 47,
  40, 62, 55, 65, 95,
  102, 117, 150, 182, 127,
  219, 299, 277, 309, 576
]

RSpec.describe '#break_code' do
  let(:preamble) { 5 }

  it 'returns 127' do
    enumerator = load_code(TEST_INPUT)
    expect(break_code(enumerator, preamble)).to eq({ number: 127, breakpoint: 14 })
  end
end

RSpec.describe '#find_weakness' do
  let(:code_break) { { number: 127, breakpoint: 14 } }

  it 'returns 127' do
    enumerator = load_code(TEST_INPUT)
    expect(find_weakness(enumerator, **code_break)).to eq(62)
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

RSpec.describe '#find_contiguous_operands' do
  let(:operands) { [ 35, 20, 15, 25, 47 ] }

  context 'if given a target number that is the sum of contiguous operands' do
    let(:contiguous_operands) { operands.slice(1, 3) }
    let(:target) { contiguous_operands.inject(&:+) }

    it 'returns the contiguous operands' do
      expect(find_contiguous_operands(target, operands)).to eq(contiguous_operands)
    end
  end

  context 'if given a target number that is NOT the sum of any contiguous operands' do
    let(:target) { operands.inject(&:+) + 1}

    it 'returns the contiguous operands' do
      expect(find_contiguous_operands(target, operands)).to eq([])
    end
  end
end
