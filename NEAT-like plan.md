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

## NEAT recombination
There are multiple methods of recombination, here is an example:

for an excess gene:
- only inherit if it is from the more fit parent.

for a disjoint gene:
- only inherit if it is from the more fit parent.

for a gene occuring in both:
- take the average weight

## NEAT mutation
1. a new link
2. a link converted into a node
3. enabling or disabling connections
4. weight shifting, multiplying the weight with a random number between 0.8 and 1.2
5. replace a weight with a new completely random weight

### A link converted into a node
Assume we have a link from 2 to 6 with weight a. After this mutation we have a link from 2 to the new node 7 with weight 1 and a link from 7 to 6 with weight a.

## Possible changes to NEAT
To stop a constant growth of the genome:
- have a mutation delete disabled genes
- have an energy penalty for the size of the genome
- have an energy penalty for the size of the enabled genome

## Speciation
NEAT has a neat little formula for this (pun intended): $$\delta = c_1\frac{E}{N} + c_2\frac{D}{N} + c_3 W$$
where E is the number of excess genes, D is the number of disjoint genes, N is the number of genes and W is the average weight difference of matching genes.
