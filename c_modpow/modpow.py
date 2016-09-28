#!/usr/bin/env python

def mod_of_power(value, power, modulus):
    x = 1
    for i in range(1, power+1):
        x = (x * value) % modulus
    return x

for i in range(1, 10):
    power = i * 10000000
    value = mod_of_power(324322, power, 3724)
    print("324322^(" + str(power) + ") mod 3724 = " + str(value));
