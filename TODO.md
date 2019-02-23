# The todo list for some major issues

## NEAT implementation

### Keeping a record of previous innovations
#### Current situation
Currently every new link or node gets a new innovation number.
#### Ideal situation
We keep a database of the previous innovations and check if that link or node already mutated earlier,
if so we assign the same innovation number to that mutation.
#### Problems
Checking will take O(log n) time if we store the mutations in a tree.
Because this will eventually add up we should probably clean it every 1000 innovations or so.