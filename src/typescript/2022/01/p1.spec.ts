import * as fs from "fs";
import * as path from "path";
import { p1 } from "./p1";

describe('2022 01 p1', function() {
    test('test p1()', function() {
        const contents = fs.readFileSync(path.join(__dirname, 'p1.input')).toString();
        expect(p1(contents)).toBe(69281);
    });
});
