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

# Image的获取
- Build from Dockerfile
- Pull from Registry

# 容器带来的麻烦
 
- 怎么去管理这么多容器？
- 怎么能方便的横向扩展？
- 如果容器down了，怎么能自动恢复？
- 如何去更新容器而不影响业务？
- 如何去监控追踪这些容器？
- 怎么去调度容器的创建？
- 保护隐私数据？

-------------------------------
rust

# 指针
指针的分类 Reference、Raw Pointer、Raw Pointer、fn Pointer

