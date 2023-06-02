
# Identity function. Returns the given argument.
id = \a a
if = id

true = \a \b a
false = \a \b b

# True if any of `a` or `b` is true
or = \a \b a true b

# True if both `a` and `b` are true
and = \a \b a b false

# Boolean negation. True if false, false if true.
not = \a a false true

# True if both `a` and `b` are different
xor = \a \b a (b false true) (b true false)

# True if both `a` and `b` are the same
xnor = \a \b not (xor a b)
