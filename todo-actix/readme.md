# 运行docker-compose

    docker-compose up -d
    
# 设置docker-compose权限 

    chmod +x /usr/local/bin/docker-compose

# 登录psql

    psql -h 127.0.0.1 -p 15432 -U actix
    
密码为actix

# 创建数据库表和插入数据

    psql -h 127.0.0.1 -p 15432 -U actix < database.sql