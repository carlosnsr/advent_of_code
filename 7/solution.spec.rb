require './solution.rb'

RSpec.describe BagRules do
  let(:rules) { BagRules.new }
  let(:input) do
    [
      "vibrant salmon bags contain 1 vibrant gold bag, 2 wavy aqua bags, 1 dotted crimson bag.\n",
      "dotted plum bags contain 3 wavy cyan bags.\n",
      "muted salmon bags contain 2 pale purple bags, 3 dull orange bags, 2 dotted lime bags, 3 clear crimson bags.\n",
      "dark tomato bags contain no other bags.\n",
      "murdered violet bags contain 1 vibrant salmon bag.\n",
    ]
  end

  describe '#initialize' do
    it 'has an empty ruleset' do
      expect(rules.ruleset).to be_empty
    end

    it 'has an empty lookup' do
      expect(rules.lookup).to be_empty
    end
  end

  describe '#load' do
    it 'calls #extract_rule for each line of input' do
      expect(rules).to receive(:extract_rule)
        .and_return({ bag: 'bag', subbags: [] })
        .exactly(input.size).times
      rules.load(input)
    end

    it 'calls #update_ruleset for each line of input' do
      expect(rules).to receive(:update_ruleset)
        .and_return({ bag: 'bag', subbags: [] })
        .exactly(input.size).times
      rules.load(input)
    end

    it 'puts each rule into the ruleset' do
      rules.load(input)
      ruleset = rules.ruleset
    end
  end

  describe '#extract_rule' do
    context 'if given another rule with no sub-bags' do
      let(:rule) { input[3] }
      let(:bag) { 'dark tomato' }

      it 'extracts the container bag' do
        expect(rules.extract_rule(rule)[:bag]).to eq(bag)
      end

      it 'extracts empty sub-bags' do
        expect(rules.extract_rule(rule)[:subbags]).to be_empty
      end
    end

    context 'if given another rule with one sub-bag' do
      let(:rule) { input[1] }
      let(:bag) { 'dotted plum' }
      let(:subbags) { [{ colour: 'wavy cyan', number: 3 }] }

      it 'extracts the container bag' do
        expect(rules.extract_rule(rule)[:bag]).to eq(bag)
      end

      it 'returns the sub-bags' do
        expect(rules.extract_rule(rule)[:subbags]).to eq(subbags)
      end
    end

    context 'if given a rule with multiple sub-bags' do
      let(:rule) { input[0] }
      let(:bag) { 'vibrant salmon' }
      let(:subbags) do
        [
          { colour: 'vibrant gold', number: 1 },
          { colour: 'wavy aqua', number: 2 },
          { colour: 'dotted crimson', number: 1 },
        ]
      end

      it 'extracts the container bag' do
        expect(rules.extract_rule(rule)[:bag]).to eq(bag)
      end

      it 'returns a list of sub-bags' do
        expect(rules.extract_rule(rule)[:subbags]).to eq(subbags)
      end
    end
  end

  describe '#update_ruleset' do
    def make_rule(bag, subs)
      { bag: bag, subbags: subs }
    end

    before(:each) { rules.update_ruleset(make_rule(bag, subbags)) }

    context 'if given another rule with no sub-bags' do
      let(:bag) { 'dark tomato' }
      let(:subbags) { [] }

      it 'adds the rule to the ruleset' do
        expect(rules.ruleset[bag]).to eq(subbags)
      end

      it 'lookup is unchanged' do
        expect(rules.lookup).to be_empty
      end
    end

    context 'if given another rule with one sub-bag' do
      let(:bag) { 'dotted plum' }
      let(:subbags) { [{ colour: 'wavy cyan', number: 3 }] }

      it 'adds the rule to the ruleset' do
        expect(rules.ruleset[bag]).to eq(subbags)
      end

      it 'adds the subbag to the lookup' do
        expect(rules.lookup[subbags[0][:colour]]).to eq([bag])
      end
    end

    context 'if given a rule with multiple sub-bags' do
      let(:bag) { 'vibrant salmon' }
      let(:subbags) do
        [
          { colour: 'vibrant gold', number: 1 },
          { colour: 'wavy aqua', number: 2 },
          { colour: 'dotted crimson', number: 1 },
        ]
      end

      it 'adds the rule to the ruleset' do
        expect(rules.ruleset[bag]).to eq(subbags)
      end

      it 'adds each subbag to the lookup' do
        subbags.each do |subbag|
          expect(rules.lookup[subbag[:colour]]).to eq([bag])
        end
      end

      context 'if the same subbag is in another rule' do
        let(:bag2) { 'murdered violet' }
        let(:subbags2) { [{ colour: 'wavy aqua', number: 2 }] }

        before(:each) { rules.update_ruleset(make_rule(bag2, subbags2)) }

        it 'updates the lookup to include that rule' do
          expect(rules.lookup[subbags2[0][:colour]]).to eq([bag, bag2])
        end
      end
    end
  end

  describe '#can_contain' do
    context 'if ruleset is empty' do
      let(:colour) { 'flat birch' }

      it 'returns the directly as false' do
        expect(rules.can_contain(colour)[:directly]).to be_falsy
      end
    end

    context 'ruleset is populated' do
      before(:each) { rules.load(input) }

      context 'if colour can contain others' do
        let(:colour) { 'vibrant salmon' }

        it 'returns the directly as true' do
          expect(rules.can_contain(colour)[:directly]).to be_truthy
        end
      end

      context 'if colour is only in other colours' do
        let(:colour) { 'dotted lime' }

        it 'returns the directly as true' do
          expect(rules.can_contain(colour)[:directly]).to be_falsey
        end

        it 'returns the colours it can be contained in' do
          expect(rules.can_contain(colour)[:inside]).to eq(['muted salmon'])
        end
      end

      context 'if colour is in other colours, which themselves can be in other colours' do
        let(:colour) { 'vibrant gold' }

        it 'returns the directly as true' do
          expect(rules.can_contain(colour)[:directly]).to be_falsey
        end

        it 'returns the colours it can be contained in' do
          expect(rules.can_contain(colour)[:inside]).to eq(['vibrant salmon', 'murdered violet'])
        end
      end
    end
  end
end
