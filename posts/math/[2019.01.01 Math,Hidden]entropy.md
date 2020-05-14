# Entropy

### Information

$$
I(x) = - \log p(x)
$$

### Entropy

$$
H(X) = \mathbb E_X [I(x)] = -\sum_{x \in \mathbb X}p(x)\log p(x)
$$

### Cross Entropy

$$
H(p, q) = \mathbb {E}_p[-\log q] = -\sum_{x \in \mathbb X}p(x)\log q(x)
$$

### Kullback–Leibler Divergence

$$
D_{KL}(p||q) = \sum_{x \in \mathbb X} {p(x) \log \frac {p(x)}{q(x)}} = H(p, q) - H(p)
$$

