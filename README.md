# text_erase
擦除文字

## 1 需求描述
题目和答案写在了一起，需要快速擦出答案，打印出一份题目。
其中题目是黑色，答案是红色。

## 2 需求分析
将原始文件扫描成jpeg文件，擦出图片文件中的“红色”，形成新的文件，打印出来。
其中红色是一个区间，擦少了，会有描边存在，擦多了，会导致其它部分细节被镂空

## 3 方案简述
分析图片的所有像素，分析各像素的RGB值，找出“红色”，将其替换成“背景色”
分析方法：将每个像素的RGB读入ck，通过统计分析，画出每种RGB对应的值的统计，三种颜色叠加，从图像上看，“红色”对应哪个区域

## 4 调研内容
- [X] 无损的逐个像素复制图片
- [X] image的使用，一个像素的信息存储方式
- [X] 处理颜色的crate
- [X] plotters调研，替换ck显示3D图像
## 5 方案设计
使用image读取图片，将图片按照像素写入ck
ck的数据结构：
```
struct Point{
    img_x: u32,
    img_y: u32,
    pixel_r: u8,
    pixel_g: u8,
    pixel_b: u8,
}
```
猜想“红色”符合的特征：
rgb三个值，绘制到同一坐标系中，逐个找较大波动的，用三个区间，框出“红色”