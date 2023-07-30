const enum Shape {
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3
}

type OpponentShape = 'A' | 'B' | 'C';
type PlayerShape = 'X' | 'Y' | 'Z';

type Match = `${OpponentShape}${PlayerShape}`;

const opponentShapeMap: Record<OpponentShape, Shape> = {
    A: Shape.ROCK,
    B: Shape.PAPER,
    C: Shape.SCISSOR
}

const playerShapeMap: Record<PlayerShape, Shape> = {
    X: Shape.ROCK,
    Y: Shape.PAPER,
    Z: Shape.SCISSOR
}

const winningShapeMap: Record<Shape, Shape> = {
    [Shape.ROCK]: Shape.SCISSOR,
    [Shape.PAPER]: Shape.ROCK,
    [Shape.SCISSOR]: Shape.PAPER
}

const enum Outcome {
    WIN = 6,
    DRAW = 3,
    LOSE = 0
}

export function p1(input: string): number {
    const contents = input.split("\n").filter(x => x.length).map(x => x[0] + x[2]) as Match[];
    let points = 0;
    for (let i = 0; i < contents.length; i++) {
        const [op, pl] = contents[i] as unknown as [OpponentShape, PlayerShape];
        if (opponentShapeMap[op] === playerShapeMap[pl]) points += Outcome.DRAW;
        if (winningShapeMap[playerShapeMap[pl]] === opponentShapeMap[op]) points += Outcome.WIN;
        points += playerShapeMap[pl]
    }
    return points;
}

