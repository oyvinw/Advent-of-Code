package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"github.com/schollz/progressbar/v3"
	"math"
	"strconv"
	"strings"
	"time"
)

type SeedMapEntry struct {
	DestinationStart int
	SourceStart      int
	Range            int
}

func (s SeedMapEntry) DestinationEnd() int {
	return s.DestinationStart + (s.Range - 1)
}

func (s SeedMapEntry) SourceEnd() int {
	return s.SourceStart + (s.Range - 1)
}

func (s SeedMapEntry) evaluate(i int) int {
	return (i - s.SourceStart) + s.DestinationStart
}

func main() {
	data := utils.ReadFileLines("day5.txt")
	seeds := make([]int, 0)
	currentMapParseIndex := -1

	var farmingMaps = make([][]SeedMapEntry, 7)

	//Parse and create maps
	for i := 0; i < len(data); i++ {
		line := data[i]

		if i == 0 {
			seedsData := strings.Fields(strings.Split(line, ":")[1])
			for s := range seedsData {
				seed, _ := strconv.Atoi(seedsData[s])
				seeds = append(seeds, seed)
			}
			continue
		}

		if len(line) == 0 {
			currentMapParseIndex++
			i++
			continue
		}

		mappings := strings.Fields(line)

		destStart, _ := strconv.Atoi(mappings[0])
		srcStart, _ := strconv.Atoi(mappings[1])
		rangeLen, _ := strconv.Atoi(mappings[2])

		seedEntry := SeedMapEntry{
			DestinationStart: destStart,
			SourceStart:      srcStart,
			Range:            rangeLen,
		}

		farmingMaps[currentMapParseIndex] = append(farmingMaps[currentMapParseIndex], seedEntry)
	}

	lowest := math.MaxInt

	//P1
	for s := range seeds {
		val := evaluateSeedThroughMaps(seeds[s], farmingMaps)
		if val < lowest {
			lowest = val
		}
	}

	fmt.Println("P1: ", lowest)

	//P2 (Neanderthal implementation)
	/*
		for s := 0; s < len(seeds); s += 2 {
			for i := seeds[s]; i < seeds[s]+seeds[s+1]; i++ {
				val := evaluateSeedThroughMaps(i)
				if val < lowest {
					lowest = val
				}
			}
		}
	*/

	//P2 (Cro-Magnon implementation)
	start := time.Now()

	lowest = math.MaxInt

	results := make(chan int)

	chunkSize := math.MaxInt
	chunks := make([]int, 0)

	for s := 0; s < len(seeds); s += 2 {
		nv := seeds[s+1]
		if nv < chunkSize {
			chunkSize = nv - 1
		}
	}

	//Chunk it up
	for s := 0; s < len(seeds); s += 2 {
		low, high := seeds[s], seeds[s]+seeds[s+1]
		for i := low; i+chunkSize < high; i += chunkSize {
			chunks = append(chunks, i)
			chunks = append(chunks, (i+chunkSize)-1)
		}

		chunks = append(chunks, chunks[len(chunks)-1]+1)
		chunks = append(chunks, high-1)
	}

	bar := progressbar.NewOptions(len(chunks)/2,
		progressbar.OptionUseANSICodes(true),
		progressbar.OptionSetElapsedTime(true),
		progressbar.OptionFullWidth())

	for s := 0; s < len(chunks); s += 2 {
		fMap := make([][]SeedMapEntry, len(farmingMaps))
		copy(fMap, farmingMaps)

		go func(s int, low int, high int, fMap [][]SeedMapEntry) {
			thisLowest := math.MaxInt
			for i := low; i <= high; i++ {
				val := evaluateSeedThroughMaps(i, fMap)
				if val < thisLowest {
					thisLowest = val
				}
			}
			_ = bar.Add(1)
			results <- thisLowest
		}(s, chunks[s], chunks[s+1], fMap)
	}

	for s := 0; s < len(chunks); s += 2 {
		r := <-results
		if r < lowest {
			lowest = r
		}
	}

	fmt.Println("P2 finished in", time.Now().Sub(start).Milliseconds(), "ms.")
	fmt.Println("P2: ", lowest)
}

func evaluateSeedThroughMaps(val int, farmingMaps [][]SeedMapEntry) int {
	for i := range farmingMaps {
		currentMap := farmingMaps[i]
		for j := range currentMap {
			currentMapEntry := currentMap[j]
			if val >= currentMapEntry.SourceStart && val <= currentMapEntry.SourceEnd() {
				val = currentMapEntry.evaluate(val)
				break
			}
		}
	}
	return val
}
