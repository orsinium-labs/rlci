
# Calculate factorial of the given natural number.
#
# The current interpreter implementation is slow for numbers and recursions,
# and factorial requires both.
fac = Y (
    \f \n is_zero n 1 (mul n (f (dec n)))
)

# Get Nth Fibonacci natural number.
fib = Y (
    \f \n lte n 2 1 (add (f (dec n)) (f (dec (dec n))))
)
