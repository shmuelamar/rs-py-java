#!/usr/bin/env python
import rsdivider_py

if __name__ == '__main__':
    result = rsdivider_py.div_numbers(4, 2)
    assert result == 2
    print('4 / 2 = %d' % result)
