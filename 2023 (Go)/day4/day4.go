package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strings"
)

func main() {
	var p1total = 0
	var p2total = 0

	data4 := utils.ReadFileLines("day4.txt")
	prizes := make([]int, len(data4))

	//No value initialization in go apparently
	for i := range prizes {
		prizes[i] = 1
	}

	for i := range data4 {
		amountWon := 0
		total := 0

		numbers := strings.Split(strings.Split(data4[i], ":")[1], "|")
		winning := strings.Fields(numbers[0])
		own := strings.Fields(numbers[1])

		for o := range own {
			for w := range winning {
				winningNumber := winning[w]
				ownNumber := own[o]

				if ownNumber == winningNumber {
					//P1
					if total == 0 {
						total++
					} else {
						total = total * 2
					}

					//P2
					prizes[i+1+amountWon] += prizes[i]
					amountWon++
				}
			}
		}

		p1total += total
	}

	for i := range prizes {
		p2total += prizes[i]
	}

	fmt.Println("P1: ", p1total)
	fmt.Println("P2: ", p2total)
}
