package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strconv"
	"strings"
)

func parseP1(line string) []int {
	sCol := strings.Fields(strings.Split(line, ":")[1])
	returnSlice := make([]int, 0)

	for _, s := range sCol {
		i, _ := strconv.Atoi(s)
		returnSlice = append(returnSlice, i)
	}

	return returnSlice
}

func parseP2(line string) []int {
	sCol := strings.Fields(strings.Split(line, ":")[1])
	returnSlice := make([]int, 0)

	s := ""

	for _, val := range sCol {
		s += val
	}

	i, _ := strconv.Atoi(s)
	returnSlice = append(returnSlice, i)

	return returnSlice
}

func main() {
	data := utils.ReadFileLines("day6.txt")

	times := parseP1(data[0])
	distances := parseP1(data[1])

	fmt.Println("P1: ", calc(times, distances))

	times = parseP2(data[0])
	distances = parseP2(data[1])

	fmt.Println("P2: ", calc(times, distances))
}

func calc(times []int, distances []int) int {
	waysToWin := make([]int, len(times))

	for i, num := range times {
		for holdTime := 0; holdTime < num; holdTime++ {
			travelTime := num - holdTime
			travelSpeed := holdTime

			distance := travelTime * travelSpeed
			if distance > distances[i] {
				waysToWin[i]++
			}
		}
	}

	total := 1
	for _, val := range waysToWin {
		total *= val
	}

	return total
}
