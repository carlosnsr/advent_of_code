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
    expect(count_trees(input)).to eq(7)
  end
end
