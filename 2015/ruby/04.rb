require 'digest'

contents = File.open("#{File.dirname(__FILE__)}./data/4.txt") { |f| f.read.chomp }

def part1(contents)
  n = 0
  secret = Digest::MD5.hexdigest("#{contents}#{n}")
  until secret[0..4].eql? '00000'
    n += 1
    secret = Digest::MD5.hexdigest("#{contents}#{n}")
  end
  n
end

def part2(contents)
  n = 0
  secret = Digest::MD5.hexdigest("#{contents}#{n}")
  until secret[0..5].eql? '000000'
    n += 1
    secret = Digest::MD5.hexdigest("#{contents}#{n}")
  end
  n
end

n = part1(contents)
puts n

n = part2(contents)
puts n
