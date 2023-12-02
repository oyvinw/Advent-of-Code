package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strconv"
	"strings"
)

const redLimit = 12
const greenLimit = 13
const blueLimit = 14

func main() {

	games := utils.ReadFile("../data/day2.txt")
	p1total, p2total := 0, 0

	//go through all games, check if there is a subset that exceeds the limits. Add the gameId if there isnt
	for i := range games {
		game := strings.Split(games[i], ":")
		p1total += p1(game)
		p2total += p2(game)
	}

	fmt.Println("Part 1: ", p1total)
	fmt.Println("Part 2: ", p2total)
}

func p1(game []string) int {
	gameId, _ := strconv.Atoi(strings.Fields(game[0])[1])
	subsets := strings.Split(game[1], ";")

	//check if the subset exceeds a limit
	for j := range subsets {
		numberColor := strings.Split(subsets[j], ",")

		for k := range numberColor {
			splitNumberColor := strings.Fields(numberColor[k])
			num, _ := strconv.Atoi(splitNumberColor[0])

			switch splitNumberColor[1] {
			case "blue":
				if num > blueLimit {
					return 0
				}
			case "green":
				if num > greenLimit {
					return 0
				}
			case "red":
				if num > redLimit {
					return 0
				}
			}
		}
	}

	return gameId
}

func p2(game []string) int {
	subsets := strings.Split(game[1], ";")
	lowestRed, lowestBlue, lowestGreen := 0, 0, 0

	//check if the subset exceeds a limit
	for j := range subsets {

		numberColor := strings.Split(subsets[j], ",")

		for k := range numberColor {
			splitNumberColor := strings.Fields(numberColor[k])
			num, _ := strconv.Atoi(splitNumberColor[0])

			switch splitNumberColor[1] {
			case "blue":
				if num > lowestBlue {
					lowestBlue = num
				}
			case "green":
				if num > lowestGreen {
					lowestGreen = num
				}
			case "red":
				if num > lowestRed {
					lowestRed = num
				}
			}
		}
	}

	return lowestRed * lowestGreen * lowestBlue
}
