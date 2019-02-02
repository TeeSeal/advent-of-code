class Main {
  static void main(String[] args) {
    def input = 556061

    def scores = [3, 7]
    def i = 0
    def j = 1

    def didFindPattern = false

    while (scores.size() < input + 10 || !didFindPattern) {
      def digits = digitsOf(scores[i] + scores[j])

      digits.each {
        scores.add(it)
        if (foundPattern(scores, input) && !didFindPattern) {
          didFindPattern = true
          println("Part 2: " + (scores.size() - input.toString().size()).toString())
        }
      }

      if (scores.size() == input + 10) println("Part 1: " + scores.takeRight(10).join())
      i = (i + scores[i] + 1) % scores.size()
      j = (j + scores[j] + 1) % scores.size()
    }
  }

  static boolean foundPattern(scores, num) {
    return scores.takeRight(num.toString().size()).join().toInteger() == num
  }

  static int[] digitsOf(int num) {
    def digits = [num % 10]
    if (num > 9) digits.add(0, num / 10)
    return digits
  }
}
