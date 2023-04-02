"use strict";

const { promisify } = require("util");

const { databaseNew, databaseClose, databaseInsert, databaseGetById } = require("./index.node");

// Wrapper class for the boxed `Database` for idiomatic JavaScript usage
class Database {
    constructor() {
        this.db = databaseNew();
    }

    // Wrap each method with a delegate to `this.db`
    // This could be done in several other ways, for example binding assignment
    // in the constructor
    insert(name) {
        return databaseInsert.call(this.db, name);
    }

    byId(id) {
        return databaseGetById.call(this.db, id);
    }

    close() {
        databaseClose.call(this.db);
    }
}

module.exports = Database;
