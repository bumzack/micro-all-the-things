"use strict";
Object.defineProperty(exports, "__esModule", {value: true});
exports.pool = void 0;
const Pool = require('pg-pool');
var pool = new Pool({
    database: 'bumzack',
    user: 'bumzack',
    password: 'bumzack!',
    port: 5432,
    ssl: false,
    max: 50,
    idleTimeoutMillis: 1000,
    connectionTimeoutMillis: 1000,
    maxUses: 7500, // close (and replace) a connection after it has been used 7500 times (see below for discussion)
});
exports.pool = pool;
//# sourceMappingURL=index.js.map