const std = @import("std");
const math = @import("std").math;

const allocator = std.heap.page_allocator;

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var inputFile = try std.fs.cwd().openFile("src/d03/input.txt", .{ .mode = .read_only });
    defer inputFile.close();

    const file_size = (try inputFile.stat()).size;

    var buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);
    try inputFile.reader().readNoEof(buffer);

    const result = try computePrioritiesSum(buffer);

    try stdout.print("{d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}

fn getFirstIntersectedChar(a: []const u8, b: []const u8) !?u8 {
    var charset_a = std.AutoHashMap(u8, void).init(allocator);
    defer charset_a.deinit();

    for (a, 0..) |c, i| {
        _ = i;
        try charset_a.put(c, {});
    }

    for (b, 0..) |c, i| {
        _ = i;
        if (charset_a.contains(c)) {
            return c;
        }
    }

    return null;
}

const ComputePrioritiesSumError = error{NoRepeatedItemFoundInLine};

fn computePrioritiesSum(input: []const u8) !i32 {
    var lines = std.mem.splitScalar(u8, input, '\n');

    var tot: i32 = 0;
    while (lines.next()) |line| {
        if (line.len < 2) {
            continue;
        }

        const halfLen: usize = @divFloor(line.len, @as(i32, 2));
        const intersected_char = try getFirstIntersectedChar(line[0..halfLen], line[halfLen..]);
        if (intersected_char == null) {
            return ComputePrioritiesSumError.NoRepeatedItemFoundInLine;
        }
        tot += try getPriority(if (intersected_char) |c| c else unreachable);
    }

    return tot;
}

const GetPriorityError = error{InvalidChar};

fn getPriority(c: u8) GetPriorityError!i32 {
    if (c >= 'a' and c <= 'z') {
        return c - 'a' + 1;
    }

    if (c >= 'A' and c <= 'Z') {
        return c - 'A' + 27;
    }

    return GetPriorityError.InvalidChar;
}

test "priority fn" {
    try std.testing.expectEqual(@as(i32, 16), try getPriority('p'));
    try std.testing.expectEqual(@as(i32, 38), try getPriority('L'));
    try std.testing.expectEqual(@as(i32, 42), try getPriority('P'));
    try std.testing.expectEqual(@as(i32, 22), try getPriority('v'));
    try std.testing.expectEqual(@as(i32, 20), try getPriority('t'));
    try std.testing.expectEqual(@as(i32, 19), try getPriority('s'));
}

test "case 0" {
    const input =
        \\vJrwpWtwJgWrhcsFMMfFFhFp
        \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        \\PmmdzqPrVvPwwTWBwg
        \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        \\ttgJtRGJQctTZtZT
        \\CrZsJsPPZsGzwwsLwLmpwMDw
        \\
    ;

    try std.testing.expectEqual(
        @as(i32, 157),
        try computePrioritiesSum(input),
    );
}
