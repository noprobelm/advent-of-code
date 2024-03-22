def part1(lines)
  wrapping_paper = 0
  lines.each do |d|
    d = d.sort
    wrapping_paper += (d[0] * d[1] * 2) + (d[0] * d[2] * 2) + (d[1] * d[2] * 2) + (d[0] * d[1])
  end
  wrapping_paper
end

def part2(lines)
  ribbon = 0
  lines.each do |d|
    d = d.sort
    ribbon += (d[0] * 2) + (d[1] * 2) + d[0] * d[1] * d[2]
  end
  ribbon
end

lines = File.open("#{File.dirname(__FILE__)}./data/2.txt") \
  { |f| f.readlines.map { |l| l.strip.split('x').map(&:to_i) } }

wrapping_paper = part1(lines)
ribbon = part2(lines)

puts wrapping_paper
puts ribbon
