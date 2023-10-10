const std = @import("std");

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var inputFile = try std.fs.cwd().openFile("src/d02/input.txt", .{ .mode = .read_only });
    defer inputFile.close();

    const file_size = (try inputFile.stat()).size;
    const allocator = std.heap.page_allocator;

    var buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);
    try inputFile.reader().readNoEof(buffer);

    const result = try compute_tota_score(buffer);

    try stdout.print("{d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}

const MatchOutcome = enum {
    win,
    lose,
    tie,
};

const Move = enum {
    rock,
    paper,
    scissors,
};

fn getMoveScore(move: Move) i32 {
    return switch (move) {
        Move.rock => 1,
        Move.paper => 2,
        Move.scissors => 3,
    };
}

fn getOutcomeScore(outcome: MatchOutcome) i32 {
    return switch (outcome) {
        MatchOutcome.lose => 0,
        MatchOutcome.tie => 3,
        MatchOutcome.win => 6,
    };
}

const ParseMoveError = error{UnknownMove};

fn parseMove(c: u8) ParseMoveError!Move {
    return switch (c) {
        'A' => Move.rock,
        'B' => Move.paper,
        'C' => Move.scissors,
        else => ParseMoveError.UnknownMove,
    };
}

const ParseOutcomeError = error{UnknownOutcome};
fn parseOutcome(c: u8) ParseOutcomeError!MatchOutcome {
    return switch (c) {
        'X' => MatchOutcome.lose,
        'Y' => MatchOutcome.tie,
        'Z' => MatchOutcome.win,
        else => ParseOutcomeError.UnknownOutcome,
    };
}

fn computeMoveForDesiredOutcome(desiredOutcome: MatchOutcome, his: Move) Move {
    if (desiredOutcome == MatchOutcome.tie) {
        return his;
    }

    return switch (his) {
        Move.rock => if (desiredOutcome == MatchOutcome.lose) Move.scissors else Move.paper,
        Move.paper => if (desiredOutcome == MatchOutcome.lose) Move.rock else Move.scissors,
        Move.scissors => if (desiredOutcome == MatchOutcome.lose) Move.paper else Move.rock,
    };
}

fn compute_tota_score(input: []const u8) !i32 {
    var lines = std.mem.splitScalar(u8, input, '\n');
    var tot: i32 = 0;
    while (lines.next()) |line| {
        if (line.len < 3) {
            continue;
        }

        const his = try parseMove(line[0]);
        const desiredOutcome = try parseOutcome(line[2]);

        const mine = computeMoveForDesiredOutcome(desiredOutcome, his);
        tot += getMoveScore(mine) + getOutcomeScore(desiredOutcome);
    }

    return tot;
}

test "case 0" {
    const input =
        \\A Y
        \\B X
        \\C Z
        \\
    ;

    try std.testing.expectEqual(
        @as(i32, 12),
        try compute_tota_score(input),
    );
}
