const std = @import("std");

test "left mod" {
    var current: i32 = 0;
    const codes = [_]i32{-1};

    for (codes) |code| {
        current = next(current, code);
    }

    try std.testing.expectEqual(99, current);
}

test "right mod" {
    var current: i32 = 99;
    const codes = [_]i32{1};

    for (codes) |code| {
        current = next(current, code);
    }

    try std.testing.expectEqual(0, current);
}

pub fn solve() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);

    const stdout = &stdout_writer.interface;

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    var data = try parseData(alloc);
    defer data.deinit(alloc);
    
    std.debug.print("part one: {d}\n", .{part1(data.items)});
    std.debug.print("part two: {d}\n", .{part2(data.items)});

    try stdout.flush(); // Don't forget to flush!
}

fn part1(codes: []i32) u32 {
    var current: i32 = 50;
    var count: u32 = 0;

    for (codes) |val| {
        current = next(current, val);
        if (current == 0) {
            count += 1;
        }
    }

    return count;
}

test "part one test" {
    var codes = [_]i32{ -68, -30, 48, -5, 60, -55, -1, -99, 14, -82 };
    const count = part1(&codes);
    try std.testing.expectEqual(3, count);
}

fn part2(codes: []i32) u32 {
    var current: i32 = 50;
    var count: u32 = 0;

    for (codes) |val| {
        count += countTimesPassedZero(current, val);
        current = next(current, val);
    }

    return count;
}

test "wraps depend on start, not just end" {
    // both end at 99 if you wrap mod 100
    // but wraps are different
    try std.testing.expectEqual(
        countTimesPassedZero(50, 49),
        0,
    );
    try std.testing.expectEqual(
        countTimesPassedZero(50, 349),
        3,
    );
}

test "wrap backward crosses 0 once" {
    const count = countTimesPassedZero(10, -30);
    try std.testing.expectEqual(1, count);
}

test "part two test" {
    var codes = [_]i32{ -68, -30, 48, -5, 60, -55, -1, -99, 14, -82 };
    const count = part2(&codes);
    try std.testing.expectEqual(6, count);
}

test "part two test big" {
    var codes = [_]i32{ -268, -30, 48, -5, 460, -55, -1, -99, 14, -82 };
    const count = part2(&codes);
    try std.testing.expectEqual(12, count);
}

fn countTimesPassedZero(pos: i32, instruction: i32) u32 {
    var offsetPos: u32 = 0;

    if (instruction < 0 and pos > 0){
        offsetPos = @abs(pos - 100);
    } else {
        offsetPos = @abs(pos);
    }

    const end = offsetPos + @abs(instruction);
    const count = @abs(@divFloor(end, 100));
    return count;
}

test "three times over from zero" {
    const count = countTimesPassedZero(0, 300);
    try std.testing.expectEqual(3, count);
}

test "three times over from 50" {
    const count = countTimesPassedZero(50, 349);
    try std.testing.expectEqual(3, count);
}

test "three times under from zero" {
    const count = countTimesPassedZero(0, -300);
    try std.testing.expectEqual(3, count);
}

test "three times under from 50" {
    const count = countTimesPassedZero(50, -349);
    try std.testing.expectEqual(3, count);
}

test "no overcount down" {
    const count = countTimesPassedZero(0, -1);
    try std.testing.expectEqual(0, count);
}

test "no overcount down exact" {
    const count = countTimesPassedZero(0, -100);
    try std.testing.expectEqual(1, count);
}

test "no overcount up" {
    const count = countTimesPassedZero(0, 1);
    try std.testing.expectEqual(0, count);
}

test "no overcount up exact" {
    const count = countTimesPassedZero(0, 100);
    try std.testing.expectEqual(1, count);
}

test "weird" {
    const count = countTimesPassedZero(10, -250);
    try std.testing.expectEqual(3, count);
}

fn next(pos: i32, instruction: i32) i32 {
    return @mod((pos + instruction), 100);
}


fn parseData(alloc: std.mem.Allocator) !std.ArrayList(i32) {
    const filename = "C:\\git\\Advent-of-Code\\2025 (Zig)\\src\\day6_input.txt";
    const delimiter = "\n";

    var codes: std.ArrayList(i32) = .empty;

    var file = try std.fs.cwd().openFile(filename, .{ .mode = .read_only });
    defer file.close();
    var read_buf: [2]u8 = undefined;
    var f_reader: std.fs.File.Reader = file.reader(&read_buf);

    var line = std.Io.Writer.Allocating.init(alloc);
    defer line.deinit();

    while (true) {
        _ = f_reader.interface.streamDelimiter(&line.writer, delimiter[0]) catch |err| {
            if (err == error.EndOfStream) break else return err;
        };
        _ = f_reader.interface.toss(1); // skip the delimiter byte.

        var s = line.written();
        if (s.len > 0 and s[s.len - 1] == '\r') {
            s = s[0 .. s.len - 1];
        }

        const num = try std.fmt.parseInt(i32, s[1..], 10);
        if (s[0] == 'L') {
            try codes.append(alloc, num);
        }

        if (s[0] == 'R') {
            try codes.append(alloc, -num);
        }

        line.clearRetainingCapacity(); // reset the accumulating buffer.
    }

    if (line.written().len > 0) {}

    return codes;
}
