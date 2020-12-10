require './solution.rb'

RSpec.describe CodeReader do
  let(:reader) { CodeReader.new }
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

  describe '#initialize' do
    it 'has a zero cursor' do
      expect(reader.cursor).to eq(0)
    end

    it 'has a zero accumulator' do
      expect(reader.acc).to eq(0)
    end
  end

  describe '#run' do
    it 'calls #run_line for each line' do
      expect(reader).to receive(:run_line).and_call_original.at_most(input.size).times
      reader.run(input)
    end

    context 'if passed no input' do
      let(:input) { [] }

      it 'calculates the correct acc value' do
        expect(reader.run(input)).to eq(0)
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

      it 'calculates the correct acc value' do
        expect(reader.run(input)).to eq(0)
      end
    end

    context 'if passed input that has no jumps' do
      let(:input) do
        [
          "acc +1\n",
          "acc +2\n",
          "acc +3\n",
          "acc -1\n",
        ]
      end

      it 'calculates the correct acc value' do
        expect(reader.run(input)).to eq(5)
      end
    end

    context 'if passed input that jumps forward' do
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

      it 'calculates the correct acc value skipping values that were jumped over' do
        expect(reader.run(input)).to eq(10)
      end
    end

    context 'if passed input that jumps backward to an already-run instructions' do
      it 'stops processing and returns the value of acc at that point' do
        expect(reader.run(input)).to eq(5)
      end
    end
  end

  def make_op(op, val, visited = false)
    { op: op, val: val, visited: visited }
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
        expect { reader.process_op(op) }.to change { reader.cursor }.by(1)
      end

      it 'does not change acc' do
        expect { reader.process_op(op) }.not_to change { reader.acc }
      end
    end

    context 'given an acc-op' do
      let(:op) { make_op('acc', 3) }

      it 'increments cursor' do
        expect { reader.process_op(op) }.to change { reader.cursor }.by(1)
      end

      it 'increments acc by the passed-in value' do
        expect { reader.process_op(op) }.to change { reader.acc }.by(3)
      end
    end

    context 'given a jmp-op' do
      let(:op) { make_op('jmp', -3) }

      it 'increments cursor by the passed-in value' do
        expect { reader.process_op(op) }.to change { reader.cursor }.by(-3)
      end

      it 'does not change acc' do
        expect { reader.process_op(op) }.not_to change { reader.acc }
      end
    end
  end
end
