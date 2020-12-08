require './solution'

describe CustomsCounter do
  let(:counter) { CustomsCounter.new }

  describe '#initialize' do
    it 'starts with zero count' do
      expect(counter.count).to eq(0)
    end

    it 'starts with empty group tally' do
      expect(counter.group_tally).to be_empty
    end

    it 'starts with zero common count' do
      expect(counter.common_count).to eq(0)
    end

    it 'starts with zero group count' do
      expect(counter.group_count).to eq(0)
    end
  end

  describe 'load' do
    context 'if passed a non-empty line' do
      let(:line) { 'abc' }

      it 'calls set_count' do
        expect(counter).to receive(:add_to_group).with(line)
        counter.load(line)
      end

      it 'does not call tally_group' do
        expect(counter).not_to receive(:tally_group)
        counter.load(line)
      end

      context 'if passed a line with whitespace' do
        let(:wpline) { "  #{line}   \n" }

        it 'strips the whitespace a' do
          expect(counter).to receive(:add_to_group).with(line)
          counter.load(wpline)
        end
      end

      context 'if passed a line with just whitespace' do
        let(:line) { "   \n" }

        it 'calls tally_group' do
          expect(counter).to receive(:tally_group) { 0 }
          counter.load(line)
        end
      end
    end

    context 'if passed an empty line' do
      let(:line) { '' }
      let(:count) { 5 }

      before(:each) { allow(counter).to receive(:tally_group) { count } }

      it 'calls tally_group' do
        expect(counter).to receive(:tally_group)
        counter.load(line)
      end

      it 'does not call add_to_group' do
        expect(counter).not_to receive(:add_to_group)
        counter.load(line)
      end
    end
  end

  describe '#add_to_group' do
    let(:line) { 'abc' }

    it 'returns the number of unique characters' do
      expect(counter.add_to_group(line)).to eq(3)
    end

    it 'increments the group count' do
      expect { counter.add_to_group(line) }
        .to change { counter.group_count }.from(0).to(1)
    end

    context 'called with two lines' do
      let(:line2) { 'abd' }

      it 'returns the number of unique characters up to that point' do
        expect(counter.add_to_group(line)).to eq(3)
        expect(counter.add_to_group(line2)).to eq(4)
      end
    end
  end

  describe '#tally_group' do
    context 'if a line has already been loaded' do
      let(:line) { 'abc' }

      before(:each) do
        counter.load(line)
        counter.tally_group
      end

      it 'adds the number of unique answers to the count' do
        expect(counter.count).to eq(3)
      end

      it 'adds the number of common answers to the count' do
        expect(counter.common_count).to eq(3)
      end

      it 'resets the group_tally' do
        expect(counter.group_tally.size).to eq(0)
      end
    end
  end

  describe '#final_count' do
    it 'returns the final count' do
      expect(counter.final_count).to eq({ count: 0, common_count: 0 })
    end

    context 'if a group was counted but not added to the total yet' do
      let(:line) { 'abc' }

      before(:each) { counter.add_to_group(line) }

      it 'has a non empty group_tally' do
        expect(counter.group_tally.size).to eq(3)
      end

      it 'adds the last group to the count' do
        counter.final_count
        expect(counter.count).to eq(3)
      end

      it 'returns the final count (includes the last group)' do
        expect(counter.final_count).to eq({ count: 3, common_count: 3 })
      end
    end
  end
end

describe 'count_customs' do
  let(:group_input) { %w[abc abd abe] }

  context 'if passed a group result' do
    it 'returns the unique answers of the group' do
      expect(count_customs(group_input))
        .to eq({ count: 5, common_count: 2 })
    end
  end

  context 'if passed multiple groups' do
    let(:input) { [].concat(group_input, [""], group_input) }

    it "returns the sum of each group's unique answers" do
      expect(count_customs(input)).to eq({ count: 10, common_count: 4 })
    end
  end
end
