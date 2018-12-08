#!/usr/bin/swift

import Foundation

typealias Metadata = [Int]
typealias Nodelist = [Node]

class Node {
    var children: Nodelist = []
    var metaData: Metadata = []

    func dataSum() -> Int {
        let ownSum = metaData.reduce(0, +)
        return children.map { $0.dataSum() }.reduce(ownSum, +)
    }

    func value() -> Int {
        if children.count == 0 { return dataSum() }

        var sum = 0
        for idx in metaData {
            if let child = childAt(idx - 1) { sum += child.value() }
        }

        return sum
    }

    func childAt(_ index: Int) -> Node? {
        if index >= children.count { return nil }
        return children[index]
    }

    static func from(_ numbers: inout [Int]) -> Node {
        let childCount = numbers.removeFirst()
        let dataCount = numbers.removeFirst()

        let node = Node.init()
        for _ in 0..<childCount {
            node.children.append(Node.from(&numbers))
        }

        for _ in 0..<dataCount {
            node.metaData.append(numbers.removeFirst())
        }

        return node
    }
}

let input = try String(contentsOf: URL(fileURLWithPath: "input.txt"), encoding: .utf8).trimmingCharacters(in: .whitespacesAndNewlines)
var numbers = input.split(separator: " ").map { Int($0)! }
let top = Node.from(&numbers)

print("Part 1: \(top.dataSum())")
print("Part 2: \(top.value())")
