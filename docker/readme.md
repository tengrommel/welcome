# docker 的架构和底层技术
- Docker提供了一个开发，打包，运行app的平台
- 把app和底层infrastructure隔离开来

# Docker Engine
- 后台进程(dockerd)
- Rest API Server
- CLI 接口

# 底层的技术支持
- Namespaces: 做隔离pid，net，ipc，mnt，uts
- Control groups: 做隔离限制
- Union file systems: Container和image的分层

# 什么是Image
- 文件和meta data的集合(root filesystem)
- 分层的，并且每一层都可以添加改变删除文件，成为一个新的image
- 不同的image可以共享相同的layer
- Image本身是read-only的