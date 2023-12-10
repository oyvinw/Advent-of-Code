package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strconv"
	"strings"
)

func main() {
	data := utils.ReadFileLines("day9.txt")

	p1Total := 0
	p2Total := 0
	//Parse
	for _, row := range data {
		fields := strings.Fields(row)
		set := make([]int, len(fields))
		for j, sNumber := range fields {
			number, _ := strconv.Atoi(sNumber)
			set[j] = number
		}

		//solve set
		set = append([]int{set[0] - ExtrapolateBackwards(set)}, set...)
		p2Total += set[0]

		set = append(set, set[len(set)-1]+ExtrapolateForwards(set))
		p1Total += set[len(set)-1]
	}

	fmt.Println("P1:", p1Total)
	fmt.Println("P2:", p2Total)
}

func ExtrapolateForwards(row []int) int {
	currentRow := make([]int, len(row)-1)
	finished := true

	for i := 0; i < len(row)-1; i++ {
		currentRow[i] = row[i+1] - row[i]
		if currentRow[i] != 0 {
			finished = false
		}
	}
	if finished {
		return currentRow[len(currentRow)-1]
	}

	return currentRow[len(currentRow)-1] + ExtrapolateForwards(currentRow)
}

func ExtrapolateBackwards(row []int) int {
	currentRow := make([]int, len(row)-1)
	finished := true

	for i := len(row) - 1; i > 0; i-- {
		currentRow[i-1] = row[i] - row[i-1]
		if currentRow[i-1] != 0 {
			finished = false
		}
	}
	if finished {
		return currentRow[0]
	}

	return currentRow[0] - ExtrapolateBackwards(currentRow)
}
