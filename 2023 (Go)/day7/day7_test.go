package main

import (
	"fmt"
	"testing"
)

func TestIdentifyFive(t *testing.T) {
	hand := IdentifyHand([]int{1, 1, 1, 1, 1})
	if hand != Five {
		fmt.Println(hand)
		t.Fail()
	}

	hand = IdentifyHand([]int{10, 1, 10, 10, 10})
	if hand != Five {
		fmt.Println(hand)
		t.Fail()
	}
}

func TestIdentifyFour(t *testing.T) {
	hand := IdentifyHand([]int{2, 2, 2, 2, 7})
	if hand != Four {
		t.Fail()
	}

	hand = IdentifyHand([]int{5, 1, 1, 1, 6})
	if hand != Four {
		t.Fail()
	}
}

func TestIdentifyFullHouse(t *testing.T) {
	hand := IdentifyHand([]int{2, 2, 1, 7, 7})
	if hand != FullHouse {
		t.Fail()
	}

	hand = IdentifyHand([]int{5, 2, 2, 2, 5})
	if hand != FullHouse {
		t.Fail()
	}
}
