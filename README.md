### HTTP-Server

**支持 / 、/home、 404三种url，根据不同的url调用不同module中的response函数，server命令行和浏览器都会有调用module提示.**

#### run

```shell
cd host

cargo run
```

#### module_200

浏览器访问`localhost:7878`或者`127.0.0.1:7878`，调用`module_200`中的`response`函数，返回`hello.html`.

#### module_home

浏览器访问`localhost:7878/home`或者`127.0.0.1:7878/home`，调用`module_home`中的`response`函数，返回`home.html`.

#### module_404

浏览器访问`localhost:7878/hahaha`或者`127.0.0.1:7878/xyz`等其他任意不存在的目录，调用`module_home`中的`response`函数，返回`404.html`.

