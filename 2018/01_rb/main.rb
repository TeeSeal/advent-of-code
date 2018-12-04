changes = File.read("input.txt").split.map(&:to_i)

puts "Part 1:"
puts changes.sum

puts "Part 2:"
freq = 0
freqs = {0 => true}

changes.cycle.each do |change|
  freq += change
  break puts freq if freqs[freq]
  freqs[freq] = true
end
