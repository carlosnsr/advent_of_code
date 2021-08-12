require './solution.rb'

RSpec.describe '#find_voltage_differences' do
  context 'given the test input' do
    TEST_INPUT = [
      28, 33, 18, 42, 31,
      14, 46, 20, 48, 47,
      24, 23, 49, 45, 19,
      38, 39, 11, 1, 32,
      25, 35, 8, 17, 7,
      9, 4, 2, 34, 10,
      3
    ]

    it 'the number of 1-volt and 3-volt differences' do
      expect(find_voltage_differences(TEST_INPUT)).to eq([22, 10])
    end
  end
end
