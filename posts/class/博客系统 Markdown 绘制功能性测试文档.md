# 博客系统 Markdown 绘制功能性测试文档

## 1. 测试二级标题及三级标题

### 1.1 三级标题

三级标题的间距是否正常？

### 1.2 连续两个三级标题

### 1.3 不支持四级标题了

## 2. 文本段落

### 2.1 中文文本两段

中国的原始社会，起自大约170万年前的元谋人，止于公元前21世纪夏王朝的建立。原始社会经历了原始人群和氏族公社两个时期。氏族公社又经历了母系氏族公社和父系氏族公社两个阶段。元谋人是已知的中国境内最早的人类。北京人是原始轩辕黄帝人群时期的典型。山顶洞人已经过着氏族公社的生活。长江流域的河姆渡氏族和黄河流域的半坡氏族是母系氏族公社的繁荣时期。大汶口文化的中晚期反映了父系氏族公社的情况。

传说中，黄帝是大约4500多年前，生活在黄河流域原始部落的部落联盟首领。他提倡种植五谷，驯养牲畜，促使这个部落联盟逐步强大。他曾率领部落打败黄河上游的炎帝部落和南方的蚩尤部落。后来炎帝部落和黄帝部落结成联盟，在黄河流域长期生活、繁衍，构成了以后华夏族的主干成分。黄帝被尊奉为华夏族的祖先。中华民族被称为炎黄子孙，就是这么来的。黄帝以后，黄河流域部落联盟的杰出首领，先后有尧、舜、禹。那时候，部落联盟首领由推选产生。尧年老了，召开部落联盟会议，大家推举有才德的舜为继承人。尧死后，舜继承了尧的位置，舜年老了，也采取同样的办法把位置让给治水有功的禹。这种更替首领位置的办法，历史上叫做“禅让”。

### 2.2 英文文本两段

Delay-and-sum(DAS) algorithm is a widespread algorithm used in various typical applications, such as medical ultrasound imaging, radar signal emission and reception, and antenna directional signal formation. It is not a typical computationally demanding algorithm, but under new circumstance, for example medical imaging cloud service, its calculation speed should be increased so as to meet the cloud network communication speed requirements. The conventional cloud infrastructures using central processing units as prime computing resources are not adequate for the fast image formation process. Therefore, SuperVessel cloud accelerated with heterogeneous field-programmable gate arrays was used as the implementation platform for the parallel delay-and-sum algorithm in this design.

This design is used for accelerating DAS algorithm with CAPI acceleration core on SuperVessel platform. And the computing speed of the SuperVessel heterogeneous implementation is 22 times faster than the algorithm implementation on a central processing unit, which also includes the data transferring time. Therefore we can use this high speed implementation for medical imaging cloud service.

## 3. 引用、列表和代码

### 3.1 引用

> 活在这珍贵的人间，太阳强烈，水波温柔。——海子

> 在向你挥舞的各色手帕中
> 是谁的手突然收回
> 紧紧捂住了自己的眼睛
> 当人们四散离去，谁
> 还站在船尾
> 衣裙漫飞，如翻涌不息的云
> 江涛
> 高一声
> 低一声
> 美丽的梦留下美丽的忧伤
> 人间天上，代代相传
> 但是，心
> 真能变成石头吗
> 为眺望远天的杳鹤
> 错过无数次春江月明
> 沿着江岸
> 金光菊和女贞子的洪流
> 正煽动新的背叛
> 与其在悬崖上展览千年
> 不如在爱人肩头痛哭一晚
>
> ——舒婷，《神女峰》

### 3.2 有序列表

1. 现代诗歌
   1. 海子
   2. 舒婷
2. 古代诗歌
   1. 李白
   2. 白居易

### 3.3 无序列表

李白的诗有：

* 《早发白帝城》
* 《赠汪伦》
* 《静夜思》

### 3.4 代码

```python
class HashTable:
    valid_clock = 0

    def __init__(self, size):
        self.size = size

        self.valid = [0] * size
        self.keys = [None] * size
        self.values = [None] * size

    def get_value(self, item):
        idx = item % self.size
        while self.valid[idx] == self.valid_clock and self.keys[idx] != item:
            idx = 0 if idx + 1 == self.size else idx + 1

        return self.values[idx] if self.valid[idx] == self.valid_clock else None

    def set_value(self, item, value):
        idx = item % self.size
        while self.valid[idx] == self.valid_clock and self.keys[idx] != item:
            idx = 0 if idx + 1 == self.size else idx + 1

        self.valid[idx] = self.valid_clock
        self.keys[idx] = item
        self.values[idx] = value
```

### 3.5 公式

$$
f(x) = \frac {\sqrt {a^2+b^2}} {\alpha }
$$

$$
d = \frac {H_{pixel}f_s}{c} (j + \sqrt {\frac {W_{pixel}^2(i-k)^2}{H_{pixel}^2} + j^2})
$$

$f_s$、$c$、$W_{pixel}$和$H_{pixel}$均为常数。
