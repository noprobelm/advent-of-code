require 'set'

mapper = { '^' => [0, 1], '>' => [1, 0], 'v' => [0, -1], '<' => [-1, 0] }
steps = File.open("#{File.dirname(__FILE__)}./data/3.txt") { |f| f.read.split('') }.map { |c| mapper[c] }

def part1(steps)
  visited = Set[[0, 0]]
  location = [0, 0]

  steps.each do |i|
    location = [location[0] + i[0], location[1] + i[1]]
    visited.add location
  end
  visited
end

def part2(steps)
  santa_location = [0, 0]
  robo_location = [0, 0]
  visited = Set[[0, 0]]

  (0..steps.length - 1).each do |i|
    if i.even?
      santa_location = [santa_location[0] + steps[i][0], santa_location[1] + steps[i][1]]
    else
      robo_location = [robo_location[0] + steps[i][0], robo_location[1] + steps[i][1]]
    end

    visited.add santa_location
    visited.add robo_location
  end
  visited
end

visited = part1(steps)
puts visited.length

visited = part2(steps)
puts visited.length
