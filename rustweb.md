# class

## Step01 编写TCP Server和Client



std::net模块

标准库的std::net模块，提供网络的基本功能

支持TCP和UDP通信

TcpListener和TcpStream



![image-20240329162049956](images/image-20240329162049956.png)

Web Server

Server 

​	监听进来的TCP字节流

Router

​	接收HTTP请求，并决定调用哪个Handler

Handler

​	处理http请求，构建http响应

HTTP Libary

解释字节流，把它转化为HTTP请求

把HTTP响应转化为字节流



## Steop02

![image-20240331194734755](images/image-20240331194734755.png)

Actix的并发（concurrency）

Actix支持两类并发：

1.异步I/O:给定 的OS原生线程在等待I/O时执行 其他任务（例如：侦听网络连接）

2.多线程并行：默认情况下启动OS原生线程的数量与系统逻辑CPU数量相同



