module day3

let data =
    System.IO.File.ReadLines("""C:\git\Advent-of-Code\2019 (F#)\solutions\day3\day3_input.txt""")
    |> Seq.map (_.Split(','))

let line (cornerPos : int * int) (instruction : string) =
    let steps = int instruction[1..]
    let cX, cY = cornerPos
    match instruction[0] with
        | 'R' -> seq {for x in cX.. (cX + steps) -> (x,cY)}
        | 'L' -> seq {for x in cX.. -1 .. (cX - steps) -> (x,cY)}
        | 'U' -> seq {for y in cY.. (cY + steps) -> (cX,y)}
        | 'D' -> seq {for y in cY.. -1 .. (cY - steps) -> (cX,y)}
        | _ -> failwith "direction out of range"

let rec nextLine (index : int) (prevCornerPos : int * int) (instructionStream : string array) : (int * int) seq =
    if index >= Seq.length instructionStream then
        []
    else
        let nl = line prevCornerPos instructionStream[index]
        (nextLine (index + 1) (Seq.last nl) instructionStream) |> Seq.append ( nl |> Seq.tail)

let manhattanDistance ((aX, aY) : int * int) ((bX, bY) : int * int) : int =
   abs ((aX - bX) + (aY - bY))

let drawWire (instructionStream : string array)  =
    nextLine 0 (0,0) instructionStream

let firstWire : (int * int) seq =
    drawWire (Seq.head data)

let secondWire : (int * int) seq =
    drawWire (Seq.last data)

let firstWireSet = Set.ofSeq (firstWire |> Seq.tail)
let secondWireSet = Set.ofSeq (secondWire |> Seq.tail)

let firstWireIndexed = firstWire |> Seq.indexed |> Seq.tail
let secondWireIndexed = secondWire |> Seq.indexed |> Seq.tail

let secondWireCrossings = secondWireIndexed |> Seq.filter (fun (_, pos) -> Set.contains pos firstWireSet)
let firstWireCrossings = firstWireIndexed |> Seq.filter (fun (_, pos) -> Set.contains pos secondWireSet)

// part 1
let part1 =
    secondWireCrossings |> Seq.map (fun (_, pos) -> manhattanDistance (0,0) pos)
    |> Seq.sort |> Seq.head |> printfn "part 1: %i"

let part2 =
    // order and zip wires together
    firstWireCrossings |> Seq.sortBy snd |> Seq.zip (secondWireCrossings |> Seq.sortBy snd)
    // calculate combined steps
    |> Seq.map (fun ((firstIndex, _), (secondIndex, _)) -> firstIndex + secondIndex + 2)
    // print smallest
    |> Seq.sort |> Seq.head |> (printfn "part 2: %i")