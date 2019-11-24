# Collatz conjecture

Form a sequence by beginning with an integer `n` and
repeatedly applying the function `f(n)`, taking the result of each step as the input of the next.

```
f(n) := n / 2       if (n % 2 == 0)
        3n + 1      otherwise
```

The collatz conjecture states that for any natural number `n` this
sequence will reach the loop `1, 4, 2, 1`.

# This project

My goal is to prove that no other loop is possible, meaning that this
sequence must either diverge to infinity or reach `1`. While experimenting with different ideas was enjoyable, 
this goal has not been reached.

# Progress

A loop `L` will require that `n / 2` is used `w` times and `3n + 1` is used `q` times.
For any `n` contained in a loop the following equation must hold.

$$
Statement\ 1\\
n = \frac{3^q}{2 ^ w}n + \frac{y}{2 ^ w}
$$

`y` can be written as the following sum.

$$
Statement\ 2\\
y = \sum_{p = 1}^q 3^{q-p}2^{a_p}
$$

Where $a_p$ is the number of divisions before the `p`-th multiplication.

$$
Statement\ 3\\
\forall p: a_p < a_{p + 1}
$$

When looking at an odd number, the following is also true.

$$
Statement\ 4\\
a_1 = 0
$$

Statement `1` can be further simplified.

$$
Statement\ 5\\
n = \frac{3^q}{2 ^ w}n + \frac{y}{2 ^ w} \Leftrightarrow
n -  \frac{3^q}{2 ^ w}n =  \frac{y}{2 ^ w} \Leftrightarrow
\frac{2^w - 3^q}{2 ^ w}n = \frac{y}{2 ^ w} \Leftrightarrow
(2^w - 3^q)n = y \Leftrightarrow
n = \frac{y}{2^w - 3^q}
$$

As $n \neq 1 \Leftrightarrow 3n + 1 \neq 4n$ we know the following about $q$ and $w$

$$
Statement\ 6\\
\frac{3}{2}q < w < 2q
$$

Due to $n\ mod\ 2 \neq 0 \Leftrightarrow 3n + 1\ mod\ 2 = 0$ it is known that each multiplication follows a division. Due to statement `6` it is also clear that the following sequence must exist at least once in a loop.

$$
Sequence\ 7\\
mul \rightarrow div \rightarrow mul \rightarrow div \rightarrow div
$$

For any $x$ starting this sequence, statement `8` must hold.

$$
Statement\ 8\\
x\ mod\ 8 = 3 \Leftrightarrow x = \dots011_{bin}
$$

For this $x$ we also know more about $y_x$

$$
Statement\ 9\\
y_x = 3^{q-1} + 3^{q-2}2 + \sum_{p = 3}^q 3^{q-p}2^{a_p}\\
a_3 \geq 3
$$

There exists a pattern for the lowest bits of $3^n$.

$$
Pattern\ 10\\
n\ mod\ 2 = 0 \Leftrightarrow 3^n = \dots001_{bin}\\
n\ mod\ 2 \neq 0 \Leftrightarrow 3^n = \dots011_{bin}\\
$$

We now have a condition for a loop
containing a sequence $\alpha \in \{0, 1\}^*$,
where $1$ denotes a step of $3n + 1$ and $0$ a step of $n / 2$.
We only consider the lowest $|\alpha|$ bits of the binary representation.

$$
Statement\ 11\\
x(2 ^ k - 3^q) = \sum_{p = 1}^q 3^{q-p}2^{p-1}\alpha_{p}
$$

$x$ is a number at start of the sequence.
$2^k - 3^q$ is always positive,
therefore its lowest $|\alpha|$ bits are the lowest bits of negative $3^q$.
$\alpha_p$ is value of the $p$-th position in $\alpha$.
The lowest bits of $3^n$ have a repeating pattern which can be used in this equation.

In case a sequence $\omega$ with the property $\omega(x) = x$ exists, $\omega^r$ must also have this property for all $r \in \N$.

This means that statement `11` can be extended.

$$
Statement\ 12\\
x(2^{rk} - 3^{rq}) = \sum_{p = 1}^{rq} 3^{rq-p}2^{p-1}\alpha_{p}
$$

The lowest $p$ bits of $3^n$ has a repeating pattern with cycle length $2^{p-2}$.
To now prove that a sequence cannot be part of any cycle, we would have to prove that
it is not possible in a cycle with a multiple of $2^{p-2}$ multiplications.

$$
Statement\ 13\\
\forall p > 2: 3^{2^{p-2}} mod\ 2^p = 1
$$

Any partial sequence $\alpha$ with $|\alpha| \leq 126$ seems to be allowed.
This was still an enjoyable experience and I might revisit this in the future.

The most interesting thing I have learned is that the following equation can be proven using this sequence.

$$
Statement\ 14\\
\forall p, k \in \N: (2^p)^k + (2^p - 1)^k = \sum_{n=1}^k (2^p-1)^{k-n}(2^p)^{n-1}
$$