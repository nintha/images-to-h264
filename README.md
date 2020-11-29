# Images To H264
把一些连续的图像转换成H264裸流，项目依赖libx264.

`crate_x264` 基于库[quadrupleslap/x264](https://github.com/quadrupleslap/x264) 上进行了一些修改，添加了一些libx264的调用参数和支持yuv420p格式的输入。

## Usage
```
USAGE:
    images-to-h264 [OPTIONS] <input>

ARGS:
    <input>    image file template name, '%d' is variable value, eg: image%d.png

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --fps <fps>                [default: 30]
    -h, --height <height>          [default: 360]
        --output <output>          output filename [default: out.h264]
        --start-num <start-num>    input %d start num [default: 0]
    -w, --width <width>            [default: 640]
```

## Example
从文件`frames/frame0.png`、`frames/frame1.png`……依次读取，当尝试读取的文件不存在时，就会停下。输出文件的高是360px，宽是640px，fps是30帧/s，输出文件名是out.h264。
```shell
./images-to-h264 "frames/frame%d.png" -h=360 -w=640 --fps=30 --start-num=0 --output=out.h264
```
## Reference
- 在浏览器中播放H264流
https://github.com/samirkumardas/jmuxer
- ubuntu下安装x264库
https://blog.csdn.net/tuolaji8/article/details/51277767

