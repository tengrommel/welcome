# 运行docker-compose

    docker-compose up -d
    
# 设置docker-compose权限 

    chmod +x /usr/local/bin/docker-compose

# 登录psql

    psql -h 127.0.0.1 -p 15432 -U actix
    
密码为actix

# 创建数据库表和插入数据

    psql -h 127.0.0.1 -p 15432 -U actix < database.sql
    
# ab测试
    
    ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:1313/todos
    
# 安装diesel
    sudo apt-get install libpq-dev
    cargo install diesel_cli --no-default-features --features postgres
    diesel setup
    diesel migration run --database-url "postgres://actix:actix@localhost:15432/actix"
    
