# Number Search

Your program must take an input file, create an output file, and return the path to the newly created output file. Ensure the path to the input file is absolute and easily modifiable. The input and output files must use Linux line endings (the line feed character: `\n`).

For each line of the input file:
* Identify the first number in the line
* Identify the last number in the line
* Combine these two numbers to form a two-digit number (if there is only a single number in a given line - use that number twice)
* Write this two-digit number to the new file

Example:

Input file:
```
g5t86aah32
h7lmq9p
3xk
y1z4w8b
n8v2a
```

Output file:
```
52
79
33
18
82
```

Program output:
`/home/user/output.txt`

## Constraints:
* There will be at least 1 line in each file
* There is no set limit to the number of lines in a file _(hint: expect very large files)_
* Each line will contain at least 1 number
* The maximum line length is 100
* Alphanumeric only - no spaces or special characters

## Scoring
* Solutions must pass all test cases (tests used for scoring will not be the same as the provided example)
* The program with the fastest execution time that passes all test cases is the winner

## Submission
* All solutions must be submitted by September 1st at 23:59:59
