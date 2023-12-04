package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strings"
)

func main() {
	var p1total = 0
	var p2total = 0

	data4 := utils.ReadFileLines("../data/day4.txt")
	prizes := make([]int, len(data4))

	for i := range prizes {
		prizes[i] = 1
	}

	fmt.Println(len(data4))

	for i := range data4 {
		for j := 0; j < prizes[i]; j++ {

			total := 0
			amountWon := 0

			card := strings.Split(data4[i], ":")
			numbers := strings.Split(card[1], "|")

			winning := strings.Fields(numbers[0])
			own := strings.Fields(numbers[1])

			for o := range own {
				for w := range winning {
					winningNumber := winning[w]
					ownNumber := own[o]

					if ownNumber == winningNumber {
						//P1
						if j == 0 {
							if total == 0 {
								total++
							} else {
								total = total * 2
							}
						}

						//P2
						prizes[i+1+amountWon]++
						amountWon++
					}
				}
			}

			p1total += total
		}
	}

	for i := range prizes {
		p2total += prizes[i]
	}

	fmt.Println("P1: ", p1total)
	fmt.Println("P2: ", p2total)
}
