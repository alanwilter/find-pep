# find-pep

A CLI tool for find peptides in UniProt fasta files

## Usage

```bash
Usage: find-pep <input_fasta.fasta> <input_list.txt>
```

## Example

```bash
find-pep input_fasta.fasta input_list.txt | tee output.txt
DTLMISR:F7HL06:1
DTLMISR:A3RFZ7-3:1
DTLMISR:F7F0D6:1
ISRNQVSLTCLVK:F7HL06:1
ISRNQVSLTCLVK:B0FPE9:2
ISRNQVSLTCLVK:F7F0D6:1
```
