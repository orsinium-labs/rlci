
# Increment the given number
inc = \n \a \b a (n a b)

# Add two numbers together
add = \a \b a inc b

# Multiply two numbers
mul = \a \b \c a (b c)

# Get `a` in the power of `b`
pow = \a \b b a

# Decrement the given number.
dec = \n \f \x n (\g \h h (g f)) (\_ x) (\x x)

# Substract `b` from `a`
sub = \a \b b dec a

# Substract the bigger number from the smaller number
diff = \a \b add (sub a b) (sub b a)

# Natural numbers, church numerals
0 = \a \b b
1 = \a a
2 = \a \b a (a b)
3 = \a \b a (a (a b))
4 = inc 3
5 = inc 4
6 = inc 5
7 = inc 6
8 = inc 7
9 = inc 8
10 = inc 9

is_zero = \n n (\_ \a \b b) (\a \b a)
gte = \a \b is_zero (sub b a)
lte = \a \b is_zero (sub a b)
gt = \a \b is_zero (sub (inc b) a)
lt = \a \b is_zero (sub (inc a) b)
eq = \a \b and (gte a b) (lte a b)

# Get the smaller of the two numbers
min = \a \b lte a b a b

# Get the bigger of the two numbers
max = \a \b gte a b a b
