require './src/solution.rb'

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
      enumerator = load_code(TEST_INPUT)
      expect(find_voltage_differences(enumerator)).to eq([22, 10])
    end
  end

  it 'for array, assume 3-difference between the outlet and the final device' do
    expect(find_voltage_differences([])).to eq([0, 1])
  end

  it 'compares the first value to 0 (the outlet) and then +3 (the device)' do
    expect(find_voltage_differences([1])).to eq([1, 1])
    expect(find_voltage_differences([2])).to eq([0, 1])
    expect(find_voltage_differences([3])).to eq([0, 2])
  end

  it 'can process multi-item arrays' do
    expect(find_voltage_differences([1, 2, 3])).to eq([3, 1])
    expect(find_voltage_differences([3, 6, 9])).to eq([0, 4])
  end

  it 'can processes unsorted arrays' do
    expect(find_voltage_differences([3, 1, 2])).to eq([3, 1])
    expect(find_voltage_differences([3, 9, 6])).to eq([0, 4])
  end
end
