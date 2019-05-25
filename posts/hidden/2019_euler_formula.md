# Euler's Formula

### Euler's Formula

$$
e^{ix} = \cos x + i \sin x
$$

### Euler's Identity

$$
e^{i \pi} + 1 = 0
$$

### Power Series

$$
\begin{aligned}
e^{ix} &= 1 + ix + \frac {(ix)^2} {2!} + \frac {(ix)^3} {3!} + \frac {(ix)^4} {4!} + ...\\
&= 1 + ix - \frac {x^2} {2!} - \frac {ix^3} {3!} + \frac {x^4} {4!} + ...\\
&= (1 - \frac {x^2} {2!} + \frac {x^4} {4!} - ...)
+ i(x - \frac {x^3} {3!} + \frac {x^5} {5!} - ...)\\
&= \cos x + i \sin x
\end{aligned}
$$

### Geometric Interpretation

$$
\begin{aligned}
e^{ix} &= \lim_{n \to \infty} (1 + \frac {ix} {n})^n\\
&=\lim_{n \to \infty} \underbrace{(1+\frac {x}{n}i) \cdots (1+\frac {x}{n}i)}_{n}\\
\end{aligned}
$$

### Trigonometric Addition Formulas

$$
\begin{aligned}
e^{i(\alpha + \beta)} &= \cos (\alpha + \beta) + i \sin (\alpha + \beta)\\
&= e^{i \alpha}e^{i \beta}\\
&= (\cos \alpha + i \sin \alpha) \times (\cos \beta + i \sin \beta)\\
&= (\cos \alpha \cos \beta - \sin \alpha \sin \beta) + i(\sin \alpha \cos \beta + \cos \alpha \sin \beta)
\end{aligned}
$$

