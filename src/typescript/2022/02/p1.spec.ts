import * as fs from 'fs';
import * as path from 'path';
import { p1 } from "./p1";

describe('2022 02 p1', function () {
    test('test p1() with small input', function() {
        const input = 'A X\nB Y\nC Z\n';
        expect(p1(input)).toBe(15);
    });

    test('test p1() with file', function() {
        const input = fs.readFileSync(path.join(__dirname, 'p1.input')).toString();
        expect(p1(input)).toBe(12772);
    });
})
