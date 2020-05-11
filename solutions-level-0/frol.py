#!/bin/env python

iteration = 0
while True:
    position = int(input())
    print('shift_left ' * (position - 1) + 'shift_right ' * iteration)
    iteration = (iteration + 2) % 10
