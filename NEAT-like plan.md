# A NEAT-like approach to evolving neural networks

## Biological mutations
There are several kinds of mutation in the real world, it would be great if our approach allowed for all of these.

Small scale:

1. Deletion
2. Duplication
3. Inversion

Bigger scale:

4. Insertion
5. Translocation

## Biological recombination
TODO

## Possible changes to NEAT
To stop a constant growth of the genome:
- have a mutation delete disabled genes
- have an energy penalty for the size of the genome
- have an energy penalty for the size of the enabled genome

## Speciation
NEAT has a neat little formula for this (pun intended): $$\delta = c_1\frac{E}{N} + c_2\frac{D}{N} + c_3 W$$
where E is the number of excess genes, D is the number of disjoint genes, N is the number of genes and W is the average weight difference of matching genes.
