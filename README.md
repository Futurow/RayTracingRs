# RayTracingRs
使用rust实现的软光线追踪
## 使用方法
```
//编译执行版本
cargo build --release 

//执行程序，结果为image.ppm文件
./target/release/ray_tracing_rs.exe > image.ppm 
```
打开image.ppm文件可使用vscode插件或专用程序
### 效果展示
![示例图片](png/imageFinal.png)
### 参考文档
[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

[Ray Tracing in One Weekend V3.0中文翻译](https://zhuanlan.zhihu.com/p/128582904)

[Rust Ray Tracing in One Weekend - Rust一周末光线追踪](https://zhuanlan.zhihu.com/p/659982592)

[Rust语言圣经(Rust Course)](https://course.rs/about-book.html)