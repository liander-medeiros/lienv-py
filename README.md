## LiEnv: A simple yet useful library
<a href="https://github.com/liander-medeiros/lienv-py/actions">
    <img src="https://github.com/liander-medeiros/lienv-py/workflows/CI/badge.svg" alt="CI">
</a>
<a href="https://pypi.org/project/lienv/">
    <img src="https://img.shields.io/pypi/v/lienv.svg" alt="PyPi Latest Release"/>
</a>

LiEnv is a tool to easily get typed values out of strings stored in environment variables.

- Simplifies the sometimes tedious task of parsing strings to typed variables
- Provides a way to do it uniformly througout the project
- Aims to prevent unwanted behaviour caused by typos, errors or malicious content in environment variables

### Currently supported Python types:
- Integers and Floats
- Dicts
- Lists
- Tuples
- Booleans

## Example

```python
>>> import lienv
>>> integer = lienv.get(int, "SOME_INTEGER") # SOME_INTEGER="42"
>>> decimal = lienv.get(float, "SOME_FLOAT") # SOME_FLOAT="2.4"
>>> dictionary = lienv.get(dict, "SOME_DICT") # SOME_DICT="{'lienv': 42}"

>>> print(f"{integer} => {type(integer)}")
42 => <class 'int'>

>>> print(f"{decimal} => {type(decimal)}")
2.4 => <class 'float'>

>>> print(f"{dictionary} => {type(dictionary)}")
{'lienv': 42} => <class 'dict'>
```

## Setup

Install the latest version with:

```sh
pip install lienv
```

## TODO

- [ ] Parse types without directly using python's eval
- [ ] Implement unit tests in Rust
- [ ] Figure out how to test the Python module
- [ ] Change default integers and floats to 32 bit based on architecture
- [ ] Add wrappers for unsigned integer parsing
- [ ] Add wrappers for 8 and 16 bit numbers


## Contributing
I'll gladly accept feedback and contributions. 

Although this project's purpose was to solve problems that at first seemed too simple, the practicality it provided to me was motivation enough to turn it into a library so maybe more people could benefit from it.

I am yet to be proficient in Rust, so pardon my silly mistakes.

<div align="center">
    <img src="https://media.npr.org/assets/img/2023/05/26/honest-work-meme-cb0f0fb2227fb84b77b3c9a851ac09b095ab74d8-s1100-c50.jpg" width="300px"/>
</div>
