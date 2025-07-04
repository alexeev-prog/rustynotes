**Метод unwrap в языке программирования Rust** — это способ **извлечь значение из типа Option или Result**. [rust.nizeclub.ru](https://rust.nizeclub.ru/_11.html)[medium.com](https://medium.com/@giorgio.martinez1926/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100-8836a98a23d6)[bytegoblin.io](https://bytegoblin.io/blog/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100.mdx)

**Option** — перечисление, которое используется, когда значение может быть либо «чем-то» (Some), либо «ничем» (None). **Result** — перечисление, которое используется для операций, которые могут завершиться успехом (Ok) или неудачей (Err). [rust.nizeclub.ru](https://rust.nizeclub.ru/_11.html)[medium.com](https://medium.com/@giorgio.martinez1926/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100-8836a98a23d6)[bytegoblin.io](https://bytegoblin.io/blog/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100.mdx)

### Принцип работы

- **Если значение — Some или Ok**, метод возвращает содержащееся значение. [medium.com](https://medium.com/@giorgio.martinez1926/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100-8836a98a23d6)[bytegoblin.io](https://bytegoblin.io/blog/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100.mdx)
- **Если значение — None или Err**, метод вызывает **панику** (прерывание программы). [rust.nizeclub.ru](https://rust.nizeclub.ru/_11.html)[medium.com](https://medium.com/@giorgio.martinez1926/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100-8836a98a23d6)[bytegoblin.io](https://bytegoblin.io/blog/unwrapping-values-with-rusts-unwrap-and-expect-tutorial-22-100.mdx)

Простыми словами, unwrap — «дай мне значение или паника». [rust.nizeclub.ru](https://rust.nizeclub.ru/_11.html)

**Когда использовать unwrap**:

- **Для быстрого прототипирования** — подходит для быстрой разработки, когда случаи ошибок обрабатываются elsewhere.
- **Когда известно, что значение существует** (например, результат из логики, которая не может потерпеть неудачу).

### Ошибки

**Использовать unwrap в продакшн-коде Rust нежелательно**. Некоторые проблемы:

- **Маскировка ошибок** — если значение Result окажется вариантом Err, то результатом вызова unwrap() будет паника, которая может закрасить всю программу. Вместо этого следует явно обработать ошибку.
- **Скрытые баги** — если значением Result является вариант Err, но его не отрабатывают, можно не понять, что что-то пошло не так.
- **Непредсказуемое поведение** — если использовать unwrap() в многопоточной среде, паника может привести к сбою или непредсказуемому поведению других потоков.