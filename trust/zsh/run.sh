cargo b
sudo setcap cap_net_admin=eip /home/teng/CLionProjects/welcome/trust/target/debug/trust
/home/teng/CLionProjects/welcome/trust/target/debug/trust &
pid=$!
sudo ip addr add 10.0.0.1/24 dev tun0 &
sudo ip link set up dev tun0
wait $pid
