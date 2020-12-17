require './solution.rb'

describe '.count_trees' do
  let(:input) do
    [
      "..##.......\n",
      "#...#...#..\n",
      ".#....#..#.\n",
      "..#.#...#.#\n",
      ".#...##..#.\n",
      "..#.##.....\n",
      ".#.#.#....#\n",
      ".#........#\n",
      "#.##...#...\n",
      "#...##....#\n",
      ".#..#...#.#\n",
    ]
  end

  it 'should return the number of trees (#) hit' do
    expect(count_trees(input)).to eq([7])
  end

  context 'if given different path strategies' do
    let(:strategies) do
      [
        { lateral: 1, vertical: 1 },
        { lateral: 3, vertical: 1 },
        { lateral: 5, vertical: 1 },
        { lateral: 7, vertical: 1 },
        { lateral: 1, vertical: 2 },
      ]
    end

    it 'should return the number of trees (#) hit for each strategy' do
      expect(count_trees(input, strategies)).to eq([2, 7, 3, 4, 2])
    end
  end
end
