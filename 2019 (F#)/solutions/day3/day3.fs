module day3

let data =
    System.IO.File.ReadLines("""C:\git\Advent-of-Code\2019 (F#)\solutions\solutions\day3\day3_input.txt""")
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
        (nextLine (index + 1) (Seq.last nl) instructionStream) |> Seq.append nl

let manhattanDistance ((aX, aY) : int * int) ((bX, bY) : int * int) : int =
   abs ((aX - bX) + (aY - bY))

let drawWire (instructionStream : string array)  =
    nextLine 0 (0,0) instructionStream

let firstWire : (int * int) seq =
    drawWire (Seq.head data)

let secondWire : (int * int) seq =
    drawWire (Seq.last data)

// part 1
let firstWireSet = Set.ofSeq (firstWire |> Seq.tail)
secondWire
|> Seq.filter (fun x -> Set.contains x firstWireSet)
|> Seq.map (manhattanDistance (0,0))
|> Seq.sort
|> Seq.head
|> printfn "%i"
