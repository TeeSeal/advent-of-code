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

func (pt point) neighbors() []point {
	return []point{
		point{pt.x, pt.y - 1},
		point{pt.x, pt.y + 1},
		point{pt.x - 1, pt.y},
		point{pt.x + 1, pt.y},
	}
}

func (pt point) distanceFrom(b point) int {
	return int(math.Abs(float64(pt.x-b.x)) + math.Abs(float64(pt.y-b.y)))
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

func contains(s []point, e point) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

func inArea(pt, base point, all []point) (result bool) {
	for _, other := range all {
		if other == base {
			continue
		}

		if base.distanceFrom(pt) >= other.distanceFrom(pt) {
			return false
		}
	}
	return true
}

func outOfBounds(pt point, bounds []point) bool {
	return pt.x == bounds[0].x || pt.y == bounds[0].y || pt.x == bounds[1].x || pt.y == bounds[1].y
}

func check(pt, base point, all, used, bounds []point) (result int, newUsed []point) {
	result = 0
	newUsed = used

	if pt == base || contains(newUsed, pt) || !inArea(pt, base, all) {
		return
	}

	if outOfBounds(pt, bounds) {
		// infinite area
		return -1, newUsed
	}

	newUsed = append(newUsed, pt)
	result++

	var add int
	for _, neighbor := range pt.neighbors() {
		add, newUsed = check(neighbor, base, all, newUsed, bounds)
		if add == -1 {
			return -1, newUsed
		}
		result += add
	}

	return
}

func main() {
	all := readPointsFrom("input.txt")

	bounds := []point{point{all[0].x, all[0].y}, point{all[0].x, all[0].y}}
	for _, pt := range all {
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

	used := make([]point, len(all))
	copy(used, all)

	var add int
	var areas []int

	for _, pt := range all {
		if outOfBounds(pt, bounds) {
			continue
		}

		area := 1
		infinite := false

		for _, neighbor := range pt.neighbors() {
			add, used = check(neighbor, pt, all, used, bounds)

			if add == -1 {
				infinite = true
				break
			}

			area += add
		}

		if !infinite {
			areas = append(areas, area)
		}
	}

	max := 0
	for _, area := range areas {
		if area > max {
			max = area
		}
	}

	fmt.Println("Part 1:", max)

	var safepts []point

	for i := bounds[0].x + 1; i < bounds[1].x; i++ {
		for j := bounds[0].y + 1; j < bounds[1].y; j++ {
			pt := point{i, j}

			sum := 0
			for _, node := range all {
				sum += pt.distanceFrom(node)
			}

			if sum < 10000 {
				safepts = append(safepts, pt)
			}
		}
	}

	fmt.Println("Part 2:", len(safepts))
}
