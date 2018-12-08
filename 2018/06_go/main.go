package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

type point struct {
	x, y int
}

func (pt point) distanceFrom(b point) int {
	return int(math.Abs(float64(pt.x-b.x)) + math.Abs(float64(pt.y-b.y)))
}

func areaChecker(allNodes []point) func(point, point) bool {
	return func(node, pt point) bool {
		for _, other := range allNodes {
			if other == node {
				continue
			}

			if node.distanceFrom(pt) >= other.distanceFrom(pt) {
				return false
			}
		}
		return true
	}
}

func readPointsFrom(filename string) (points []point) {
	bytes, err := ioutil.ReadFile(filename)
	if err != nil {
		print(err)
	}

	lines := strings.Split(string(bytes), "\n")
	lines = lines[:len(lines)-1]
	points = make([]point, len(lines))

	for i, line := range lines {
		coords := strings.Split(line, ", ")

		x, err := strconv.Atoi(coords[0])
		if err != nil {
			print(err)
		}

		y, err := strconv.Atoi(coords[1])
		if err != nil {
			print(err)
		}

		points[i] = point{x, y}
	}

	return
}

func includes(nodes []point, node point) bool {
	for _, pt := range nodes {
		if pt == node {
			return true
		}
	}
	return false
}

func filter(nodes []point, check func(point) bool) (result []point) {
	for _, node := range nodes {
		if check(node) {
			result = append(result, node)
		}
	}
	return
}

func max(slice []int) (result int) {
	result = slice[0]
	for _, x := range slice {
		if x > result {
			result = x
		}
	}
	return
}

func main() {
	nodes := readPointsFrom("input.txt")
	sameArea := areaChecker(nodes)

	bounds := []point{point{nodes[0].x, nodes[0].y}, point{nodes[0].x, nodes[0].y}}
	for _, pt := range nodes {
		if pt.x < bounds[0].x {
			bounds[0].x = pt.x
		}
		if pt.x > bounds[1].x {
			bounds[1].x = pt.x
		}
		if pt.y < bounds[0].y {
			bounds[0].y = pt.y
		}
		if pt.y > bounds[1].y {
			bounds[1].y = pt.y
		}
	}

	// Find all nodes with infinite areas
	var notGucciNodes []point
	for y := bounds[0].y; y <= bounds[1].y; y++ {
		for x := bounds[0].x; x <= bounds[1].x; x++ {
			if y != bounds[0].y && y != bounds[1].y && x > bounds[0].x && x < bounds[1].x {
				continue
			}

			pt := point{x, y}

			for _, node := range nodes {
				if sameArea(node, pt) {
					notGucciNodes = append(notGucciNodes, node)
					break
				}
			}
		}
	}

	gucciNodes := filter(nodes, func(node point) bool { return !includes(notGucciNodes, node) })
	areas := make([]int, len(gucciNodes))
	safepts := 0

	for y := bounds[0].y + 1; y < bounds[1].y; y++ {
		for x := bounds[0].x + 1; x < bounds[1].x; x++ {
			pt := point{x, y}

			for i, node := range gucciNodes {
				if sameArea(node, pt) {
					areas[i]++
					break
				}
			}

			distanceSum := 0
			for _, node := range nodes {
				distanceSum += pt.distanceFrom(node)
			}

			if distanceSum < 10000 {
				safepts++
			}
		}
	}

	fmt.Println("Part 1:", max(areas))
	fmt.Println("Part 2:", safepts)
}
