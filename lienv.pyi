from typing import Type, TypeVar

T = TypeVar('T')


def get(var_type: Type[T], var_name: str) -> T:
    """
    Parse the content of an environment variable into an object of the specified type.

    Args:
        var_name (str): The name of the environment variable to be parsed.
        var_type (Type[T]): The type into which the variable content should be parsed.

    Returns:
        T: An object of the specified type, obtained by parsing the content of the environment variable.

    Raises:
        ValueError: If the content cannot be parsed into the specified type.
        KeyError: If the name of the source environment variable is invalid.
        SyntaxError: For Lists and Dicts if the content cannot be parsed due to invalid syntax.
    """
    ...
