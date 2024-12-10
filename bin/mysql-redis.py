import hashlib
import json
import pymysql
import redis
from fastapi import FastAPI, Depends
import uvicorn

class MysqlRedis:
    def __init__(self, mysql_conn, redis_client, cache_options=None):
        self.mysql_conn = mysql_conn
        self.redis_client = redis_client
        self.cache_options = cache_options or {
            "expire": 2629746,  # Default expiration time in seconds
            "keyPrefix": "sql."
        }

    def _generate_key(self, sql, values):
        combined = sql + json.dumps(values or {})
        return self.cache_options["keyPrefix"] + hashlib.md5(combined.encode()).hexdigest()

    def query(self, sql, values=None):
        key = self._generate_key(sql, values)

        # Attempt to retrieve from Redis cache
        try:
            cached_result = self.redis_client.get(key)
            if cached_result:
                return json.loads(cached_result)
        except redis.RedisError:
            pass  # Proceed with MySQL query if Redis is unavailable

        # Query MySQL
        try:
            with self.mysql_conn.cursor() as cursor:
                print(sql, values)
                cursor.execute(sql, values or ())
                result = cursor.fetchall()

                # Attempt to cache the result in Redis
                try:
                    self.redis_client.setex(key, self.cache_options["expire"], json.dumps(result))
                except redis.RedisError:
                    pass  # Ignore caching errors

                return result
        except pymysql.MySQLError as e:
            raise Exception(f"MySQL query failed: {str(e)}")

app = FastAPI()

# グローバル接続
mysql_conn = None
redis_client = None
cache = None

@app.on_event("startup")
async def startup_event():
    global mysql_conn, redis_client, cache
    # MySQL connection
    mysql_conn = pymysql.connect(
        host="localhost",
        user="root",
        password="",
        database="employees"
    )
    # Redis connection
    redis_client = redis.StrictRedis(host="localhost", port=6379, decode_responses=True)
    # MysqlRedis インスタンスを作成
    cache = MysqlRedis(mysql_conn, redis_client)

@app.on_event("shutdown")
async def shutdown_event():
    global mysql_conn, redis_client
    if mysql_conn:
        mysql_conn.close()
    if redis_client:
        redis_client.close()

def get_cache():
    return cache

@app.get("/")
async def root():
    return {"message": "Hello World"}

@app.get("/employee/{emp_no}")
async def get_employee_by_id(emp_no: int, cache=Depends(get_cache)):
    sql = "SELECT last_name FROM employees WHERE emp_no = %s;"
    values = (emp_no,)
    result = cache.query(sql, values)
    print(result)
    return {"message": result}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000, log_level="debug")
