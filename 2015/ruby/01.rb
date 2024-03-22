chars = File.open("#{File.dirname(__FILE__)}./data/1.txt") { |f| f.read.split("") }

def part1(chars)
  floor = 0
  chars.each do |c|
    case c
    when '('
      floor += 1
    when ')'
      floor -= 1
    end
  end
  puts "Final floor reached is #{floor}"
end

def part2(chars)
  floor = 0
  i = 0
  chars.each do |c|
    case c
    when '('
      floor += 1
    when ')'
      floor -= 1
    end
    i += 1
    break if floor == -1
  end
  puts "Basement is first entered on step #{i}"
end

part1(chars)
part2(chars)
