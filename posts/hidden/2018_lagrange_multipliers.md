# 拉格朗日乘子法

拉格朗日乘子法（Lagrange Multipliers），可以将有 $d$ 个变量与 $k$ 个约束条件的最优化问题转化为具有 $d+k$ 个变量的无约束优化问题。

### 一、简单情况

$$
\begin{aligned}
& \min_{\boldsymbol x} && f(\boldsymbol x) \\
& \textrm{s.t.} && g(\boldsymbol x) = 0
\end{aligned}
$$

从几何角度看，该问题的目标是在方程 $g(\boldsymbol x) = 0$ 确定的 $d-1$ 维曲面上寻找能使目标函数 $f(\boldsymbol x)$ 最小化的点，此时：

1. 对于约束曲面上的任一点 $\boldsymbol x$，该点的梯度 $\nabla g(\boldsymbol x)$ 正交于约束平面
2. 在最优点 $\boldsymbol x^*$，目标函数在该点的梯度 $\nabla f(\boldsymbol x)$ 正交于约束平面。

上述条件即函数等值线与约束曲面相切，可通过反证法证明：若梯度 $\nabla f(\boldsymbol x^*)$ 与约束曲面不正交，则仍可在约束曲面上移动该点使函数值进一步下降。

由此可知，在最优点 $\boldsymbol x^*$，梯度 $\nabla g(\boldsymbol x)$ 和 $\nabla f(\boldsymbol x)$ 的方向相同或相反，即存在 $\lambda \neq 0$ 使得：

$$
\nabla f(\boldsymbol x^*) + \lambda \nabla g(\boldsymbol x^*) = 0
$$

$\lambda$ 称为拉格朗日乘子。定义拉格朗日函数：

$$
L(\boldsymbol x, \lambda) = f(\boldsymbol x) + \lambda g(\boldsymbol x)
$$

当函数 $L(\boldsymbol x, \lambda)$ 的 [Jacobian 矩阵](https://en.wikipedia.org/wiki/Jacobian_matrix_and_determinant) $J_L = \boldsymbol 0$ 时，约束条件和梯度条件同时满足。于是，原约束优化问题可转化为对拉格朗日函数 $L(\boldsymbol x, \lambda)$ 的无约束优化问题。

举个例子🌰，求椭圆 $\frac {x^2}{4} + \frac {y^2}{3} = 1$ 上到点 $(1, 1)$ 的最短距离的平方，即：

$$
\begin{aligned}
& \min_{x, y} && f(x, y) = {(x - 1)^2+(y - 1)^2} \\
& \textrm{s.t.} && g(x, y) = \frac {x^2}{4} + \frac {y^2}{3} - 1 = 0
\end{aligned}
$$

解：定义拉格朗日函数：

$$
L(x, y, \lambda) = f(x, y) + \lambda g(x, y)
$$

对应的 Jacobian 矩阵为：

$$
J_L = \left [ 2(x - 1)+\frac {\lambda} {2}x, 2(y - 1)+\frac {2\lambda }{3}y, \frac {x^2}{4} + \frac {y^2}{3} - 1 \right ] = \boldsymbol 0
$$

可得：

$$
\lambda = 4(\frac 1 x - 1) = 3(\frac 1 y - 1)
$$

可得：

$$
y = \frac {3x} {4 - x}
$$

可得：

$$
\frac {x^2} {4} + \frac {3x^2} {(4 - x)^2} = 1
$$

可得：

$$
x^4 - 8x^3 + 24 x^2 + 32x - 64 = 0
$$

根据[四次方程求根公式](https://zh.wikipedia.org/wiki/%E5%9B%9B%E6%AC%A1%E6%96%B9%E7%A8%8B#%E6%B1%82%E6%A0%B9%E5%85%AC%E5%BC%8F)，可解得 $x_1 \approx 1.24, x_2 \approx -1.71$。由 $\lambda > 0$ 可得 $x > 0, y > 0$，最终可求得 $x \approx 1.24, y \approx 1.36, \min f(x, y) \approx 0.19$。

> 上面这道题目是笔者高中的时候想到却不会解决的问题。后来大一的《工科数学分析》课程中有拉格朗日乘子法，却没有好好学习，实在惭愧。

[未完待续]

### 参考文献

1. 周志华. "机器学习." 清华大学出版社，北京.
2. ["Lagrange multiplier." 维基百科.](https://en.wikipedia.org/wiki/Lagrange_multiplier)

