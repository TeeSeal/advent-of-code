class Node {
  constructor(number = 0) {
    this.n = number
    this.prev = null
    this.next = null
  }
}

class DoublyLinkedList {
  constructor() {
    const marble = new Node()
    this.first = marble
    this.last = marble
  }

  push(number) {
    const marble = new Node(number)
    this.last.next = marble
    marble.prev = this.last
    this.last = marble
  }

  pop() {
    const marble = this.last
    this.last = marble.prev
    this.last.next = null
    return marble.n
  }

  rotateRight(amount) {
    if (this.first.n == this.last.n) return
    for (let i = 0; i < amount; i++) {
      const marble = this.last
      this.last = marble.prev
      this.last.next = null

      marble.next = this.first
      marble.prev = null
      this.first.prev = marble
      this.first = marble
    }
  }

  rotateLeft(amount) {
    if (this.first.n == this.last.n) return
    for (let i = 0; i < amount; i++) {
      const marble = this.first
      this.first = marble.next
      this.first.prev = null

      marble.prev = this.last
      marble.next = null
      this.last.next = marble
      this.last = marble
    }
  }
}

module.exports = DoublyLinkedList

