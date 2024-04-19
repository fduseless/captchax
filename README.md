# CaptchaX

Captcha for Python, which generates captcha image by Rust.

## Feature

- Native Package base on Rust.
- Simple, Easy to use;
- High performance.

## Install

Install captchax with pip:

```bash
pip install captchax
```

## Usage

```python
import captchax
text, image_bytes = captchax.create_image()
```

```python
help(captchax.create_image)
```

> len: num of character,
>
> difficulty: range [1,10]
>
> line: draw bezier curve or ellipse
>
> noise: whether add gaussian noise
>
> format: png | jpg | jpeg | webp


## Thanks

[rucaptcha](https://github.com/huacnlee/rucaptcha)
