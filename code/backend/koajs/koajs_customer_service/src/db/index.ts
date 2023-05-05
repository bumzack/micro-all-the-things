const { Pool } = require('pg')
const config1 = require('../configuration')

const start = async (): Promise<typeof Pool> => {
    const host = config1.get('PGHOST')
    const user = config1.get('PGUSER')
    const port = config1.get('PGPORT')
    const password = config1.get('PGPASSWORD')
    const database = config1.get('PGDATABASE')
  const  pool = new Pool({ user, host, database, password, port })
    return pool;
}


const close = async (): Promise<void> => {
    await this.pool.end()
}


const query = async (q:string, data): Promise<void> => {
    return this.pool.query(q, data)
}

const queryOne = async (q:string, data): Promise<void> => {
    return this.pool.query(q, data).then(r => r.rows[0])
}



export { start, query, close, queryOne }
