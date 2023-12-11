package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
)

type Pos struct {
	x, y int
}

func main() {
	data := utils.ReadFileChars("day11.txt")
	galaxies := make([]Pos, 0)

	xAdjustedPostions := make(map[int]int, len(data))

	expansionAdjustedX := 0
	for x := range data[0] {
		hasGalaxies := false
		for y := range data {
			switch data[y][x] {
			case '#':
				hasGalaxies = true
			}
		}
		if !hasGalaxies {
			//replace with += 2 for p1
			expansionAdjustedX += 1000000
		} else {
			expansionAdjustedX++
		}
		xAdjustedPostions[x] = expansionAdjustedX
	}

	expansionAdjustedY := 0
	for _, row := range data {
		hasGalaxies := false
		for x, sym := range row {
			switch sym {
			case '#':
				hasGalaxies = true
				galaxies = append(galaxies, Pos{xAdjustedPostions[x], expansionAdjustedY})
			}
		}
		if !hasGalaxies {
			//replace with += 2 for p1
			expansionAdjustedY += 1000000
		} else {
			expansionAdjustedY++
		}
	}

	total := 0
	for i, this := range galaxies {
		for j, other := range galaxies {
			if j <= i {
				continue
			}
			total += distance(this, other)
		}
	}

	fmt.Println(total)
}

func distance(a, b Pos) int {
	return utils.Abs(a.y-b.y) + utils.Abs(a.x-b.x)
}
