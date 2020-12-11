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

    it 'has 0 actually valid' do
      expect(validator.actually_valid).to eq(0)
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

      it 'increments #actually_valid' do
        expect { validator.validate(passport) }.to change { validator.actually_valid }.by(1)
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

      it 'increments #actually_valid' do
        expect { validator.validate(passport) }.to change { validator.actually_valid }.by(1)
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

  describe '#validate_year' do
    let(:min) { 1920 }
    let(:max) { 2002 }

    it 'returns false if year is < min' do
      expect(validator.validate_year('1919', min, max)).to be_falsey
    end

    it 'returns false if year is > max' do
      expect(validator.validate_year('2003', min, max)).to be_falsey
    end

    it 'returns false if year is not a number' do
      expect(validator.validate_year('hello', min, max)).to be_falsey
    end

    it 'returns true if year is a number inclusively between min and max' do
      expect(validator.validate_year('1920', min, max)).to be_truthy
      expect(validator.validate_year('2002', min, max)).to be_truthy
      expect(validator.validate_year('1980', min, max)).to be_truthy
    end
  end

  describe 'validate_hgt' do
    it 'returns true if height is in cm or inches' do
      expect(validator.validate_hgt('150cm')).to be_truthy
      expect(validator.validate_hgt('59in')).to be_truthy
    end

    it 'returns false if height is not cm or inches' do
      expect(validator.validate_hgt('149')).to be_falsey
    end

    it 'returns true if height is in cm and too small or big' do
      expect(validator.validate_hgt('149cm')).to be_falsey
      expect(validator.validate_hgt('194cm')).to be_falsey
    end

    it 'returns true if height is in inches and too small or big' do
      expect(validator.validate_hgt('58in')).to be_falsey
      expect(validator.validate_hgt('77in')).to be_falsey
    end
  end

  describe 'validate_hcl' do
    it 'returns true if passed a # and six digits' do
      expect(validator.validate_hcl('#000000')).to be_truthy
      expect(validator.validate_hcl('#000999')).to be_truthy
      expect(validator.validate_hcl('#123456')).to be_truthy
    end

    it 'returns false otherwise' do
      expect(validator.validate_hcl('149')).to be_falsey
      expect(validator.validate_hcl('999999')).to be_falsey
      expect(validator.validate_hcl('#12345')).to be_falsey
    end
  end
end
