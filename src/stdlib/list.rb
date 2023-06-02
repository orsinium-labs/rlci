
# A pair.
# Uses closures to hold two values.
cons = \a \b \c c a b

# Get the first value from a pair.
car = \p p true

# Get the second value from a pair.
cdr = \p p false

############

# An empty list.
# A list consists of nested pairs, and each item is a pair itself
# where the first item indicates if that's the end of the list
# and the second item (if that's not the end) is the actual element value.
empty_list = cons true true

# Add at the beginning of the `xs` list the `x` element.
prepend = \xs \x cons false (cons x xs)

# Check if the given list is empty.
is_empty = car

# Get the first element from the list.
head = \xs car (cdr xs)

# Remove the first element from the list.
tail = \xs cdr (cdr xs)
