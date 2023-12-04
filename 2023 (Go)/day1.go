package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strconv"
	"unicode"
)

func main() {
	input := utils.ReadFileLines("data/day1.txt")
	output := make([]int, len(input))

	for i := range input {
		first, last := '0', '0'
		runes := []rune(input[i])
		for j := range runes {
			isNumber, number := checkForNumber(j, runes)
			if isNumber {
				if first == '0' {
					first = number
				}

				last = number
			}
		}

		output[i], _ = strconv.Atoi(string(first) + string(last))
	}

	sum := 0
	for i := range output {
		sum += output[i]
	}

	fmt.Print(sum)
}

// She's real a beauty
func checkForNumber(index int, runes []rune) (isNumber bool, number rune) {
	if unicode.IsDigit(runes[index]) {
		return true, runes[index]
	}

	if len(runes) >= index+3 && string(runes[index:index+3]) == "one" {
		return true, '1'
	}

	if len(runes) >= index+3 && string(runes[index:index+3]) == "two" {
		return true, '2'
	}

	if len(runes) >= index+5 && string(runes[index:index+5]) == "three" {
		return true, '3'
	}

	if len(runes) >= index+4 && string(runes[index:index+4]) == "four" {
		return true, '4'
	}

	if len(runes) >= index+4 && string(runes[index:index+4]) == "five" {
		return true, '5'
	}

	if len(runes) >= index+3 && string(runes[index:index+3]) == "six" {
		return true, '6'
	}

	if len(runes) >= index+5 && string(runes[index:index+5]) == "seven" {
		return true, '7'
	}

	if len(runes) >= index+5 && string(runes[index:index+5]) == "eight" {
		return true, '8'
	}

	if len(runes) >= index+4 && string(runes[index:index+4]) == "nine" {
		return true, '9'
	}

	return false, '0'
}
