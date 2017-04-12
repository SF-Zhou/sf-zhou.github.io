# 机器学习基础知识回顾

最近各大公司实习面试，机器学习也是热门的考察对象故而将遇到的问题统一的回顾一次。

## Linear Regression & Logistic Regression

### Gradient Descent

$$
w = w - \alpha \frac {1}{m} \sum_{i}{L(y_i, f(x_i, w))'}
$$

### Target

$$
w^*=\arg \underset {w}{\min} \sum_{i}{L(y_i, f(x_i, w))+\lambda \Omega(w)}
$$

### Norm

Formally the $l_p$-norm of $x$ is defined as:
$$
\left \| x \right \|_p = \sqrt[p]{\sum_{i} \left |x_i \right |^p} \text{ where } p\in \mathbb{R}
$$

### References

1. [l0-Norm, l1-Norm, l2-Norm, … , l-infinity Norm](https://rorasa.wordpress.com/2012/05/13/l0-norm-l1-norm-l2-norm-l-infinity-norm/)