require './solution.rb'

RSpec.describe PassportValidator do
  let(:validator) { PassportValidator.new }
  let(:input) do
    [
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n",
      "byr:1937 iyr:2017 cid:147 hgt:183cm\n",
      "\n",
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n",
      "hcl:#cfa07d byr:1929\n",
      "\n",
      "hcl:#ae17e1 iyr:2013\n",
      "eyr:2024\n",
      "ecl:brn pid:760753108 byr:1931\n",
      "hgt:179cm\n",
      "\n",
      "hcl:#cfa07d eyr:2025 pid:166559648\n",
      "iyr:2011 ecl:brn hgt:59in\n",
    ]
  end

  describe '#initialize' do
    it 'has 0 count' do
      expect(validator.count).to eq(0)
    end

    it 'has 0 valid' do
      expect(validator.valid).to eq(0)
    end
  end

  describe '#count' do
    it 'counts the number of passports read in' do
      expect { validator.load(input) }.to change { validator.count }.by(4)
    end

    it 'calls #validate for each passport' do
      expect(validator).to receive(:validate).exactly(4).times
      validator.load(input)
    end

    it 'counts the number of valid passports read in' do
      expect { validator.load(input) }.to change { validator.valid }.by(2)
    end
  end

  describe '#validate' do
    context 'passed a passport with all the fields' do
      let(:passport) do
        [
          "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
          "byr:1937 iyr:2017 cid:147 hgt:183cm",
        ]
      end

      it 'returns true' do
        expect(validator.validate(passport)).to be_truthy
      end

      it 'increments #valid' do
        expect { validator.validate(passport) }.to change { validator.valid }.by(1)
      end

      it 'increments #count' do
        expect { validator.validate(passport) }.to change { validator.count }.by(1)
      end
    end

    context 'passed a passport missing hgt' do
      let(:passport) do
        [
          "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
          "hcl:#cfa07d byr:1929",
        ]
      end

      it 'returns false' do
        expect(validator.validate(passport)).to be_falsey
      end

      it 'does not change #valid' do
        expect { validator.validate(passport) }.not_to change { validator.valid }
      end

      it 'increments #count' do
        expect { validator.validate(passport) }.to change { validator.count }.by(1)
      end
    end

    context 'passed a passport missing only the cid field' do
      let(:passport) do
        [
          "hcl:#ae17e1 iyr:2013",
          "eyr:2024",
          "ecl:brn pid:760753108 byr:1931",
          "hgt:179cm",
        ]
      end

      it 'returns true' do
        expect(validator.validate(passport)).to be_truthy
      end

      it 'increments #valid' do
        expect { validator.validate(passport) }.to change { validator.valid }.by(1)
      end

      it 'increments #count' do
        expect { validator.validate(passport) }.to change { validator.count }.by(1)
      end
    end

    context 'passed a passport missing cid and byr' do
      let(:passport) do
        [
          "hcl:#cfa07d eyr:2025 pid:166559648",
          "iyr:2011 ecl:brn hgt:59in",
        ]
      end

      it 'returns false' do
        expect(validator.validate(passport)).to be_falsey
      end

      it 'does not change #valid' do
        expect { validator.validate(passport) }.not_to change { validator.valid }
      end

      it 'increments #count' do
        expect { validator.validate(passport) }.to change { validator.count }.by(1)
      end
    end
  end
end
