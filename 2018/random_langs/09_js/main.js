const Circle = require('./DoublyLinkedList')

function play(playerCount, marbleCount) {
  const circle = new Circle()
  const players = Array.from({ length: playerCount }, () => 0)

  for (let marble = 1; marble <= marbleCount; marble++) {
    if (marble % 23 === 0) {
      circle.rotateRight(7)
      players[marble % playerCount] += marble + circle.pop()
      circle.rotateLeft(1)
    } else {
      circle.rotateLeft(1)
      circle.push(marble)
    }
  }

  return Math.max(...players)
}

console.log(`Part 1: ${play(432, 71019)}`)
console.log(`Part 2: ${play(432, 71019 * 100)}`)
