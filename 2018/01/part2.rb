changes = File.read("input.txt").split.map(&:to_i)
freq = 0
freqs = {0 => true}

changes.cycle.each do |change|
  freq += change
  break puts freq if freqs[freq]
  freqs[freq] = true
end
