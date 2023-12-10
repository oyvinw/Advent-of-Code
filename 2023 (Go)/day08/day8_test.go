package main

import "testing"

func TestAllPathsAtDestination(t *testing.T) {
	if !pathAtDestination("FGZ") {
		t.Fail()
	}

	if pathAtDestination("KKR") {
		t.Fail()
	}
}
