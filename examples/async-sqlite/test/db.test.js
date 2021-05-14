"use strict";

const assert = require("assert");

const Database = require("..");

describe("SQLite Database", () => {
    it("should insert and return name", async () => {
        const db = new Database();
        const expected = "Marty McFly";
        const id = await db.insert(expected);
        const actual = await db.byId(id);

        assert.strictEqual(actual, expected);

        db.close();
    });

    it("should return undefined for non-existent user", async () => {
        const db = new Database();

        assert.strictEqual(await db.byId(5), undefined);

        db.close();
    });

    it("should reject calls to a closed database", async () => {
        const db = new Database();

        db.close();

        await assert.rejects(async () => db.byId(5));
    });
});
