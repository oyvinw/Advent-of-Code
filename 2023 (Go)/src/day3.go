package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strconv"
	"unicode"
)

type Pos struct {
	y int
	x int
}

var data [][]rune
var gearMap map[Pos][]int
var currentNumberInt int

func main() {
	data = utils.ReadFileChars("../data/day3.txt")
	parsingNumber := false
	currentNumber := ""
	numberPositions := make([]Pos, 0)
	p1Total, p2Total := 0, 0

	gearMap = make(map[Pos][]int)

	for y := range data {
		for x := range data[y] {
			if unicode.IsDigit(data[y][x]) {
				if !parsingNumber {
					//start new number parsing
					parsingNumber = true
					currentNumber = ""
					numberPositions = make([]Pos, 0)
				}

				if parsingNumber {
					//continue number parsing
					currentNumber += string(data[y][x])
					numberPositions = append(numberPositions, Pos{y, x})
				}
			}

			if x == len(data[y]) || !unicode.IsDigit(data[y][x]) {
				if parsingNumber {
					//number finished parsing. Check all neighbours of numbers for symbols other than '.'
					parsingNumber = false
					currentNumberInt, _ = strconv.Atoi(currentNumber)

					if !checkNumberForNeighbourSymbols(numberPositions) {
						p1Total += currentNumberInt
					}

				} else {
					//just another non-number, we don't care
				}
			}
		}
	}

	for k, v := range gearMap {
		if len(v) == 2 {
			p2Total += v[0] * v[1]
		}
	}

	fmt.Println("P1: ", p1Total)
	fmt.Println("P2: ", p2Total)
}

func checkNumberForNeighbourSymbols(digitPositions []Pos) bool {
	thisRow := digitPositions[0].y
	rowBelow := thisRow + 1
	rowAbove := thisRow - 1

	start, end := digitPositions[0].x, digitPositions[len(digitPositions)-1].x

	returnVal := true

	if start != 0 {
		start--
	}

	if !(end == len(data[thisRow])-1) {
		end++
	}

	if !(0 > rowAbove) {
		//Safe to Check above
		if !(searchRow(rowAbove, start, end)) {
			returnVal = false
		}
	}

	//Check this row
	if !(searchRow(thisRow, start, end)) {
		returnVal = false
	}

	if len(data) > rowBelow {
		//Safe to check below
		if !(searchRow(rowBelow, start, end)) {
			returnVal = false
		}
	}

	return returnVal
}

func searchRow(row int, start int, end int) bool {
	returnVal := true

	for x := start; x <= end; x++ {
		value := data[row][x]
		if string(value) != "." && !unicode.IsDigit(value) {
			if string(value) == "*" {
				gearPos := Pos{row, x}
				_, ok := gearMap[gearPos]
				if !ok {
					gearMap[gearPos] = make([]int, 0)
				}
				gearMap[gearPos] = append(gearMap[gearPos], currentNumberInt)
			}
			returnVal = false
		}
	}

	return returnVal
}
