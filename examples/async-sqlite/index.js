"use strict";

const { promisify } = require("util");

const { databaseNew, databaseClose, databaseInsert, databaseGetById } = require("./index.node");

// Convert the DB methods from using callbacks to returning promises
const databaseInsertAsync = promisify(databaseInsert);
const databaseGetByIdAsync = promisify(databaseGetById);

// Wrapper class for the boxed `Database` for idiomatic JavaScript usage
class Database {
    constructor() {
        this.db = databaseNew();
    }

    // Wrap each method with a delegate to `this.db`
    // This could be node in several other ways, for example binding assignment
    // in the constructor
    insert(name) {
        return databaseInsertAsync.call(this.db, name);
    }

    byId(id) {
        return databaseGetByIdAsync.call(this.db, id);
    }

    close() {
        databaseClose.call(this.db);
    }
}

module.exports = Database;
