package main

import (
	"cmp"
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"slices"
	"strconv"
	"strings"
)

type HandType int

const (
	Five HandType = iota
	Four
	FullHouse
	Three
	TwoPair
	Pair
	HighCard
)

type Hand struct {
	Cards    []int
	HandType HandType
	Bid      int
}

func main() {
	data := utils.ReadFileLines("day7.txt")
	hands := make([]Hand, len(data))

	for i, line := range data {
		split := strings.Fields(line)
		bid, _ := strconv.Atoi(split[1])
		cards := make([]int, 5)
		for i, v := range split[0] {
			strv := string(v)
			num, err := strconv.Atoi(strv)
			if err != nil {
				switch strv {
				case "A":
					num = 14
				case "K":
					num = 13
				case "Q":
					num = 12
				case "J":
					//num = 11 //P1
					num = 1 //P2
				case "T":
					num = 10
				}
			}
			cards[i] = num
		}

		handType := IdentifyHand(cards)
		hands[i] = Hand{cards, handType, bid}
	}

	handCmp := func(b, a Hand) int {
		comp := cmp.Compare(a.HandType, b.HandType)
		if comp == 0 {
			for i := range a.Cards {
				comp = cmp.Compare(b.Cards[i], a.Cards[i])
				if comp != 0 {
					break
				}
			}
		}

		return comp
	}

	slices.SortFunc(hands, handCmp)
	p1Total := 0

	for i, v := range hands {
		p1Total += v.Bid * (i + 1)
		fmt.Println("Hand type:", v.HandType, "Cards:", v.Cards, "Bid:", v.Bid)
	}

	fmt.Println(p1Total)
}

// Filth, but probably efficient (?????)
func IdentifyHand(hand []int) HandType {
	handMap := make(map[int]int)

	for _, v := range hand {
		handMap[v]++
	}

	//P2
	numberOfJokers := handMap[1]
	if numberOfJokers == 5 {
		return Five
	}

	delete(handMap, 1)

	setSize := 0

Exit:
	for i, set := range handMap {
		switch set {
		case 5:
			return Five
		case 4:
			setSize = 4
			break Exit
		case 3:
			if containsValue(handMap, 2, i) {
				return FullHouse
			}
			setSize = 3
			break Exit
		case 2:
			if containsValue(handMap, 3, i) {
				if numberOfJokers <= 0 {
					return FullHouse
				}
				setSize = 2
				break Exit
			}
			if containsValue(handMap, 2, i) {
				if numberOfJokers > 0 {
					return FullHouse
				}
				return TwoPair
			}
			setSize = 2
			break Exit
		}
	}

	if len(handMap) == 5-numberOfJokers {
		setSize = 1
		fmt.Println("setsize:", setSize, "jokers:", numberOfJokers)
	}

	setSize += numberOfJokers
	switch setSize {
	case 5:
		return Five
	case 4:
		return Four
	case 3:
		return Three
	case 2:
		return Pair
	}

	return HighCard
}

func containsValue(sMap map[int]int, num int, excluding int) bool {
	for i, v := range sMap {
		if i != excluding && v == num {
			return true
		}
	}
	return false
}
