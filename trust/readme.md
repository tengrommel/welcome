# tcp

    sudo setcap cap_net_admin=eip /home/teng/CLionProjects/welcome/trust/target/debug/trust
# 添加ip地址

     sudo ip addr add 10.0.0.1/24 dev tun0
# 启动网卡
    
     sudo ip link set up dev tun0
# 加入循环

    修改run.sh
# ping 10.0.0.2
    
    ping -I tun0 10.0.1.2