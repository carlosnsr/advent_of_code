require './solution.rb'

RSpec.describe 'run with provided test input' do
  let(:input) do
    [
      "nop +0\n",
      "acc +1\n",
      "jmp +4\n",
      "acc +3\n",
      "jmp -3\n",
      "acc -99\n",
      "acc +1\n",
      "jmp -4\n",
      "acc +6\n",
    ]
  end

  it 'returns the answer before repeating an already-run instruction' do
    reader = CodeReader.new
    operations = reader.parse(input)
    expect(reader.run(operations)).to eq(5)
  end
end

RSpec.describe CodeReader do
  let(:reader) { CodeReader.new }

  describe '#run' do
    let(:operations) { reader.parse(input) }

    context 'if passed no operations' do
      let(:input) { [] }

      it 'calculates the correct acc value' do
        expect(reader.run(operations)).to eq(0)
      end
    end

    context 'if passed with just no-ops' do
      let(:input) do
        [
          "nop +0\n",
          "nop +1\n",
          "nop +2\n",
        ]
      end

      it 'returns 0' do
        expect(reader.run(operations)).to eq(0)
      end
    end

    context 'if passed operations that has no jumps' do
      let(:input) do
        [
          "acc +1\n",
          "acc +2\n",
          "acc +3\n",
          "acc -1\n",
        ]
      end

      it 'calculates the total accumulated values' do
        expect(reader.run(operations)).to eq(5)
      end
    end

    context 'if passed operations that jumps forward' do
      let(:input) do
        [
          "acc +1\n",
          "jmp +4\n",
          "acc +2\n",
          "acc +3\n",
          "acc -1\n",
          "acc +9\n",
        ]
      end

      it 'does not add values that were skipped over' do
        expect(reader.run(operations)).to eq(10)
      end
    end
  end

  def make_op(op, val, visited = false)
    { op: op, val: val }
  end

  describe '#parse_line' do
    context 'if it receives a non-op line' do
      let(:no_op) { "nop +0\n" }

      it 'returns an instruction' do
        expect(reader.parse_line(no_op)).to eq(make_op('nop', 0))
      end
    end

    context 'if it receives an accumulate line' do
      let(:acc_op) { "acc +1\n" }

      it 'returns an instruction' do
        expect(reader.parse_line(acc_op)).to eq(make_op('acc', 1))
      end
    end

    context 'if it receives an accumulate line' do
      let(:jmp_op) { "jmp -3\n" }

      it 'returns an instruction' do
        expect(reader.parse_line(jmp_op)).to eq(make_op('jmp', -3))
      end
    end
  end

  describe 'process_op' do
    context 'given a no-op' do
      let(:op) { make_op('nop', 0) }

      it 'increments cursor' do
        expect(reader.process_op(op, 0, 0)).to eq([1, 0])
      end
    end

    context 'given an acc-op' do
      let(:op) { make_op('acc', 3) }

      it 'increments cursor and acc' do
        expect(reader.process_op(op, 0, 0)).to eq([1, 3])
      end
    end

    context 'given a jmp-op' do
      let(:op) { make_op('jmp', -3) }

      it 'increments cursor by the passed-in value' do
        expect(reader.process_op(op, 0, 0)).to eq([-3, 0])
      end
    end
  end
end
