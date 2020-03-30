# tcp

    sudo setcap cap_net_admin=eip /home/teng/CLionProjects/welcome/trust/target/debug/trust
    
# 添加ip地址

     sudo ip addr add 10.0.0.1/24 dev tun0
     
# 启动网卡
    
     sudo ip link set up dev tun0
    
    